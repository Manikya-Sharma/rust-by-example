#![allow(dead_code)]
/* Multiple error types */
// sometimes Option has to interact with Result
// or result of one type of error interacts with other

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // error 1
    2 * first.parse::<i32>().unwrap() // error 2
}

fn part1() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    println!("The first doubled is {}", double_first(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {}", double_first(strings));
    // Error 2: Element isn't a number
}

// Method 1: Embed Errors
use std::num::ParseIntError;

fn my_func_1(vec: Vec<&str>) /*  -> Option<Result<i32, ParseIntError>> */
{
    /* vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n)) */

    // can also return Result<Option<i32>, ParseIntError> for which map_or can be used.
}

fn part2() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", my_func_1(numbers));

    println!("The first doubled is {:?}", my_func_1(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {:?}", my_func_1(strings));
    // Error 2: the element doesn't parse to a number
}

// Defining an error type
// good error type should
// 1. represent errors with same type
// 2. present reasonable error message to the user
// 3. Can be compared with other types
// 4. Stores information about the error

use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn my_func_2(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn part3() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(my_func_2(numbers));
    print(my_func_2(empty));
    print(my_func_2(strings));
}

/* Boxing Errors */
// drawback: Error not statically determined

use std::error;

type Result2<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn my_func_3(vec: Vec<&str>) -> Result2<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // Converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // Converts to Box
                .map(|i| 2 * i)
        })
}

fn print_2(result: Result2<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn part4() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print_2(my_func_3(numbers));
    print_2(my_func_3(empty));
    print_2(my_func_3(strings));
}

/* ? can mean unwrap or return Err(From::from(err)), so it can be used for custom error types */

// we can also wrap error in out own custom type instead of Box but such libraries already exist

/* Iterating over results */

// this might happen when Iter::map fails

// filter_map: calls a function and filters out None

fn part5() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);
}

// map_err() can be used with filter_map() to store errors
fn part6() {
    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

// whole collect can give a result

fn part7() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: std::result::Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", numbers);
}

// collect all valid values and failures with partition

fn part8() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(std::result::Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(std::result::Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(std::result::Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

fn main() {
    // part1();
    // part2();
    // part3();
    // part4();
    // part5();
    // part6();
    part7();
}
