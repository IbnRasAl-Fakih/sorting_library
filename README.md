# Sorting Library in Rust

This Rust library crate provides sorting functions for various data types including integers, floats, strings, and custom data structures.

## Sorting Algorithms Implemented

- Quick sort
- Selection sort
- Insertion sort
- Merge sort

## Usage

To use this library crate in your Rust project, follow these steps:

1. Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
sorting_library = { git = "https://github.com/IbnRasAl-Fakih/sorting_library" }
```

2. In your Rust project, import the sorting functions you need:
3. 
```code
use sorting_library::sort::{quick_sort, selection_sort, insertion_sort, merge_sort};
```

4. Use the sorting functions with appropriate comparison functions for your data types.
5. 
```code
// Example with integers
let mut numbers = vec![4, 2, 5, 1, 3];
let compare_asc = |a: &i32, b: &i32| a < b;
quick_sort::sort(&mut numbers, &compare_asc);
println!("Quick sorted integers: {:?}", numbers);
```

## Screenshots
![image_2024-04-20_21-02-55](https://github.com/IbnRasAl-Fakih/sorting_library/assets/121658609/042f0d93-9274-413f-89c8-c823f38ad0fd)
![image_2024-04-20_21-04-05](https://github.com/IbnRasAl-Fakih/sorting_library/assets/121658609/7005ca58-f106-425c-a826-644aec37ba97)


## Contributing
Contributions are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
