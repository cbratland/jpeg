use std::process::exit;

const OUTPUT: &str = "output.jpg";

mod basis;
mod jpeg;

#[derive(Debug)]
enum Args {
    // compress <path> <quality>
    Compress { path: String, quality: f64 },
    // basis
    GenBasis,
}

fn main() {
    let Ok(args) = parse_args() else {
        eprintln!("invalid args");
        return;
    };

    match args {
        Args::Compress { path, quality } => {
            println!("compressing `{path}` with quality {quality}");
            let img = image::open(&std::path::Path::new(&path)).expect("failed to open image");
            let output_buffer = jpeg::compress_image(&img, quality);
            output_buffer.save(OUTPUT).expect("failed to save image");
            println!("result saved to {OUTPUT}");
        }
        Args::GenBasis => basis::generate_basis_img(),
    };
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    let args = match pargs.subcommand()?.as_deref() {
        // compress <path> <quality>
        Some("compress") => Args::Compress {
            path: pargs.free_from_str()?,
            quality: pargs.free_from_str()?,
        },
        Some("basis") => Args::GenBasis,
        Some(x) => {
            eprintln!("unknown command: {x}");
            exit(1);
        }
        None => {
            eprintln!("no command specified");
            exit(1);
        }
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("warning: unknown argument(s): {remaining:?}.");
    }

    Ok(args)
}
