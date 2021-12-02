# mdbrot

Mdbrot is a simple program that generates an image of a Mandelbrot set using Rust OpenCL bindings (ocl).

## Installation

Clone the repo.
```bash
$ git clone https://github.com/vilfa/mdbrot
$ cd mdbrot
```

You can then install the package with cargo and run it as any other executable
```bash
$ cargo install --path .
$ mdbrot [width [height [imgpath]]]
```
**OR**

just compile and run it directly.
```bash
$ cargo r -- [width [height [imgpath]]]
```

When running it directly it is recommended to use the release flag, otherwise the image writing process can take up to a few seconds compared to less than a second in release mode.
```bash
$ cargo r --release -- [width [height [imgpath]]]
```

## Usage/Examples
```
USAGE:
    mdbrot [FLAGS] [width [height [imgpath]]]

FLAGS:
    -h, --help
            Prints help information

If no options are passed the defaults are:
    width=3840 
    height=2160
    pathname=`mandelbrot.png`
```

This example generates a 4k image with red as the primary channel.
```bash
$ mdbrot 3840 2160 mandelbrot.png
```

