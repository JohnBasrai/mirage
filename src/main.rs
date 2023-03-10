use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

use anyhow::{Context, Result};

fn main() -> Result<()> {
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
    fn execute(self) -> Result<()> {
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
    let degrees: u32 = str
        .parse()
        .map_err(|_| format!("`{}` Isn't a valid number.", str))?;

    match degrees {
        val if val == 90 || val == 180 || val == 270 => Ok(val),
        _ => Err(format!("Rotation value must one of: 90, 180 or 270")),
    }
}

fn percent_in_range(str: &str) -> Result<f32, String> {
    let value: f32 = str
        .parse::<f32>()
        .map_err(|_| format!("`{}` Isn't valid percentage.", str))?;

    if value >= 0.0 && value <= 100.0 {
        Ok(value)
    } else {
        Err(format!("not in range 0 - 100"))
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

mod test {
    #[test]
    fn basics() {
        assert_eq!(0, 0);
    }
}
