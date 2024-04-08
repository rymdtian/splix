use clap::Parser;
use image;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// Path of image to convert
    #[arg(short, long)]
    image: PathBuf,

    /// Number of rows
    #[arg(short, long)]
    rows: Option<u32>,

    /// Number of columns
    #[arg(short, long)]
    cols: Option<u32>,
}

fn validate_args(cli: &Cli) -> Result<(), String> {
    let image_path = cli.image.clone();
    let rows = cli.rows;
    let cols = cli.cols;

    // Validate image
    if let Err(_) = image::open(&image_path) {
        return Err(format!(
            "splix: image: The provided file '{}' is not a valid image",
            image_path.display()
        ));
    }

    // Validate user input
    if let Some(rows) = rows {
        if rows == 0 {
            return Err("splix: rows: Must be a positive integer".to_string());
        }
    }

    if let Some(cols) = cols {
        if cols == 0 {
            return Err("splix: cols: Must be a positive integer".to_string());
        }
    }

    if rows.is_none() && cols.is_none() {
        return Err("splix: At least one of '--rows', '--cols' needs to be specified.".to_string());
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = validate_args(&cli) {
        eprintln!("{}", err);
    }
}
