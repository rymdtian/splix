use std::path::PathBuf;
use std::error::Error;
use clap::Parser;
use image;

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

fn main() {
    let cli = Cli::parse();
    let image_path = cli.image;
    let rows = cli.rows;
    let cols = cli.cols;

    // Validate image
    let img = match image::open(&image_path) {
        Ok(img) => img,
        Err(_) => {
            eprintln!("splix: image: The provided file '{}' is not a valid image", image_path.display());
            return;
        }
    };

    // Validate user input
    if let Some(rows) = rows {
        if rows == 0 {
            eprintln!("splix: rows: Must be a positive integer");
            return;
        }
    }
    
    if let Some(cols) = cols {
        if cols == 0 {
            eprintln!("splix: cols: Must be a positive integer");
            return;
        }
    }

    if rows.is_none() && cols.is_none() {
        eprintln!("splix: At least one of '--rows', '--cols' needs to be specified.");
        return;
    }
}

