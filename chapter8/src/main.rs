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

fn main() {
    // part1();
    // part2();
    // part3();
    // part4();
    // part5();
    // part6();
    part7();
}
