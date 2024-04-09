use clap::Parser;
use image::*;
use std::fs;
use std::path::PathBuf;

/// Command-line arguments for splix.
#[derive(Parser)]
struct Cli {
    /// Path of the image to convert.
    #[arg(short, long)]
    image: PathBuf,

    /// Number of rows to split the image into.
    #[arg(short, long)]
    rows: Option<u32>,

    /// Number of columns to split the image into.
    #[arg(short, long)]
    cols: Option<u32>,
}

/// Validates the provided command-line arguments.
///
/// # Arguments
///
/// * `cli` - A reference to the command-line arguments parsed by Clap.
///
/// # Returns
///
/// * `Ok(())` if the arguments are valid, otherwise returns an error message.
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

/// Splits the input image into specified number of rows and columns.
///
/// # Arguments
///
/// * `img_path` - Path to the input image file.
/// * `rows` - Number of rows to split the image into.
/// * `cols` - Number of columns to split the image into.
///
/// # Returns
///
/// A vector of split images.
fn split_image(img_path: &PathBuf, rows: u32, cols: u32) -> Vec<DynamicImage> {
    let mut img = image::open(img_path).unwrap();
    let mut split_images = Vec::new();
    let (width, height) = img.dimensions();

    let rows = rows.clamp(1, height);
    let cols = cols.clamp(1, width);

    let row_height = height / rows;
    let col_width = width / cols;

    for i in 0..rows {
        let y = row_height * i;
        let crop_height = if i == rows - 1 {
            height - y
        } else {
            row_height
        };

        for j in 0..cols {
            let x = col_width * j;
            let crop_width = if j == cols - 1 { width - x } else { col_width };

            let sub_img = img.crop(x, y, crop_width, crop_height);
            split_images.push(sub_img);
        }
    }

    split_images
}

/// Saves the split images to the specified directory.
///
/// # Arguments
///
/// * `split_images` - A reference to a vector containing the split images.
/// * `save_directory_str` - Path to the directory where split images will be saved.
/// * `img_format` - Format of the image.
/// * `img_format_str` - String representation of the image format.
fn save_images(
    split_images: &Vec<DynamicImage>,
    save_directory_str: &str,
    img_format: &ImageFormat,
    img_format_str: &str,
) {
    let mut split_image_path = PathBuf::from(save_directory_str);
    if let Err(err) = fs::create_dir_all(&split_image_path) {
        eprintln!(
            "splix: Failed to create directory {}: {}",
            save_directory_str, err
        );
    }
    split_image_path.push("placeholder_filename");

    for i in 0..split_images.len() {
        split_image_path.set_file_name(format!("{}.{}", i, img_format_str));

        if split_image_path.exists() {
            if let Err(err) = fs::remove_file(&split_image_path) {
                eprintln!("Failed to remove existing image {}: {}", i, err);
                continue; // Skip saving this image if removing the existing one fails
            }
        }

        if let Err(err) = split_images[i].save_with_format(&split_image_path, *img_format) {
            eprintln!("splix: Failed to save image #{}: {}", i, err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_image_correct_number_of_images() {
        let path = &PathBuf::from("./assets/16x16.png");
        assert!(split_image(path, 3, 5).len() == 15);
        assert!(split_image(path, 1, 1).len() == 1);
        assert!(split_image(path, 16, 16).len() == 256);
        assert!(split_image(path, 17, 17).len() == 256);
    }
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = validate_args(&cli) {
        eprintln!("{}", err);
        return;
    }

    let img_path = cli.image;
    let rows = cli.rows.unwrap_or(1);
    let cols = cli.cols.unwrap_or(1);
    let save_directory = "split_images"; // TODO: turn to user argument

    let split_images = split_image(&img_path, rows, cols);
    let img_format = io::Reader::open(&img_path).unwrap().format().unwrap();
    let img_format_str = img_format.extensions_str()[0];
    save_images(&split_images, save_directory, &img_format, img_format_str);
}
