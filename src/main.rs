use clap::Parser;
use image::*;
use rayon::prelude::*;
use std::path::PathBuf;
use std::{cmp, fs, vec};
use walkdir::WalkDir;

/// Lightning-fast image splitter.  
#[derive(Parser)]
#[clap(version)]
struct Cli {
    /// Path of the image(s) to convert.
    /// Specify the path of an image, or a directory of images.
    #[arg(verbatim_doc_comment)]
    images: PathBuf,

    /// The number of rows to split the image into.
    /// Specify an integer, or a list of integers.
    /// Ex:
    /// -r 4        Split the image into 4 equal rows.
    /// -r 2,3,1,5  Split the image into four rows of different heights.
    ///             The image will be divided vertically into 2+3+1+5=11 equal sections.
    ///             The first row will take up 2 sections, second row 3 sections, etc.
    #[arg(short, long, value_delimiter = ',', verbatim_doc_comment)]
    rows: Option<Vec<u32>>,

    /// The number of columns to split the image into.
    /// Specity an integer, or a list of integers.
    /// Ex:
    /// -c 4        Split the image into 4 equal columns.
    /// -c 2,3,1,5  Split the image into four columns of different widths.
    ///             The image will be divided horizontally into 2+3+1+5=11 equal sections.
    ///             The first column will take up 2 sections, second column 3 sections, etc.
    #[arg(short, long, value_delimiter = ',', verbatim_doc_comment)]
    cols: Option<Vec<u32>>,

    /// An optional directory to save the splixed images in. Default: `./splixed-images`.
    #[arg(short = 'd', long = "output-dir")]
    output_dir: Option<PathBuf>,

    /// An optional flag to enable recursive search for images in specified directory.
    #[arg(short = 'R', long)]
    recursive: bool,
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
    let img_dir = &cli.images;
    let rows = cli.rows.as_ref();
    let cols = cli.cols.as_ref();

    match img_dir.try_exists() {
        Ok(true) => {}
        Ok(false) | Err(_) => {
            return Err(format!(
                "splix: image: The provided path '{}' does not exist",
                img_dir.display()
            ))
        }
    }

    if rows.is_none() && cols.is_none() {
        return Err("splix: At least one of '--rows', '--cols' needs to be specified".to_string());
    }

    if let Some(rows) = rows {
        if rows.iter().any(|&row_val| row_val == 0) {
            return Err("splix: rows: All row sizes must be greater than zero".to_string());
        }
    }

    if let Some(cols) = cols {
        if cols.iter().any(|&col_val| col_val == 0) {
            return Err("splix: cols: All column sizes must be greater than zero".to_string());
        }
    }

    Ok(())
}

/// Splits the input image into the specified number of rows and columns.
///
/// # Arguments
///
/// * `img` - Image to split.
/// * `rows` - Number of rows to split the image into. Provide a single integer for equal division, or a list of integers for custom division.
/// * `cols` - Number of columns to split the image into. Provide a single integer for equal division, or a list of integers for custom division.
///
/// # Returns
///
/// A vector of split images.
fn split_image(mut img: DynamicImage, rows: &Vec<u32>, cols: &Vec<u32>) -> Vec<DynamicImage> {
    let (width, height) = img.dimensions();
    let sum_rows: u32 = rows.iter().sum();
    let sum_cols: u32 = cols.iter().sum();

    let single_rows: Vec<u32>;
    let rows = if rows.len() > 1 && height >= sum_rows {
        rows
    } else {
        single_rows = vec![1; cmp::min(height as usize, rows[0] as usize)];
        &single_rows
    };

    let single_cols: Vec<u32>;
    let cols = if cols.len() > 1 && width >= sum_cols {
        cols
    } else {
        single_cols = vec![1; cmp::min(width as usize, cols[0] as usize)];
        &single_cols
    };

    let row_height = cmp::max(1, height / sum_rows);
    let col_width = cmp::max(1, width / sum_cols);

    let mut split_images = Vec::new();
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
/// * `output_directory` - Path to the directory where split images will be saved.
/// * `img_file_name` - Path of image excluding parent directories and extension.
/// * `img_format` - Format of the image.
/// * `img_format_str` - String representation of the image format.
/// * `num_rows` - Number of rows the image was split into.
/// * `num_cols` _ Number of columns the image was split into.
fn save_images(
    split_images: &Vec<DynamicImage>,
    output_directory: PathBuf,
    img_file_name: &str,
    img_format: &ImageFormat,
    img_format_str: &str,
    num_cols: usize,
) {
    if !output_directory.exists() {
        if let Err(err) = fs::create_dir_all(&output_directory) {
            eprintln!(
                "splix: Failed to create directory {}: {}",
                output_directory.to_string_lossy(),
                err
            );
        }
    }

    let mut output_directory = output_directory;
    output_directory.push("placeholder");

    split_images.par_iter().enumerate().for_each(|(i, image)| {
        let mut file_path = output_directory.clone();
        file_path.set_file_name(format!(
            "{}-r{}c{}.{}",
            img_file_name,
            i / num_cols,
            i % num_cols,
            img_format_str
        ));

        if file_path.exists() {
            if let Err(err) = fs::remove_file(&file_path) {
                eprintln!(
                    "Failed to remove existing image {}: {}",
                    file_path.file_stem().unwrap().to_string_lossy(),
                    err
                );
                return;
            }
        }

        if let Err(err) = image.save_with_format(&file_path, *img_format) {
            eprintln!(
                "splix: Failed to save image {}: {}",
                file_path.file_stem().unwrap().to_string_lossy(),
                err
            );
        }
    });
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = validate_args(&cli) {
        eprintln!("{}", err);
        return;
    }

    let img_dir = cli.images;
    let rows = cli.rows.unwrap_or(vec![1]);
    let cols = cli.cols.unwrap_or(vec![1]);
    let output_directory = cli.output_dir.unwrap_or(PathBuf::from("splixed-images"));

    WalkDir::new(&img_dir)
        .max_depth(if cli.recursive { usize::MAX } else { 1 })
        .into_iter()
        .par_bridge()
        .filter_map(|entry| entry.ok().filter(|entry| entry.path().is_file()))
        .for_each(|entry| {
            if let Ok(img) = image::open(entry.path()) {
                let split_images = split_image(img, &rows, &cols);
                let img_file_name = &entry.path().file_stem().unwrap().to_string_lossy();
                let img_format = &ImageFormat::from_path(entry.path()).unwrap();
                let img_format_str = img_format.extensions_str()[0];

                save_images(
                    &split_images,
                    output_directory.clone(),
                    img_file_name,
                    img_format,
                    img_format_str,
                    if cols.len() > 1 {
                        cols.len()
                    } else {
                        cols[0] as usize
                    },
                );
            }
        });
}
