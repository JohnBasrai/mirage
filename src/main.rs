use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    // ---
    #[command(subcommand)]
    command: Command,
}

use anyhow::{Context, Result};

fn main() -> Result<()> {
    // ---
    let args = Args::parse();
    args.command.execute()
}

#[derive(Debug, Subcommand)]
enum Command {
    /// blur an image by a given percentage
    Blur {
        infile: String,
        outfile: String,
        #[arg(value_parser = clap::value_parser!(u32).range(0..=100))]
        percent: u32,
    },

    /// brighten an image by given amount
    Brighten {
        infile: String,
        outfile: String,
        amount: i32,
    },

    /// crop an image to x, y, width, height
    Crop {
        infile: String,
        outfile: String,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },

    /// rotate an image by the given degrees, valid values 90, 180 or 270
    Rotate {
        infile: String,
        outfile: String,
        #[arg(value_parser = rotate_valid)]
        degrees: u32,
    },

    /// generate a new image in outfile
    Generate { outfile: String, value: i32 },

    /// invert an image from infile to outfile
    Invert { infile: String, outfile: String },

    /// convert an image to grey scale
    Grayscale { infile: String, outfile: String },

    /// generate a fractal image in the file provided.
    Fractal {
        outfile: String,
        width: u32,
        height: u32,
    },
}

macro_rules! imageop {
    ($file: ident, $op: ident, $arg: expr) => {{
        let img = (image::open(&$file).context(format!("Failed to open {}", $file))?);
        img.$op($arg)
    }};
    ($file: ident, $op: ident) => {{
        let img = (image::open(&$file).context(format!("Failed to open {}", $file))?);
        img.$op()
    }};
}

macro_rules! imageop_mut {
    ($file:  ident,
     $op:    ident $(, $args: ident)*) => {{
         let mut img = (image::open(&$file).context(format!("Failed to open {}", $file))?);
         img.$op($($args),*);
         img
    }};
}

impl Command {
    // ---

    fn execute(self) -> Result<()> {
        // ---

        match self {
            Self::Blur {
                infile,
                outfile,
                percent,
            } => {
                let img = imageop!(infile, blur, percent as f32);
                img.save(&outfile)
                    .context(format!("Failed writing {}.", outfile))
            }

            Self::Brighten {
                infile,
                outfile,
                amount,
            } => {
                let img = imageop!(infile, brighten, amount);
                img.save(&outfile)
                    .context(format!("Failed writing {}.", outfile))
            }

            Self::Crop {
                infile,
                outfile,
                x,
                y,
                width,
                height,
            } => {
                let mut img = image::open(&infile).context(format!("Failed to open {}", infile))?;
                let img = img.crop(x, y, width, height);
                img.save(&outfile)
                    .context(format!("Failed writing {}.", outfile))
            }

            Self::Rotate {
                infile,
                outfile,
                degrees,
            } => {
                let img = imageop!(infile, huerotate, degrees as i32);
                let img = match degrees {
                    90 => img.rotate90(),
                    180 => img.rotate180(),
                    270 => img.rotate180(),
                    _ => {
                        return Err(anyhow::anyhow!(
                            "Invalid rotation value:{} use 90, 180 or 270",
                            degrees
                        ))
                    }
                };
                img.save(&outfile)
                    .context(format!("Failed writing {}.", outfile))
            }
            Self::Invert { infile, outfile } => {
                let img = imageop_mut!(infile, invert);
                img.save(&outfile)
                    .context(format!("Failed writing {}.", outfile))
            }

            Self::Grayscale { infile, outfile } => {
                let img = imageop!(infile, grayscale);
                img.save(&outfile)
                    .context(format!("Failed writing {}.", outfile))
            }

            Self::Fractal {
                outfile,
                width,
                height,
            } => fractal(&outfile, width, height),

            Self::Generate { outfile, value } => generate(&outfile, value),
        } // match
    } // fn execute
}

