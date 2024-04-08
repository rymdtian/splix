use clap::Parser;
use image::*;
use std::cmp;
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

fn main() {
    let cli = Cli::parse();

    if let Err(err) = validate_args(&cli) {
        eprintln!("{}", err);
    }

    let img_path = cli.image;
    let rows = cli.rows.unwrap_or(1);
    let cols = cli.cols.unwrap_or(1);

    split_image(&img_path, rows, cols);
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

    if rows.is_none() && cols.is_none() {
        return Err("splix: At least one of '--rows', '--cols' needs to be specified.".to_string());
    }

    Ok(())
}

fn split_image(img_path: &PathBuf, mut rows: u32, mut cols: u32) -> Vec<DynamicImage> {
    let mut img = image::open(img_path).unwrap();
    let mut split_images: Vec<DynamicImage> = Vec::new();
    let (width, height) = img.dimensions();

    rows = rows.max(1);
    cols = cols.max(1);
    rows = rows.min(height);
    cols = cols.min(width);

    for i in 0..rows {
        split_images.push(img.crop(
            0,
            height / rows * i,
            width,
            cmp::max(
                height / rows * (i + 1) - height / rows * i,
                height - height / rows * i,
            ),
        ));
    }

    for i in 0..split_images.len() {
        let mut split_image = split_images[i].clone();
        for j in 0..cols {
            split_images.push(split_image.crop(
                width / cols * j,
                height / rows * i as u32,
                cmp::max(
                    width / cols * (j + 1) - width / cols * j,
                    width - width / cols * j,
                ),
                split_image.height(),
            ));
        }
    }

    if cols > 0 {
        split_images.drain(0..rows as usize);
    }

    split_images
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_image_correct_number_of_images() {
        let path = &PathBuf::from("./test_assets/16x16.png");
        assert!(split_image(path, 3, 5).len() == 15);
        assert!(split_image(path, 1, 1).len() == 1);
        assert!(split_image(path, 16, 16).len() == 256);
        assert!(split_image(path, 17, 17).len() == 256);
    }
}
