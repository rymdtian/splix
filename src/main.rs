use clap::Parser;
use image::*;
use std::path::PathBuf;
use std::{fs, process, vec};

/// Command-line arguments for splix.
#[derive(Parser)]
struct Cli {
    /// Path of the image to convert.
    #[arg(short, long)]
    image: PathBuf,

    /// The number of rows to split the image into.
    /// You can either specify an integer, or a list of integers:
    /// -r 4        This will split the image into 4 equal rows
    /// -r 2,3,1,5  This will split the image into four rows of different heights.
    ///             The image will be divided vertically into 2+3+1+5=11 equal sections.
    ///             The first row will take up 2 sections, second row 3 sections, etc.
    #[arg(short, long, value_delimiter = ',', verbatim_doc_comment)]
    rows: Option<Vec<u32>>,

    /// The number of columns to split the image into.
    /// You can either specify an integer, or a list of integers.
    /// -c 4        This will split the image into 4 equal rows):
    /// -c 2,3,1,5  This will split the image into four columns of different widths.
    ///             The image will be divided horizontally into 2+3+1+5=11 equal sections.
    ///             The first column will take up 2 sections, second column 3 sections, etc.
    #[arg(short, long, value_delimiter = ',', verbatim_doc_comment)]
    cols: Option<Vec<u32>>,

    /// Directory to save the splixed images in. Default: `./splixed-images`.
    #[arg(short = 'o', long = "output-dir")]
    output_dir: Option<PathBuf>,
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
    let rows = &cli.rows;
    let cols = &cli.cols;

    // Validate image
    if let Err(_) = image::open(&image_path) {
        return Err(format!(
            "splix: image: The provided file '{}' is not a valid image",
            image_path.display()
        ));
    }

    if rows.is_none() && cols.is_none() {
        return Err("splix: At least one of '--rows', '--cols' needs to be specified".to_string());
    }

    if let Some(rows) = rows {
        for row_val in rows {
            if *row_val == 0 {
                return Err("splix: rows: Row size(s) must be greater than zero".to_string());
            }
        }
    }

    if let Some(cols) = cols {
        for col_val in cols {
            if *col_val == 0 {
                return Err("splix: cols: Column size(s) must be greater than zero".to_string());
            }
        }
    }

    Ok(())
}

/// Splits the input image into the specified number of rows and columns.
///
/// # Arguments
///
/// * `img_path` - Path to the input image file.
/// * `rows` - Number of rows to split the image into. Provide a single integer for equal division, or a list of integers for custom division.
/// * `cols` - Number of columns to split the image into. Provide a single integer for equal division, or a list of integers for custom division.
///
/// # Returns
///
/// A vector of split images.
fn split_image(img_path: &PathBuf, rows: &Vec<u32>, cols: &Vec<u32>) -> Vec<DynamicImage> {
    let mut img = image::open(img_path).unwrap();
    let mut split_images = Vec::new();
    let (width, height) = img.dimensions();

    let sum_rows: u32 = rows.iter().sum();
    let sum_cols: u32 = cols.iter().sum();

    let rows = rows.clone();
    let cols = cols.clone();

    if sum_rows > height {
        eprintln!(
            "splix: rows: The sum of provided rows ({}) exceeds image height ({})",
            sum_rows, height
        );
        process::exit(1);
    }

    if sum_cols > width {
        eprintln!(
            "splix: cols: The sum of provided columns ({}) exceedsd image width ({})",
            sum_cols, width
        );
        process::exit(1);
    }

    let rows = if rows.len() > 1 {
        rows
    } else {
        vec![1; rows[0] as usize]
    };

    let cols = if cols.len() > 1 {
        cols
    } else {
        vec![1; cols[0] as usize]
    };

    let row_height = height / sum_rows;
    let col_width = width / sum_cols;

    let mut x = 0;
    let mut y = 0;

    for i in 0..rows.len() {
        y = if i == 0 {
            0
        } else {
            y + row_height * rows[i - 1]
        };

        let crop_height = if i == rows.len() - 1 {
            height - y
        } else {
            row_height * rows[i]
        };

        for j in 0..cols.len() {
            x = if j == 0 {
                0
            } else {
                x as u32 + col_width * cols[j - 1]
            };

            let crop_width = if j == cols.len() - 1 {
                width - x
            } else {
                col_width * cols[j]
            };

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
    output_directory: &PathBuf,
    img_file_name: &str,
    img_format: &ImageFormat,
    img_format_str: &str,
    num_rows: usize,
    num_cols: usize,
) {
    let mut output_directory = output_directory.clone();
    if !output_directory.exists() {
        if let Err(err) = fs::create_dir_all(&output_directory) {
            eprintln!(
                "splix: Failed to create directory {}: {}",
                output_directory.to_string_lossy(),
                err
            );
        }
    }
    output_directory.push("placeholder");

    for i in 0..num_rows {
        for j in 0..num_cols {
            output_directory
                .set_file_name(format!("{}-r{}c{}.{}", img_file_name, i, j, img_format_str));
            if output_directory.exists() {
                if let Err(err) = fs::remove_file(&output_directory) {
                    eprintln!("Failed to remove existing image {}: {}", i, err);
                    continue; // Skip saving this image if removing the existing one fails
                }
            }

            if let Err(err) =
                split_images[i * num_cols + j].save_with_format(&output_directory, *img_format)
            {
                eprintln!("splix: Failed to save image #{}: {}", i, err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_image_correct_number_of_images() {
        let path = &PathBuf::from("./assets/16x16.png");
        assert!(split_image(path, &vec![3], &vec![5]).len() == 15);
        assert!(split_image(path, &vec![1], &vec![1]).len() == 1);
        assert!(split_image(path, &vec![16], &vec![16]).len() == 256);
    }
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = validate_args(&cli) {
        eprintln!("{}", err);
        return;
    }

    let img_path = cli.image;
    let rows = cli.rows.unwrap_or(vec![1]);
    let cols = cli.cols.unwrap_or(vec![1]);
    let output_directory = cli.output_dir.unwrap_or(PathBuf::from("splixed-images"));

    let img_file_name = img_path.file_stem().unwrap().to_string_lossy();
    let img_format = io::Reader::open(&img_path).unwrap().format().unwrap();
    let img_format_str = img_format.extensions_str()[0];

    let split_images = split_image(&img_path, &rows, &cols);

    save_images(
        &split_images,
        &output_directory,
        &img_file_name,
        &img_format,
        img_format_str,
        if rows.len() > 1 {
            rows.len()
        } else {
            rows[0] as usize
        },
        if cols.len() > 1 {
            cols.len()
        } else {
            cols[0] as usize
        },
    );
}
