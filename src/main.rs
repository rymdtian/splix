use std::path::PathBuf;
use clap::Parser;
use image::io::Reader as ImageReader;

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

    /// Custom grid string 
    #[arg(short, long)]
    grid: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let image_path = cli.image;
    let rows = cli.rows;
    let cols = cli.cols;
    let grid = cli.grid;

    let image_reader = match ImageReader::open(&image_path) {
        Ok(reader) => reader,
        Err(_error) => {
            println!("Error: The provided file '{}' is not a valid image.", image_path.display());
            return;
        }
    };

    if grid.is_none() && rows.is_none() && cols.is_none() {
        return;
    }

    if grid.is_some() && (rows.is_some() && cols.is_some()) {
        println!("'grid' specified so ignoring 'rows' and 'cols'");
    }
}

