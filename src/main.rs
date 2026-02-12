use std::io;
use std::io::Write;
use std::process::ExitCode;

use clap::Parser;

use rs_pnm_shrink::AspectMode;
use rs_pnm_shrink::FilterMode;
use rs_pnm_shrink::ResizeConfig;
use rs_pnm_shrink::ShrinkConfig;
use rs_pnm_shrink::bytes2image;
use rs_pnm_shrink::img2wtr;
use rs_pnm_shrink::reader2bytes;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Shrink images to PNM format via stdin/stdout",
    long_about = "A WASI-compliant tool to resize images from stdin and output them as PNM to stdout. \
                  Supports various resizing hints, explicit dimensions, and filtering modes."
)]
struct Cli {
    /// Resize mode hint.
    ///
    /// Defines the target square dimensions.
    /// Available hints:
    /// - min/minimal (8x8)
    /// - tiny (16x16)
    /// - small (32x32)
    /// - normal (64x64)
    /// - large (128x128)
    /// - Large (256x256)
    /// - LARGE (512x512)
    /// - huge (1024x1024)
    /// - Huge (2048x2048)
    /// - HUGE (4096x4096)
    #[arg(
        short,
        long,
        default_value = "min",
        value_name = "HINT",
        verbatim_doc_comment
    )]
    size_hint: String,

    /// Explicit target width in pixels.
    ///
    /// If provided, it overrides the width derived from the size-hint.
    #[arg(long, value_name = "PIXELS")]
    width: Option<u32>,

    /// Explicit target height in pixels.
    ///
    /// If provided, it overrides the height derived from the size-hint.
    #[arg(long, value_name = "PIXELS")]
    height: Option<u32>,

    /// Aspect ratio handling strategy.
    ///
    /// - preserve: Resize to fit while maintaining aspect ratio.
    /// - ignore: Stretch or squash to match exactly.
    /// - clip: Resize to fill, cropping if necessary.
    #[arg(short, long, value_enum, default_value_t = AspectMode::default(), verbatim_doc_comment)]
    aspect: AspectMode,

    /// Filtering algorithm for resampling.
    ///
    /// - nearest: Fastest, blocky.
    /// - triangle: Linear interpolation.
    /// - catmull-rom: Cubic interpolation (Balanced).
    /// - gaussian: Blurry but smooth.
    /// - lanczos3: Best quality, slowest, sometimes with artifacts.
    #[arg(short, long, value_enum, default_value_t = FilterMode::default(), verbatim_doc_comment)]
    filter: FilterMode,

    /// Maximum input image size in bytes.
    ///
    /// Prevents excessive memory usage.
    #[arg(long, default_value_t = 1048576, value_name = "BYTES")]
    input_limit: u64,
}

fn sub(cli: Cli) -> Result<(), io::Error> {
    let mut new_sz = ResizeConfig::from_size_hint(&cli.size_hint).ok_or_else(|| {
        io::Error::other(format!("Invalid resize config value: {}", cli.size_hint))
    })?;

    if let Some(w) = cli.width {
        new_sz.new_width = w;
    }
    if let Some(h) = cli.height {
        new_sz.new_height = h;
    }

    let cfg = ShrinkConfig {
        aspect: cli.aspect,
        filter: cli.filter,
        new_sz,
    };

    let input_reader = io::stdin();

    let img_data = reader2bytes(input_reader, cli.input_limit)?;
    let img = bytes2image(&img_data)?;
    let converted_img = cfg.convert(&img);

    let mut output_writer = io::stdout();

    // img2wtr requires Write + Seek. For stdout, we cannot seek.
    // So, we write to a buffer first then to the output.
    let mut buf = io::Cursor::new(Vec::new());
    img2wtr(&converted_img, &mut buf)?;
    output_writer.write_all(buf.into_inner().as_slice())?;
    output_writer.flush()?;

    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    sub(cli).map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
