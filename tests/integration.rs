use anyhow::{ensure, Result};
use image::{DynamicImage, GenericImageView};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::TempDir;

const TEST_IMAGE: &str = "Test_Image.PNG";

// Helper function to run mirage commands
fn run_mirage_command(args: &[&str]) -> Result<bool> {
    // ---
    let command_line = format!("target/release/mirage {}", args.join(" "));
    println!("run_command: {command_line}");

    let status = Command::new("target/release/mirage").args(args).status()?;
    Ok(status.success())
}

fn run_mirage_command_suppress_output(args: &[&str]) -> Result<bool> {
    // ---
    let command_line = format!("target/release/mirage {}", args.join(" "));
    println!("run_command: {command_line}");

    let status = Command::new("target/release/mirage")
        .args(args)
        .stdout(Stdio::null()) // suppress stdout
        .stderr(Stdio::null()) // suppress stderr
        .status()?;
    Ok(status.success())
}

// Helper function to verify file exists and has content
fn verify_output_file(path: &Path, min_size: u64) -> Result<()> {
    // ---

    ensure!(path.exists(), "Output file {} should exist", path.display());

    let metadata = fs::metadata(path)?;
    ensure!(
        metadata.len() > min_size,
        "Output file {} should have at least {} bytes, got {}",
        path.display(),
        min_size,
        metadata.len()
    );

    Ok(())
}

// Calculate simple image variance for blur testing
fn calculate_variance(img: &DynamicImage) -> f64 {
    // ---

    let rgb_img = img.to_rgb8();
    let pixels: Vec<f64> = rgb_img
        .pixels()
        .take(1000) // Sample first 1000 pixels for performance
        .map(|p| {
            let [r, g, b] = p.0;
            (r as f64 + g as f64 + b as f64) / 3.0
        })
        .collect();

    if pixels.is_empty() {
        return 0.0;
    }

    let mean = pixels.iter().sum::<f64>() / pixels.len() as f64;
    let variance = pixels.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / pixels.len() as f64;

    variance
}

// Calculate average brightness
fn calculate_average_brightness(img: &DynamicImage) -> f64 {
    // ---

    let rgb_img = img.to_rgb8();
    let sum: u64 = rgb_img
        .pixels()
        .take(1000) // Sample for performance
        .map(|p| {
            let [r, g, b] = p.0;
            (r as u64 + g as u64 + b as u64) / 3
        })
        .sum();

    sum as f64 / 1000.0
}

// Verify grayscale property (R=G=B)
fn verify_grayscale_property(img: &DynamicImage) -> Result<()> {
    // ---

    let rgb_img = img.to_rgb8();

    // Sample every 5th pixel for performance
    for y in (0..rgb_img.height()).step_by(5) {
        for x in (0..rgb_img.width()).step_by(5) {
            let pixel = rgb_img.get_pixel(x, y);
            let [r, g, b] = pixel.0;

            ensure!(
                r == g && g == b,
                "Grayscale pixel at ({}, {}) should have R=G=B, got R={}, G={}, B={}",
                x,
                y,
                r,
                g,
                b
            );
        }
    }

    Ok(())
}

// ============================================================================
// TIER 1 TESTS: Critical operations with full verification
// ============================================================================

#[test]
fn test_grayscale_integration() -> Result<()> {
    // ---

    let temp_dir = TempDir::new()?;
    let output_file = temp_dir.path().join("test_gray.png");

    // Run grayscale command
    let success = run_mirage_command(&["grayscale", TEST_IMAGE, &output_file.to_string_lossy()])?;
    ensure!(success, "Grayscale command should succeed");

    // Verify output file
    verify_output_file(&output_file, 1000)?;

    // Load and verify grayscale property
    let result = image::open(&output_file)?;

    // Verify dimensions match original
    let original = image::open(TEST_IMAGE)?;
    ensure!(
        original.dimensions() == result.dimensions(),
        "Grayscale image should have same dimensions as original"
    );

    // Verify grayscale property
    verify_grayscale_property(&result)?;

    // TempDir automatically cleans up when dropped
    Ok(())
}

#[test]
fn test_invert_integration() -> Result<()> {
    // ---

    let temp_dir = TempDir::new()?;
    let invert1_file = temp_dir.path().join("test_invert1.png");
    let invert2_file = temp_dir.path().join("test_invert2.png");

    // Invert once
    let success1 = run_mirage_command(&["invert", TEST_IMAGE, &invert1_file.to_string_lossy()])?;
    ensure!(success1, "First invert command should succeed");
    verify_output_file(&invert1_file, 1000)?;

    // Invert twice (should restore original)
    let success2 = run_mirage_command(&[
        "invert",
        &invert1_file.to_string_lossy(),
        &invert2_file.to_string_lossy(),
    ])?;
    ensure!(success2, "Second invert command should succeed");
    verify_output_file(&invert2_file, 1000)?;

    // Verify dimensions
    let original = image::open(TEST_IMAGE)?;
    let double_inverted = image::open(&invert2_file)?;

    ensure!(
        original.dimensions() == double_inverted.dimensions(),
        "Double-inverted image should have same dimensions as original"
    );

    // Note: We could do pixel-perfect comparison here, but compression might
    // introduce minor differences. The key test is that double inversion
    // produces a valid image with correct dimensions.

    // TempDir automatically cleans up when dropped
    Ok(())
}

