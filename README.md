# fractal-generator-cli-cli

A command line tool to generate and display fractals on the terminal using Delaunay triangulation and recursion.

## Features

Generates fractals using recursive subdivision
Displays the fractal on the terminal using the crossterm crate for improved visuals
Takes depth, size, and number of random points as command-line arguments
Uses Delaunay triangulation to calculate the fractal geometry

## Dependencies

delaunay_creator for Delaunay triangulation
crossterm for terminal manipulation
structopt for command-line argument parsing
Installation
Clone the repository and build the binary using Cargo:

```
cd fractal-generator-cli
cargo build --release
```

This will generate an executable binary in ./target/release/.

Usage
Run the fractal generator with the desired depth, size, and number of random points:

```
./target/release/fractal-generator-cli -d 4 -s 100 -n 100
```

Command line arguments:

-d, --depth: Depth of the fractal recursion (default: 4)
-s, --size: Size of the fractal (default: 100)
-n, --num-points: Number of random points (default: 100)
The fractal will be displayed on the terminal. Adjust the terminal size to get the best viewing experience.

License
This project is licensed under the MIT License.

Contributing
Pull requests and issues are welcome. Please feel free to contribute to the project.
