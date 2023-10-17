#![allow(dead_code)]
/* metaprogramming in rust - macros */

// macros end with a bang (!)
// they expand into source code

// unlike C macros, they expand into abstract syntax tree and there are no precedence bugs

// macro_rules!

// macro name say_hello
macro_rules! say_hello {
    // () means no arguments
    () => {
        // macro will expand to this
        println!("Hello!")
    };
}

fn part1() {
    say_hello!();
}

// benefits of macros
// 1. DRY
// 2. Moke your own special syntax
// 3. variable number of arguments (Variadic Interface)

/* Designators */

macro_rules! create_function {
    // ident designator is used for variable/function name
    // arguments in macro are prefixed by dollar
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// create functions foo and bar
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // expr is designator for expressions
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression)
    };
}

fn part2() {
    foo();
    bar();
    print_result!(1u32 + 1);
    // blocks are also expressions!
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    })
}

// available designators:-
// 1. ident
// 2. expr
// 3. block
// 4. item
// 5. literal
// 6. pat (for pattern)
// 7. path
// 8. stmt (statement)
// 9. tt (token tree)
// 10. ty (type)
// 11. vis (visibility qualifier)

/* overload */
// macros can be overloaded

macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    // each arm must end with a semicolon
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

fn part3() {
    test!(1i32+1 == 2i32; and 2i32 *2 == 4i32);
    test!(true; or false);
}

/* Repeat */
// + argument in macro indicates that argument repeats at-least once
// * means zero or more times

macro_rules! find_min {
    // Base case
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        // call find_min on tail of $y
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn part4() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}

// DRY e.g. tests

use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // tt -> token tree
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}:dimensions mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt,) => {
        fn $func<T :$bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {

            }
        }
    };
}

fn main() {
    // part1();
    // part2();
    // part3();
    part4();
}