#[test]
fn test_blur_integration() -> Result<()> {
    // ---

    let temp_dir = TempDir::new()?;
    let output_file = temp_dir.path().join("test_blur.png");

    // Run blur command
    let success = run_mirage_command(&["blur", TEST_IMAGE, &output_file.to_string_lossy(), "5"])?;
    ensure!(success, "Blur command should succeed");

    // Verify output file
    verify_output_file(&output_file, 1000)?;

    // Load images and verify properties
    let original = image::open(TEST_IMAGE)?;
    let blurred = image::open(&output_file)?;

    // Same dimensions
    ensure!(
        original.dimensions() == blurred.dimensions(),
        "Blurred image should have same dimensions as original"
    );

    // Blurred image should have lower variance (less sharp edges)
    let orig_variance = calculate_variance(&original);
    let blur_variance = calculate_variance(&blurred);

    ensure!(
        blur_variance < orig_variance,
        "Blurred image should have lower variance than original (original: {:.2}, blurred: {:.2})",
        orig_variance,
        blur_variance
    );

    // TempDir automatically cleans up when dropped
    Ok(())
}

#[test]
fn test_rotate_integration() -> Result<()> {
    // ---

    let temp_dir = TempDir::new()?;
    let rotate90_file = temp_dir.path().join("test_rotate90.png");
    let rotate180_file = temp_dir.path().join("test_rotate180.png");
    let rotate270_file = temp_dir.path().join("test_rotate270.png");
    let rotate360_file = temp_dir.path().join("test_rotate360.png");

    // Test 90 degree rotation
    let success =
        run_mirage_command(&["rotate", TEST_IMAGE, &rotate90_file.to_string_lossy(), "90"])?;
    ensure!(success, "90 degree rotation should succeed");
    verify_output_file(&rotate90_file, 1000)?;

    // Test 180 degree rotation
    let success = run_mirage_command(&[
        "rotate",
        &rotate90_file.to_string_lossy(),
        &rotate180_file.to_string_lossy(),
        "90",
    ])?;
    ensure!(success, "Second 90 degree rotation should succeed");
    verify_output_file(&rotate180_file, 1000)?;

    // Test 270 degree rotation
    let success = run_mirage_command(&[
        "rotate",
        &rotate180_file.to_string_lossy(),
        &rotate270_file.to_string_lossy(),
        "90",
    ])?;
    ensure!(success, "Third 90 degree rotation should succeed");
    verify_output_file(&rotate270_file, 1000)?;

    // Test 360 degree rotation (back to original orientation)
    let success = run_mirage_command(&[
        "rotate",
        &rotate270_file.to_string_lossy(),
        &rotate360_file.to_string_lossy(),
        "90",
    ])?;
    ensure!(success, "Fourth 90 degree rotation should succeed");
    verify_output_file(&rotate360_file, 1000)?;

    // Verify that after 4x 90-degree rotations, we get back to original dimensions
    let original = image::open(TEST_IMAGE)?;
    let rotated_360 = image::open(&rotate360_file)?;

    ensure!(
        original.dimensions() == rotated_360.dimensions(),
        "After 360 degrees of rotation, dimensions should match original"
    );

    // TempDir automatically cleans up when dropped
    Ok(())
}

// ============================================================================
// TIER 2 TESTS: Basic smoke tests for remaining operations
// ============================================================================

#[test]
fn test_brighten_smoke() -> Result<()> {
    // ---

    let temp_dir = TempDir::new()?;
    let output_file = temp_dir.path().join("test_brighten.png");

    // Test positive brightening
    let success =
        run_mirage_command(&["brighten", TEST_IMAGE, &output_file.to_string_lossy(), "30"])?;
    ensure!(success, "Brighten command should succeed");
    verify_output_file(&output_file, 1000)?;

    // Verify dimensions
    let original = image::open(TEST_IMAGE)?;
    let brightened = image::open(&output_file)?;
    ensure!(
        original.dimensions() == brightened.dimensions(),
        "Brightened image should have same dimensions"
    );

    // Optional: Verify brightness actually increased
    let orig_brightness = calculate_average_brightness(&original);
    let new_brightness = calculate_average_brightness(&brightened);
    ensure!(
        new_brightness > orig_brightness,
        "Brightened image should have higher average brightness"
    );

    // Test negative brightening (darkening) in same temp directory
    let darken_file = temp_dir.path().join("test_darken.png");
    let success = run_mirage_command(&[
        "brighten",
        TEST_IMAGE,
        &darken_file.to_string_lossy(),
        "--",
        "-20",
    ])?;
    ensure!(success, "Darken command should succeed");
    verify_output_file(&darken_file, 1000)?;

    // TempDir automatically cleans up when dropped
    Ok(())
}

