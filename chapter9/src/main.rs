#![allow(dead_code)]
// intro

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

fn part1() {
    fizzbuzz_to(100);
}

// Associated functions and methods

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // associated function
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // method
    // &self is sugar for self: &Self
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// consuming using method

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying ({}, {})", first, second);
    }
}

fn part2() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point { x: 3.0, y: 4.0 },
    };
    println!("Area of rectangle = {}", rectangle.area());

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}

// closures

// they can capture enclosing environment
// input and return type can be inferred

fn part3() {
    let outer_var = 42;
    // fn function(i: i32) -> i32 {
    //     i + outer_val
    // }
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;
    println!("Closure1: {}", closure_annotated(1));
    println!("Closure2: {}", closure_inferred(1));
    // note that if a type has been inferred, then other type cannot be used for the same closure.
    // e.g. if we now call for 1i64, it will be invalid
    let one = || 1;
    println!("{}", one());
}

// capturing in closures
// three ways to capture:-
// 1. by reference
// 2. by mutable reference
// 3. by value
// they can automatically determine it reducing requirement for explicit declaration

fn part4() {
    use std::mem;

    let color = String::from("green");
    // reference borrowed immutably
    let print = || println!("color: {}", color);
    print();

    // these are allowed
    let _reborrow = &color;
    print();

    let _color_moved = color;

    // reference borrowed mutably
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("Count is {}", count);
    };
    inc();

    // let _reborrow = &count;
    // cant do let _reborrow = &count because mutably borrowed

    inc();

    // here we can reborrow
    let _count_reborrowed = &count;

    let movable = Box::new(3);

    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    // to force move, move keyword can be used e.g. in multithreading

    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // now we cant use haystack in this scope because it has been moved to closure
}

// closures as input parameters for functions

// traits
// 1. Fn -> reference
// 2. FnMut -> mutable reference
// 3. FnOnce -> move value

// compiler captures variable in least restrictive manner possible, even though only more restrictive trait has been implemented

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn part5() {
    use std::mem;

    let greeting = "hello";

    // this is an owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}", greeting);

        farewell.push_str("!!!");

        println!("Then I screamed {}", farewell);
        println!("Now I can sleep...");
        mem::drop(farewell);
    };

    apply(diary);
    // this wont work if FnMut or Fn only was implemented because we have moved the value

    let double = |x| x * 2;
    println!("3 doubled {}", apply_to_3(double));
}

// closure as output
// Fn, FnMut and FnOnce trait implementations can be passed
// must use move keyword to prevent invalid references in closure

fn create_fn() -> impl Fn() {
    let text = "function!".to_owned();

    move || println!("This is a {}", text)
}

fn create_fn_mut() -> impl FnMut() {
    let text = "mutable function!".to_owned();
    move || println!("This is a {}", text)
}

fn create_fn_once() -> impl FnOnce() {
    let text = "FnOnce function!".to_owned();
    move || println!("This is a {}", text)
}

fn part6() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fn_mut();
    let fn_once = create_fn_once();

    fn_plain();
    fn_mut();
    fn_once();
}

// some examples of use of closures

// Iterator::any -> function which if passed an iterator will return true if any element satisfies a predicate

fn part7() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // destructuring needed for .iter() not for .into_iter()
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));

    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
}

// Iterator::find
// It searches for first value which satisfies a condition. If none satisfy, returns None.

fn part8() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    // && because we want to refer an item of &i32 returned by .iter()
    println!("Find two in vec1: {:?}", iter.find(|&&x| x == 2));

    println!("Find two in vec2: {:?}", into_iter.find(|&x| x == 2));

    // for index of a position, use Iterator::position
}

// Higher order functions

// these are functions which take a function and provide another function
// they can drastically reduce code size but provide less control

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn part9() {
    let upper = 1000;
    println!("Sum of squared odd numbers");
    // imperative approach
    let mut acc = 0;
    for n in 0.. {
        let n_square = n * n;
        if n_square >= upper {
            break;
        } else if is_odd(n_square) {
            acc += n_square;
        }
    }
    println!("First Approach: {}", acc);

    // functional approach
    let required_sum: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_square| n_square < upper)
        .filter(|&n_square| is_odd(n_square))
        .sum();
    println!("Second Approach: {}", required_sum);
}

// diverging functions
// they never return

fn foo() -> ! {
    panic!("I cant return anything! not like returning ()");
}
// it can be cast to any type therefore used in many places

// also used in network servers. process terminators

fn part10() {
    fn sum_odd(up_to: u32) -> u32 {
        let mut sum = 0;
        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                false => continue, //continue returns never type which can be cast to u32 implicitly
            };
            sum += addition;
        }
        sum
    }
    println!("Sum of odd numbers: {}", sum_odd(10));
}

fn main() {
    // part1();
    // part2();
    // part3();
    // part4();
    // part5();
    // part6();
    // part7();
    // part8();
    // part9();
    part10();
}
