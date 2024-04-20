mod sort;

use sort::{quick_sort, selection_sort, insertion_sort, merge_sort};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Employee {
    name: String,
    age: u32,
    salary: u32,
}

fn main() {
    // Test with integers
    let numbers = vec![15, 0, 57, 100, -4];
    let compare_int = |a: &i32, b: &i32| a < b;

    println!("Original integers: {:?}\n", numbers);

    // Quick sort
    let mut numbers_quick_int = numbers.clone();
    quick_sort::sort(&mut numbers_quick_int, &compare_int);
    println!("Quick sorted integers: {:?}", numbers_quick_int);

    // Selection sort
    let mut numbers_selection_int = numbers.clone();
    selection_sort::sort(&mut numbers_selection_int, &compare_int);
    println!("Selection sorted integers: {:?}", numbers_selection_int);

    // Insertion sort
    let mut numbers_insertion_int = numbers.clone();
    insertion_sort::sort(&mut numbers_insertion_int, &compare_int);
    println!("Insertion sorted integers: {:?}", numbers_insertion_int);

    // Merge sort
    let mut numbers_merge_int = numbers.clone();
    merge_sort::sort(&mut numbers_merge_int, &compare_int);
    println!("Merge sorted integers: {:?}", numbers_merge_int);

    // Test with floats
    let floats = vec![0.4, 27.4, 54.7, 1.84, 3.6];
    let compare_float = |a: &f64, b: &f64| a > b;

    println!("\nOriginal floats: {:?}\n", floats);

    // Quick sort
    let mut floats_quick = floats.clone();
    quick_sort::sort(&mut floats_quick, &compare_float);
    println!("Quick sorted floats: {:?}", floats_quick);

    // Selection sort
    let mut floats_selection = floats.clone();
    selection_sort::sort(&mut floats_selection, &compare_float);
    println!("Selection sorted floats: {:?}", floats_selection);

    // Insertion sort
    let mut floats_insertion = floats.clone();
    insertion_sort::sort(&mut floats_insertion, &compare_float);
    println!("Insertion sorted floats: {:?}", floats_insertion);

    // Merge sort
    let mut floats_merge = floats.clone();
    merge_sort::sort(&mut floats_merge, &compare_float);
    println!("Merge sorted floats: {:?}", floats_merge);

    // Test with strings
    let strings = vec!["aitu", "student", "blockchain", "rust", "solana"];
    let compare_str = |a: &&str, b: &&str| a < b;

    println!("\nOriginal strings: {:?}\n", strings);

    // Quick sort
    let mut strings_quick = strings.clone();
    quick_sort::sort(&mut strings_quick, &compare_str);
    println!("Quick sorted strings: {:?}", strings_quick);

    // Selection sort
    let mut strings_selection = strings.clone();
    selection_sort::sort(&mut strings_selection, &compare_str);
    println!("Selection sorted strings: {:?}", strings_selection);

    // Insertion sort
    let mut strings_insertion = strings.clone();
    insertion_sort::sort(&mut strings_insertion, &compare_str);
    println!("Insertion sorted strings: {:?}", strings_insertion);

    // Merge sort
    let mut strings_merge = strings.clone();
    merge_sort::sort(&mut strings_merge, &compare_str);
    println!("Merge sorted strings: {:?}", strings_merge);

    let employees = vec![
        Employee { name: "Marat".to_string(), age: 20, salary: 500000 },
        Employee { name: "Aibek".to_string(), age: 25, salary: 600000 },
        Employee { name: "Beka".to_string(), age: 27, salary: 450000 },
        Employee { name: "Sanat".to_string(), age: 40, salary: 700000 },
    ];

    let compare_name = |a: &Employee, b: &Employee| a.name < b.name;

    // Compare employees by age
    let compare_age = |a: &Employee, b: &Employee| a.age < b.age;

    // Compare employees by salary
    let compare_salary = |a: &Employee, b: &Employee| a.salary < b.salary;

    println!("\nOriginal employees: {:?}\n", employees);

    // Quick sort employees by name
    let mut employees_quick_name = employees.clone();
    quick_sort::sort(&mut employees_quick_name, &compare_name);
    println!("Quick sorted employees by name: {:?}", employees_quick_name);

    // Quick sort employees by age
    let mut employees_quick_age = employees.clone();
    quick_sort::sort(&mut employees_quick_age, &compare_age);
    println!("Quick sorted employees by age: {:?}", employees_quick_age);

    // Quick sort employees by salary
    let mut employees_quick_salary = employees.clone();
    quick_sort::sort(&mut employees_quick_salary, &compare_salary);
    println!("Quick sorted employees by salary: {:?}", employees_quick_salary);

    println!("\nOriginal employees: {:?}\n", employees);

    // Selection sort employees by name
    let mut employees_selection_name = employees.clone();
    quick_sort::sort(&mut employees_selection_name, &compare_name);
    println!("Selection sorted employees by name: {:?}", employees_quick_name);

    // Selection sort employees by age
    let mut employees_selection_age = employees.clone();
    quick_sort::sort(&mut employees_selection_age, &compare_age);
    println!("Selection sorted employees by age: {:?}", employees_quick_age);

    // Selection sort employees by salary
    let mut employees_selection_salary = employees.clone();
    quick_sort::sort(&mut employees_selection_salary, &compare_salary);
    println!("Selection sorted employees by salary: {:?}", employees_quick_salary);

    println!("\nOriginal employees: {:?}\n", employees);

    // Insertion sort employees by name
    let mut employees_insertion_name = employees.clone();
    quick_sort::sort(&mut employees_insertion_name, &compare_name);
    println!("Insertion sorted employees by name: {:?}", employees_quick_name);

    // Insertion sort employees by age
    let mut employees_insertion_age = employees.clone();
    quick_sort::sort(&mut employees_insertion_age, &compare_age);
    println!("Insertion sorted employees by age: {:?}", employees_quick_age);

    // Insertion sort employees by salary
    let mut employees_insertion_salary = employees.clone();
    quick_sort::sort(&mut employees_insertion_salary, &compare_salary);
    println!("Insertion sorted employees by salary: {:?}", employees_quick_salary);

    println!("\nOriginal employees: {:?}\n", employees);

    // Merge sort employees by name
    let mut employees_merge_name = employees.clone();
    quick_sort::sort(&mut employees_merge_name, &compare_name);
    println!("Merge sorted employees by name: {:?}", employees_quick_name);

    // Merge sort employees by age
    let mut employees_merge_age = employees.clone();
    quick_sort::sort(&mut employees_merge_age, &compare_age);
    println!("Merge sorted employees by age: {:?}", employees_quick_age);

    // Merge sort employees by salary
    let mut employees_merge_salary = employees.clone();
    quick_sort::sort(&mut employees_merge_salary, &compare_salary);
    println!("Merge sorted employees by salary: {:?}", employees_quick_salary);
}
