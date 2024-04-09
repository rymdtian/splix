# Splix

Splix is a command-line tool written in Rust for splitting images into multiple images based on specified rows, columns, or a custom grid. It offers a simple and efficient way to break down images into smaller components, facilitating various image processing tasks.

## Disclaimer
first time writing Rust so code might be terrible

## Options
```plaintext
Usage: splix [OPTIONS] --images <IMAGES>

Options:
  -i, --images <IMAGES>          Path of the image(s) to convert.
                                 Specify the path of an image, or a directory of images. [aliases: image]
  -r, --rows <ROWS>              The number of rows to split the image into.
                                 Specify an integer, or a list of integers:
                                 -r 4        Split the image into 4 equal rows.
                                 -r 2,3,1,5  Split the image into four rows of different heights.
                                             The image will be divided vertically into 2+3+1+5=11 equal sections.
                                             The first row will take up 2 sections, second row 3 sections, etc.
  -c, --cols <COLS>              The number of columns to split the image into.
                                 Speicty an integer, or a list of integers.
                                 -c 4        Split the image into 4 equal columns.
                                 -c 2,3,1,5  Split the image into four columns of different widths.
                                             The image will be divided horizontally into 2+3+1+5=11 equal sections.
                                             The first column will take up 2 sections, second column 3 sections, etc.
  -o, --output-dir <OUTPUT_DIR>  Directory to save the splixed images in. Default: `./splixed-images`
  -R, --recursive                Enable recursive search for images in specified directory
  -h, --help                     Print help
```
## Future
- grid of any combination of row heights/widths

Future argument details:
-g --grid
i_1:j_1,...,j_n/i_2,k_1,...,k_n/...
custom grid, any number of rows of any size and any number of cols of any size
each row: [row size]:{col sizes}/