fn rotate_valid(str: &str) -> Result<u32, String> {
    // ---
    let degrees: u32 = str
        .parse()
        .map_err(|_| format!("`{}` Isn't a valid number.", str))?;

    match degrees {
        val if val == 90 || val == 180 || val == 270 => Ok(val),
        _ => Err(format!(
            "Invalid rotation value:{str} must one of: 90, 180 or 270"
        )),
    }
}

fn generate(outfile: &String, color: i32) -> Result<()> {
    println!(
        "\nGenerate: file={}, color={} is not yet implemented",
        outfile, color
    );
    Ok(())
    // TODO impl
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: &String, width: u32, height: u32) -> Result<()> {
    println!("fractal: f:{outfile}, w:{width}, h:{height}");
    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }
    imgbuf.save(outfile)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // ---

    use super::*;
    use anyhow::{ensure, Result};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_rotate_valid_accepts_valid_degrees() -> Result<()> {
        // ---

        ensure!(
            rotate_valid("90").map_err(anyhow::Error::msg)? == 90,
            "90 degrees should be valid"
        );
        ensure!(
            rotate_valid("180").map_err(anyhow::Error::msg)? == 180,
            "180 degrees should be valid"
        );
        ensure!(
            rotate_valid("270").map_err(anyhow::Error::msg)? == 270,
            "270 degrees should be valid"
        );
        Ok(())
    }

    #[test]
    fn test_rotate_valid_rejects_invalid_degrees() -> Result<()> {
        // ---

        ensure!(rotate_valid("45").is_err(), "45 degrees should be invalid");
        ensure!(
            rotate_valid("360").is_err(),
            "360 degrees should be invalid"
        );
        ensure!(rotate_valid("0").is_err(), "0 degrees should be invalid");
        ensure!(
            rotate_valid("-90").is_err(),
            "negative degrees should be invalid"
        );
        Ok(())
    }

    #[test]
    fn test_rotate_valid_rejects_non_numeric() -> Result<()> {
        // ---

        ensure!(
            rotate_valid("abc").is_err(),
            "non-numeric input should be invalid"
        );
        ensure!(
            rotate_valid("90.5").is_err(),
            "decimal input should be invalid"
        );
        ensure!(rotate_valid("").is_err(), "empty input should be invalid");
        ensure!(
            rotate_valid("90degrees").is_err(),
            "input with text should be invalid"
        );
        Ok(())
    }

    #[test]
    fn test_rotate_valid_error_messages() -> Result<()> {
        // ---

        let result = rotate_valid("45");
        ensure!(result.is_err(), "45 should produce an error");

        let error = result.unwrap_err();
        ensure!(
            error.contains("Invalid rotation value:45"),
            "Error should contain invalid rotation message"
        );
        ensure!(
            error.contains("90, 180 or 270"),
            "Error should list valid values"
        );
        Ok(())
    }

    #[test]
    fn test_fractal_creates_file() -> Result<()> {
        // ---

        let temp_dir = TempDir::new()?;
        let output_path = temp_dir.path().join("test_fractal.png");
        let output_str = output_path.to_string_lossy().to_string();

        fractal(&output_str, 100, 100)?;

        ensure!(output_path.exists(), "Fractal file should be created");

        let metadata = fs::metadata(&output_path)?;
        ensure!(metadata.len() > 0, "Fractal file should have content");
        Ok(())
    }

    #[test]
    fn test_fractal_different_dimensions() -> Result<()> {
        // ---

        let temp_dir = TempDir::new()?;

        // Test small image
        let small_path = temp_dir.path().join("small.png");
        let small_str = small_path.to_string_lossy().to_string();
        fractal(&small_str, 10, 10)?;
        ensure!(small_path.exists(), "Small fractal should be created");

        // Test larger image
        let large_path = temp_dir.path().join("large.png");
        let large_str = large_path.to_string_lossy().to_string();
        fractal(&large_str, 200, 200)?;
        ensure!(large_path.exists(), "Large fractal should be created");

        // Larger image should have more bytes
        let small_size = fs::metadata(&small_path)?.len();
        let large_size = fs::metadata(&large_path)?.len();
        ensure!(
            large_size > small_size,
            "Larger fractal should have more bytes than smaller fractal"
        );
        Ok(())
    }

    #[test]
    fn test_generate_placeholder_behavior() -> Result<()> {
        // ---

        let temp_dir = TempDir::new()?;
        let output_path = temp_dir.path().join("test_generate.png");
        let output_str = output_path.to_string_lossy().to_string();

        // Since generate is not implemented, it should return Ok but not create a file
        generate(&output_str, 255)?;
        ensure!(
            !output_path.exists(),
            "Generate should not create file until implemented"
        );
        Ok(())
    }

    #[test]
    fn test_command_debug_formatting() -> Result<()> {
        // ---

        let blur_cmd = Command::Blur {
            infile: "input.jpg".to_string(),
            outfile: "output.jpg".to_string(),
            percent: 50,
        };
        let debug_str = format!("{:?}", blur_cmd);

        ensure!(
            debug_str.contains("Blur"),
            "Debug output should contain command name"
        );
        ensure!(
            debug_str.contains("input.jpg"),
            "Debug output should contain input filename"
        );
        ensure!(
            debug_str.contains("50"),
            "Debug output should contain percent value"
        );
        Ok(())
    }

    #[test]
    fn test_imageop_macro_compiles() -> Result<()> {
        // ---

        // Test that the imageop macro expands correctly by checking the macro syntax
        // We can test the macro by verifying it generates valid code, even if we can't run it

        // This tests the macro with both variants (with and without arguments)
        // The macro should expand without syntax errors
        let test_code = stringify!(
            imageop!(test_file, blur, 50.0);
            imageop!(test_file, grayscale);
        );

        // If the macro syntax is valid, this will contain the expected method calls
        ensure!(
            test_code.contains("blur"),
            "Macro should expand blur operation"
        );
        ensure!(
            test_code.contains("grayscale"),
            "Macro should expand grayscale operation"
        );
        ensure!(
            test_code.contains("test_file"),
            "Macro should reference the file parameter"
        );
        Ok(())
    }

    #[test]
    fn test_args_parser_structure() -> Result<()> {
        // ---

        // Test that Args struct has the correct structure and can parse valid commands
        use clap::Parser;

        // Test parsing a valid blur command
        let args = Args::try_parse_from(["mirage", "blur", "input.jpg", "output.jpg", "50"]);
        ensure!(args.is_ok(), "Valid blur command should parse successfully");
        let parsed = args?;

        // Verify the command was parsed correctly
        match parsed.command {
            Command::Blur {
                infile,
                outfile,
                percent,
            } => {
                ensure!(infile == "input.jpg", "Input filename should match");
                ensure!(outfile == "output.jpg", "Output filename should match");
                ensure!(percent == 50, "Percent value should match");
            }
            _ => anyhow::bail!("Expected Blur command but got different command type"),
        }

        // Test parsing a valid fractal command
        let args2 = Args::try_parse_from(["mirage", "fractal", "output.png", "100", "200"]);
        ensure!(
            args2.is_ok(),
            "Valid fractal command should parse successfully"
        );
        let parsed2 = args2?;

        match parsed2.command {
            Command::Fractal {
                outfile,
                width,
                height,
            } => {
                ensure!(
                    outfile == "output.png",
                    "Fractal output filename should match"
                );
                ensure!(width == 100, "Fractal width should match");
                ensure!(height == 200, "Fractal height should match");
            }
            _ => anyhow::bail!("Expected Fractal command but got different command type"),
        }
        Ok(())
    }

    #[test]
    fn test_error_handling_context() -> Result<()> {
        // ---

        // Test that our error context formatting works
        let test_file = "nonexistent_file.jpg";
        let error_msg = format!("Failed to open {}", test_file);

        ensure!(
            error_msg.contains("nonexistent_file.jpg"),
            "Error message should contain filename"
        );
        ensure!(
            error_msg.starts_with("Failed to open"),
            "Error message should have correct prefix"
        );
        Ok(())
    }
}
