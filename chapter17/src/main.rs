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
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y)
            }
        }
    };
}

// implement `add_assign`, `mul_assign` and `sub_assign`
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;
    macro_rules! test {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0_usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);
                    assert_eq!(x, z);
                }
            }
        };
    }

    // test
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}

/* Domain Specific Language */
// it is a mini language embedded in rust
// this allows for concise syntax for special functionality

// e.g. we want a calculator api to be built which works as
// eval 1+2 -> 1 + 2 = 3

macro_rules! calculate {
    (eval $e:expr) => {
        let val:usize = $e;
        println!("{} = {}", stringify!{$e}, val);
    };
}

fn part5() {
    calculate! {
        eval 1 + 2
    }
    calculate! {
        eval (1+2)* (3/4)
    }
}

/* Variadic Interface */
// to allow for a variable number of arguments
macro_rules! calculate2 {
    (eval $e:expr) => {
        let val:usize = $e;
        println!("{} = {}", stringify!($e), val);
    };

    // decompose eval's recursively
    (eval $e:expr, $(eval $es:expr), +) => {
        {
            calculate2! {eval $e}
            calculate2! { $(eval $es), +}
        }
    };
}

fn part6() {
    calculate2! {
        eval 1+2,
        eval 3+4,
        eval (2*3)+1
    }
}

fn main() {
    // part1();
    // part2();
    // part3();
    // part4();
    // part5();
    part6();
}
