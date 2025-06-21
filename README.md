# Mirage 🌟

A fast and simple command-line image processing tool written in Rust. Mirage provides essential image manipulation operations with an intuitive interface.

## Features

| Feature | Description |
|---------|-------------|
| **Blur** | Apply gaussian blur with configurable intensity (0-100%) |
| **Brighten** | Adjust image brightness with positive or negative values |
| **Crop** | Extract rectangular regions from images |
| **Rotate** | Rotate images by 90°, 180°, or 270° |
| **Invert** | Create negative images by inverting colors |
| **Grayscale** | Convert color images to grayscale |
| **Fractal** | Generate beautiful fractal images |
| **Generate** | Create solid color images *(coming soon)* |

## Installation

### From Source

```bash
git clone https://github.com/yourusername/mirage.git
cd mirage
cargo build --release
```

The binary will be available at `target/release/mirage`.

### Prerequisites

- Rust 1.70+ (2021 edition)
- Cargo package manager

## Usage

```bash
mirage <COMMAND> [OPTIONS]
```

### Examples

```bash
# Blur the image
cargo run -- blur Test_Image.PNG blurred.png 10

# Convert to grayscale  
cargo run -- grayscale Test_Image.PNG gray.png

# Generate a fractal
cargo run -- fractal my_fractal.png 400 400
```

### Available Commands

| Command | Description | Arguments |
|---------|-------------|-----------|
| `blur` | Apply gaussian blur | `<infile> <outfile> <percent>` |
| `brighten` | Adjust brightness | `<infile> <outfile> <amount>` |
| `crop` | Extract image region | `<infile> <outfile> <x> <y> <width> <height>` |
| `rotate` | Rotate image | `<infile> <outfile> <degrees>` |
| `invert` | Invert colors | `<infile> <outfile>` |
| `grayscale` | Convert to grayscale | `<infile> <outfile>` |
| `fractal` | Generate fractal | `<outfile> <width> <height>` |
| `generate` | Create solid color image | `<outfile> <value>` *(coming soon)* |

### Help

```bash
# General help
mirage --help

# Command-specific help
mirage blur --help
```

## Supported Image Formats

Mirage supports common image formats including:
- JPEG/JPG
- PNG
- BMP
- TIFF
- WebP
- And more via the `image` crate

## Development

## Tips

- Generated images can be viewed in any web browser or image viewer
- Use different file extensions (.png, .jpg, .bmp) as needed for output
- The tool automatically detects input format and can convert between formats

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running in Development

```bash
cargo run -- <command> [args]
```

Example:
```bash
cargo run -- blur test.jpg blurred.jpg 25
```

## Dependencies

- [`image`](https://crates.io/crates/image) - Image processing library
- [`clap`](https://crates.io/crates/clap) - Command line argument parsing
- [`anyhow`](https://crates.io/crates/anyhow) - Error handling
- [`num-complex`](https://crates.io/crates/num-complex) - Complex number support for fractals

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

### TODO

- [ ] Implement the `generate` command
- [ ] Add more image filters (sharpen, contrast, etc.)
- [ ] Support for batch processing
- [ ] Add configuration file support
- [ ] Implement additional fractal types

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Fractal generation code adapted from [PistonDevelopers/image](https://github.com/PistonDevelopers/image)
- Built with the excellent Rust `image` crate
