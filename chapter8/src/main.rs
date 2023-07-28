#![allow(dead_code)]
// if else

fn part1() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!("increase 10 times");
        10 * n
    } else {
        println!("half the number");
        n / 2
    };
    println!("Now big_n: {}", big_n);
}

// loops: loop

fn part2() {
    // loop means an infinite loop
    let mut count = 0u32;
    loop {
        count += 1;

        if count == 3 {
            println!("three!");
            continue;
        }
        println!("Count: {}", count);

        if count == 5 {
            println!("Exiting the loop");
            break;
        }
    }
}

// loop labels

#[allow(unused_labels, unreachable_code)]
fn part3() {
    'outer: loop {
        println!("Inside outer loop");

        'inner: loop {
            println!("Inside inner loop");

            break 'outer;
        }
        println!("Never reached here");
    }
    println!("Out of outer loop");
}

// returning from loop

fn part4() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
            // counter*2 is returned
        }
    };
    println!("Result = {}", result);
}

// while loop

fn part5() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

// for-in loop

fn part6() {
    for n in 1..101 {
        // or for n in 1..=100
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", n);
        }
    }
}

// for with iterators

fn part7() {
    // for loop applies into_iter method on a collection

    // iter - borrows element therefore allows reuse of collection
    let names = vec!["nameA", "nameB", "nameC"];

    for name in names.iter() {
        match name {
            &"nameC" => println!("Special welcome to {}", name),
            _ => println!("Hello {}", name),
        }
    }

    // into_iter - it moves te collection inside loop, therefore, consumes te collection

    // iter_mut - mutably borrows, therefore we can modify collection
    let mut names = vec!["nameA", "nameB", "nameC"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "nameC" => "Special welcome",
            _ => "Hello",
        }
    }
    println!("Modified: {:?}", names);
}

// match

fn part8() {
    // match can be used like a C switch
    let number = 7;
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 9 | 11 => println!("Prime number"),
        13..=19 => println!("Teen"),
        _ => println!("Too big to analyze"),
    }

    let boolean = true;
    let _binary = match boolean {
        true => 1,
        false => 0,
    };
}

// match destructuring

fn part9() {
    println!("Tuples :-");

    let triple = (3, 1, 5);
    match triple {
        (0, y, z) => println!("`0` : `{:?}` : `{:?}", y, z),
        (1, ..) => println!("1 : something"),
        (.., 2) => println!("something : 2"),
        (3, .., 4) => println!("3 : something : 4"),
        _ => println!("It is something not special"),
    }

    println!("Arrays :-");
    let array = [2, 5, 9, 7];
    match array {
        [0, y, z, ..] => println!("0, {y}, {z}"),
        // use _ to ignore single value
        [1, y, ..] => println!("starts with 1 then has {y}"),

        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {second} and other elements are {:?}",
            tail
        ),

        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }

    // enum and structs can also be destructured in obvious manner
    println!("Structs");
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, a), y } => println!("Got {a}"),
        Foo { x, .. } => (),
    }
}

// pointers/ref
// Dereferencing => `*`
// Destructuring => `&`, `ref`, and `ref mut`

fn part10() {
    let reference = &4;
    match reference {
        &val => println!("Got by destructuring, {:?}", val),
    }

    match *reference {
        val => println!("Got by dereferencing: {:?}", val),
    }

    // another way to assign reference
    let ref _a_reference = 4;

    let value = 4;
    match value {
        ref r => println!("Got a reference: {:?}", r),
    }

    // similarly ref mut can be used
    let mut mut_value = 7;
    match mut_value {
        ref mut m => {
            *m += 10; // deref is mandatory before adding
            println!("New value of mut_value is {:?}", m);
        }
    }
}

// match guards

fn part11() {
    let number = 4u8;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater then zero"),
        _ => unreachable!("Should never happen, but mandatory because compiler does not check guard condition in match"),
    }
}

// @ sigil to prevent rebinding

fn age() -> u32 {
    19
}

fn part12() {
    match age() {
        1 => println!("Haven't celebrated first birthday"),
        // this will allow to use range condition and get value
        n @ 1..=12 => println!("A child of age {}", n),
        n @ 13..=19 => println!("A teenager of age {}", n),
        n => println!("A old person of age {}", n),
    }

    // can also use in enum destructuring
    // MyEnum::Variant(n @ 15) => printlN!("15!")
}

// if let -> when we know the situation will never differ

fn part13() {
    let number = Some(7);
    if let Some(i) = number {
        println!("Number matched: {}", i);
    }

    let letter: Option<char> = None;
    // for using _,
    if let Some(i) = letter {
        println!("Got letter {}", i)
    } else {
        println!("Could not match with Some")
    }

    // for particular cases
    let i_like_letters = false;
    let emoticon: Option<i32> = None;
    if let Some(i) = emoticon {
        println!("Matched {}", i);
    } else if i_like_letters {
        println!("Letters, which you like ");
    } else {
        println!("Neither letters nor emoticons..");
    }

    // note that we can use if-let to match enum variants even if it does not implement PartialEq
}

// let-else
// to match a pattern otherwise necessarily do a task (diverge ie.e break/return/panic)

use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut iterator = s.split(' ');

    let (Some(count_str), Some(item)) = (iterator.next(), iterator.next()) else {
        panic!("Cant segment pair");
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("Cant parse the integer");
    };
    (count, item)
}

fn part14() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}

// while let

fn part15() {
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Exiting");
                    optional = None;
                } else {
                    println!("i is {} still", i);
                    optional = Some(i + 1);
                }
            }
            _ => {
                break;
            }
        }
    }

    // equivalent :-
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Exiting");
            optional = None;
        } else {
            println!("i is {} yet", i);
            optional = Some(i + 1);
        }
    }
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
    // part10();
    // part11();
    // part12();
    // part13();
    // part14();
    part15();
}
