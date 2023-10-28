/* Rc */
// reference counting allows for keeping track of multiple references

// reference increasing on each clone but deep copy is not made

use std::rc::Rc;

fn part1() {
    let rc_examples = "Rc examples".to_string();

    {
        println!(" --- rc_a is created ---");
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        {
            println!("--- rc_a is cloned to rc_b ---");
            let rc_b = Rc::clone(&rc_a);
            println!("Reference count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        println!("--- rc_a is dropped out of scope ---");
    }
    // note that rc_examples was moved so it cannot be used
}


/* Arc - sharing data between threads */
use std::time::Duration;
use std::sync::Arc;
use std::thread;

fn part2() {
    let apple = Arc::new("The same apple");
    for _ in 0..10 {
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    // part1();
    part2();
}
