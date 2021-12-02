#[derive(Debug)]
pub enum Arg {
    Width(u32),
    Height(u32),
    Path(std::path::PathBuf),
}

impl Arg {
    pub fn parse(sargs: Vec<String>) -> Vec<Arg> {
        let mut args: Vec<Arg> = vec![];
        match sargs.get(1) {
            Some(v) => args.push(Arg::Width(v.parse::<u32>().unwrap())),
            None => {
                println!(
                    "Warning: missing argument {}: {}, using default: {}",
                    1, "width", 3840
                );
            }
        }
        match sargs.get(2) {
            Some(v) => args.push(Arg::Height(v.parse::<u32>().unwrap())),
            None => {
                args.push(Arg::Height(2160));
                println!(
                    "Warning: missing argument {}: {}, using default: {}",
                    2, "height", 2160
                );
            }
        }
        match sargs.get(3) {
            Some(v) => {
                args.push(Arg::Path(std::path::PathBuf::from(v)));
            }
            None => {
                args.push(Arg::Path(std::path::PathBuf::from("mandelbrot.png")));
                println!(
                    "Warning: missing argument {}: {}, using default: {}",
                    3, "path", "mandelbrot.png"
                );
            }
        }

        args
    }
    pub fn width(&self) -> Option<u32> {
        if let Self::Width(w) = self {
            Some(*w)
        } else {
            None
        }
    }
    pub fn height(&self) -> Option<u32> {
        if let Self::Height(h) = self {
            Some(*h)
        } else {
            None
        }
    }
    pub fn path(&self) -> Option<std::path::PathBuf> {
        if let Self::Path(p) = self {
            Some(p.to_owned())
        } else {
            None
        }
    }
}

pub fn parse_args() -> Vec<Arg> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2
        && (args[1].eq_ignore_ascii_case("-h") || args[1].eq_ignore_ascii_case("--help"))
    {
        print!("{}", help());
        std::process::exit(0);
    }
    Arg::parse(args)
}

pub fn source() -> String {
    r#"
        __kernel void mandelbrot(__global uchar *image,
                                    int width,
                                    int height)
        {
            const int id = get_global_id(0);
            const int max_iter = 800,
                max_color = 255,
                i = id / width,
                j = id % width;
                
            float x0 = (float)j / width * 3.5f - 2.5f,
                y0 = (float)i / height * 2.0f - 1.0f,
                x = 0,
                y = 0;
                
            float tx;
            int iter = 0;
            while ((x * x + y * y <= 4) && (iter < max_iter))
            {
                tx = x * x - y * y + x0;
                y = 2 * x * y + y0;
                x = tx;
                iter++;
            }
            
            int color = 1.0f + iter - log(log(sqrt(x * x + y * y))) / log(2.0f);
            color = (8 * max_color * color) / max_iter;
            color = (color > max_color) ? max_color : color;
            
            image[4 * i * width + 4 * j + 0] = color;
            image[4 * i * width + 4 * j + 1] = 0;
            image[4 * i * width + 4 * j + 2] = 0;
            image[4 * i * width + 4 * j + 3] = 255;
        }
    "#
    .to_owned()
}

pub fn csource() -> std::ffi::CString {
    std::ffi::CString::new(source()).unwrap()
}

pub fn help() -> String {
    r#"mdbrot
A simple program that generates an image of 
a Mandelbrot set using Rust OpenCL bindings.

USAGE:
    mdbrot [FLAGS] [width [height [imgpath]]]

FLAGS:
    -h, --help
            Prints help information

If no options are passed the defaults are:
    width=3840 
    height=2160
    pathname=`mandelbrot.png`
"#
    .to_owned()
}