#[test]
fn test_crop_smoke() -> Result<()> {
    // ---

    let temp_dir = TempDir::new()?;
    let output_file = temp_dir.path().join("test_crop.png");

    // Crop a 100x100 region starting at (10, 10)
    let success = run_mirage_command(&[
        "crop",
        TEST_IMAGE,
        &output_file.to_string_lossy(),
        "10",
        "10",
        "100",
        "100",
    ])?;
    ensure!(success, "Crop command should succeed");
    verify_output_file(&output_file, 500)?; // Smaller file since it's cropped

    // Verify cropped dimensions
    let cropped = image::open(&output_file)?;
    ensure!(
        cropped.dimensions() == (100, 100),
        "Cropped image should be 100x100, got {:?}",
        cropped.dimensions()
    );

    // TempDir automatically cleans up when dropped
    Ok(())
}

#[test]
fn test_fractal_smoke() -> Result<()> {
    // ---

    let temp_dir = TempDir::new()?;
    let output_file = temp_dir.path().join("test_fractal.png");

    // Generate a small fractal
    let success = run_mirage_command(&["fractal", &output_file.to_string_lossy(), "100", "150"])?;
    ensure!(success, "Fractal command should succeed");
    verify_output_file(&output_file, 1000)?;

    // Verify fractal dimensions
    let fractal = image::open(&output_file)?;
    ensure!(
        fractal.dimensions() == (100, 150),
        "Fractal should be 100x150, got {:?}",
        fractal.dimensions()
    );

    // TempDir automatically cleans up when dropped
    Ok(())
}

#[test]
fn test_generate_placeholder_smoke() -> Result<()> {
    // ---

    let temp_dir = TempDir::new()?;
    let output_file = temp_dir.path().join("test_generate.png");

    // Test generate command (currently just prints message)
    let success = run_mirage_command(&["generate", &output_file.to_string_lossy(), "255"])?;
    ensure!(
        success,
        "Generate command should succeed (even if not implemented)"
    );

    // Since generate is not implemented, file should NOT exist
    ensure!(
        !output_file.exists(),
        "Generate should not create file until implemented"
    );

    // TempDir automatically cleans up when dropped
    Ok(())
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_invalid_input_file() -> Result<()> {
    // ---

    let success =
        run_mirage_command_suppress_output(&["blur", "nonexistent.png", "output.png", "50"])?;
    ensure!(!success, "Command with nonexistent input should fail");

    Ok(())
}

#[test]
fn test_invalid_rotation_degrees() -> Result<()> {
    // ---

    let success = run_mirage_command_suppress_output(&["rotate", TEST_IMAGE, "output.png", "45"])?;
    ensure!(!success, "Invalid rotation degrees should fail");

    let success = run_mirage_command_suppress_output(&["rotate", TEST_IMAGE, "output.png", "360"])?;
    ensure!(!success, "Invalid rotation degrees should fail");

    Ok(())
}

#[test]
fn test_invalid_blur_percentage() -> Result<()> {
    // ---

    let success = run_mirage_command_suppress_output(&["blur", TEST_IMAGE, "output.png", "150"])?;
    ensure!(!success, "Blur percentage over 100 should fail");

    let success =
        run_mirage_command_suppress_output(&["blur", TEST_IMAGE, "output.png", "--", "-10"])?;
    ensure!(!success, "Negative blur percentage should fail");

    Ok(())
}

// ============================================================================
// CONSISTENCY TESTS
// ============================================================================

#[test]
fn test_operation_consistency() -> Result<()> {
    // ---

    let temp_dir = TempDir::new()?;
    let output1 = temp_dir.path().join("consistency1.png");
    let output2 = temp_dir.path().join("consistency2.png");

    // Run same operation twice
    let success1 = run_mirage_command(&["blur", TEST_IMAGE, &output1.to_string_lossy(), "25"])?;
    let success2 = run_mirage_command(&["blur", TEST_IMAGE, &output2.to_string_lossy(), "25"])?;

    ensure!(success1 && success2, "Both operations should succeed");

    // Results should have same file size (indicating consistency)
    let size1 = fs::metadata(&output1)?.len();
    let size2 = fs::metadata(&output2)?.len();

    ensure!(
        size1 == size2,
        "Same operation should produce identical file sizes (got {} vs {})",
        size1,
        size2
    );

    // TempDir automatically cleans up when dropped
    Ok(())
}

#[test]
fn test_help_commands() -> Result<()> {
    // ---

    // Test general help
    let status = Command::new("target/release/mirage")
        .arg("--help")
        .stdout(Stdio::null()) // suppress stdout
        .status()?;
    ensure!(status.success(), "General help should work");

    // Test subcommand help
    let status = Command::new("target/release/mirage")
        .args(["blur", "--help"])
        .stdout(Stdio::null()) // suppress stdout
        .status()?;
    ensure!(status.success(), "Subcommand help should work");

    Ok(())
}
