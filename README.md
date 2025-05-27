# Geometry Library

[![Tests](https://github.com/Tairesh/geometry/actions/workflows/tests.yml/badge.svg)](https://github.com/Tairesh/geometry/actions/workflows/tests.yml)
[![Commit activity](https://img.shields.io/github/commit-activity/m/tairesh/geometry)](https://github.com/Tairesh/geometry/commits/main)
[![Lines of code](https://tokei.rs/b1/github/Tairesh/geometry?category=code)](https://github.com/Tairesh/geometry/tree/main)

A Rust library for 2D geometric calculations. This library provides types and functions for working with points, directions, circles, lines, and basic vector operations. It aims to be a simple and efficient tool for geometry-related tasks in Rust projects.

## Features

*   **Point (`Point`)**: Represents a 2D point with `i32` coordinates.
    *   Creation from `(i32, i32)` tuples or `vek::Vec2<f32>`.
    *   Arithmetic operations: addition, subtraction, multiplication, division with scalars, `Point`s, or `vek::Vec2`.
    *   Distance calculations (Euclidean and squared distance).
    *   Line drawing to another `Point` using Bresenham's line algorithm.
    *   Conversion to and from map/grid indices.
*   **Direction (`Direction`, `DIR8`, `DIR9`)**: Represents cardinal and intercardinal directions (e.g., North, NorthWest). Useful for grid-based movements.
*   **2D Direction (`TwoDimDirection`)**: Represents horizontal-only directions (East or West), useful in specific 2D contexts. It includes error handling for conversions from the more general `Direction` type (which can represent vertical or diagonal movements).
*   **Circles**:
    *   Generate points for a circle outline using `circles::circle(center: Point, radius: i32)`.
    *   Pre-defined constant arrays of points for circles of various radii (e.g., `CIRCLE5`, `CIRCLE7`, `CIRCLE9`, `CIRCLE11`, `CIRCLE13`).
*   **CP437 Module (`cp437`)**: Provides support related to Code Page 437 (likely for character encoding or specific character sets, e.g. for roguelikes).
*   **Type Aliases**:
    *   `Vec2` as an alias for `vek::Vec2<f32>`.
    *   `Rect` as an alias for `vek::Rect<f32, f32>`.

## Usage

Here are a few examples of how to use the `geometry` library:

```rust
use geometry::{Point, Direction, circles};

fn main() {
    // Create points
    let p1 = Point::new(10, 20);
    let p2 = Point::from((30, 25));

    // Add a direction to a point
    let p3 = p1 + Direction::NorthEast;
    println!("Point p1: {:?}, after moving NorthEast: {:?}", p1, p3); // p1: Point { x: 10, y: 20 }, after moving NorthEast: Point { x: 11, y: 19 }


    // Calculate distance
    let distance = p1.distance_to(p2);
    println!("Distance between {:?} and {:?}: {}", p1, p2, distance);

    // Generate a line between two points
    let line_points = p1.line_to(p2);
    println!("Line from {:?} to {:?}: {:?}", p1, p2, line_points);

    // Generate points for a circle
    let circle_center = Point::new(50, 50);
    let radius = 5;
    let circle_points = circles::circle(circle_center, radius);
    println!("Circle with center {:?} and radius {}: {:?}", circle_center, radius, circle_points);

    // Using a pre-defined circle
    let predefined_circle_points = circles::CIRCLE5;
    println!("Predefined CIRCLE5 points (first 3): {:?}", &predefined_circle_points[0..3]); // Example output: [(-1, -2), (0, -2), (1, -2)]
}
```

## Feature Flags

The `geometry` library uses feature flags to enable optional functionalities:

*   **`rand`**: Enables features that depend on the `rand` crate.
    *   Currently, this includes `Point::random()` for generating points with random coordinates.
    *   This feature is enabled by default.

*   **`serde`**: Enables serialization and deserialization capabilities for library types (like `Point`) using the `serde` crate.
    *   This allows you to easily integrate these types with formats like JSON, TOML, etc.
    *   This feature is enabled by default.

You can customize the included features by modifying the dependency in your `Cargo.toml`:

```toml
[dependencies.geometry]
version = "0.1.0" # Replace with the desired version
default-features = false
features = ["serde"] # Example: only enable serde
```

## Installation

To use the `geometry` library in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
geometry = "0.1.0" # Replace with the latest version if different
```

Then, run `cargo build` or `cargo run` to download and compile the dependency.

## Development

This repository includes a `Makefile` with common development commands:

*   `make build`: Compile the library in release mode.
*   `make test`: Run the unit tests.
*   `make fmt`: Format the codebase.
*   `make fmt-check`: Check if the codebase is formatted.
*   `make clippy`: Run Clippy for linting and style checks.
*   `make check`: Run `fmt-check`, `test`, and `clippy`. This is useful for a quick sanity check.
*   `make before-commit`: Runs `fmt`, `update`, `check`. Recommended before committing changes.
*   `make update`: Update dependencies.
*   `make clean`: Remove build artifacts.

Ensure you have `make` installed to use these commands.

## Contributing

Contributions are welcome! If you have improvements, bug fixes, or new features you'd like to add, please follow these steps:

1.  Fork the repository.
2.  Create a new branch for your changes (`git checkout -b feature/your-feature-name`).
3.  Make your changes and ensure they pass all checks (`make check`).
4.  Commit your changes (`git commit -am 'Add some feature'`).
5.  Push to the branch (`git push origin feature/your-feature-name`).
6.  Create a new Pull Request.

Please ensure your code is well-documented and includes tests where appropriate.

## License

This project is licensed under the [MIT License](LICENSE). <!-- Ensure a LICENSE file exists at the root -->
