#![allow(dead_code)]
/* Result<T, E> */
// Ok(T) : Element T was found
// Err(E): Error found with element E

// there are many combinators which overlap between Option and Result

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();

    first_number * second_number
}

fn part1() {
    let twenty = multiply("10", "2");

    println!("double: {}", twenty);

    let tt = multiply("t", "2");
    println!("double: {}", tt);
}

// main can also return a Result
/* use std::num::ParseIntError; */

/* Map for result */
// code with match statement :-
use std::num::ParseIntError;

fn multiply_v2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_num) => match second_number_str.parse::<i32>() {
            Ok(second_num) => Ok(first_num * second_num),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => {
            println!("Error: {}", e)
        }
    }
}

fn part2() {
    // This still presents a reasonable answer.
    let twenty = multiply_v2("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply_v2("t", "2");
    print(tt);
}
// using Option's map and and_then for Result
fn multiply_v3(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_num| {
        second_number_str
            .parse::<i32>()
            .map(|second_num| first_num * second_num)
    })
}

fn part3() {
    // This still presents a reasonable answer.
    let twenty = multiply_v3("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply_v3("t", "2");
    print(tt);
}

/* Result alias */
// e.g. io::Result<T> is alias for io::Result<T, io::Error>
type AliasResult<T> = Result<T, ParseIntError>;

fn multiply_v4(first_number_str: &str, second_number_str: &str) -> AliasResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_num| {
        second_number_str
            .parse::<i32>()
            .map(|second_num| first_num * second_num)
    })
}

fn part4() {
    print(multiply_v4("10", "2"));
    print(multiply_v4("t", "2"));
}

// early return allows for better code

fn multiply_v5(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_num) => first_num,
        Err(e) => return Err(e),
    };
    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_num) => second_num,
        Err(e) => return Err(e),
    };
    Ok(first_number * second_number)
}

fn part5() {
    print(multiply_v5("10", "2"));
    print(multiply_v5("t", "2"));
}

// ? to unwrap without panic
fn multiply_v6(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    // legacy method :-
    // let first_number = try!(first_number_str.parse::<i32>());
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number*second_number)
}

fn part6() {
    print(multiply_v6("10", "2"));
    print(multiply_v6("t", "2"));
}

fn main() /* -> Result<(), ParseIntError> */
{
    // part1();
    /* let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(num) => num,
        Err(e) => return Err(e)
    };
    println!("{}", number);
    Ok(()) */
    // part2();
    // part3();
    // part4();
    // part5();
    part6();
}
