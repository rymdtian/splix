# Splix

Splix is a command-line tool written in Rust for splitting images into multiple images based on specified rows, columns, or a custom grid. It offers a simple and efficient way to break down images into smaller components, facilitating various image processing tasks.

Disclaimer: first time writing Rust so code might be terrible

Future: 
- variable row heights and col widths
- grid of any combination of row heights/widths

Future argument details:
splix 
-r --row
n [int] - n rows
n_1,n_2,...,n_m {int} - m rows, with height of row being single value with respect to entire height of img

-w --width
-r --row
n [int] - n cols
n_1,n_2,...,n_m {int} - m cols, with width of col being single value with respect to entire width of img

-g --grid
i_1:j_1,...,j_n/i_2,k_1,...,k_n/...
custom grid, any number of rows of any size and any number of cols of any size
each row: [row size]:{col sizes}/
