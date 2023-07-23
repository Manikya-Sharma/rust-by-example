#![allow(dead_code)]
/* structs */
//
// // There are 3 types of structs:-
// // 1. Tuple structs i.e. named tuples
// // 2. regular structs
// // 3. Unit structs
//
// use std::fmt;
//
// fn main() {
//
// let name = String::from("Peter");
// let age = 14;
// let peter = Person {name, age};
// println!("{}", peter);
//
// let point = Point {x: 5, y: 10};
// let another_point = Point{x: 7, ..point}; // use field of other point
//
// // destructure point
// let Point {x: left_edge, y: top_edge} = point;
//
// let _rectangle = Rectangle {
// top_left: Point {x: left_edge, y: top_edge },
// bottom_right: another_point
// };
//
// // tuple struct
// let _unit = Unit;
// let rgb = RGB(255, 0, 0);
// println!("Color: {:?}", rgb);
// // destructuring tuple struct
// let RGB(red, blue, green) = rgb;
// println!("red: {}, blue: {}, green: {}", red, blue, green);
// }
//
// struct Person {
// name: String,
// age: u8,
// }
//
// impl fmt::Display for Person {
// fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// write!(f, "I am {} aged {}", self.name, self.age)
// }
// }
//
// // unit struct
// struct Unit;
//
// // tuple struct
// #[derive(Debug)]
// struct RGB (u8, u8, u8);
//
// struct Point {
// x: i32,
// y: i32
// }
//
// struct Rectangle {
// top_left: Point,
// bottom_right: Point,
// }
//

// structs activity

// fn main() {
//     let rect = Rectangle {
//         top_left: Point { x: 1, y: 2 },
//         bottom_right: Point { x: 3, y: 5 },
//     };
//     println!("Area = {}", rect.area());
//     let my_square = square(Point { x: 1, y: 2 }, 5);
//     println!("Area of square: {}", my_square.area());
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// struct Rectangle {
//     top_left: Point,
//     bottom_right: Point,
// }

// impl Rectangle {
//     fn area(&self) -> i32 {
//         let Point { x: x1, y: y1 } = self.top_left;
//         let Point { x: x2, y: y2 } = self.bottom_right;
//         (x2 - x1) * (y2 - y1)
//     }
// }

// fn square(top_left: Point, side: i32) -> Rectangle {
//     let Point { x, y } = top_left;
//     Rectangle {
//         top_left: Point { x, y },
//         bottom_right: Point {
//             x: x + side,
//             y: y + side,
//         },
//     }
// }

// enums:-

/* fn main() {
     let pressed = WebEvent::KeyPress('X');
    let pasted = WebEvent::Paste("my_text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char), // just like tuple struct
    Paste(String),
    Click { x: i64, y: i64 }, // C like struct
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page load"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Pressed {}", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    };
}
*/

// type aliases e.g. self alias in implementation blocks
/*
fn main() {
    let x = Operations::Add;
    println!("{}", x.run(5, 6));
}

enum VeryVeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVeryVerboseEnumOfThingsToDoWithNumbers;

impl Operations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
*/
// use declaration

/*
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor have no money"),
    }
    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
*/

// c-like enums

/* enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("two is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
 */

// Linked List

use crate::List::*;

enum List {
    // or Node
    Cons(u32, Box<List>), // or data
    Nil,                  // or NULL
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(3);
    list = list.prepend(4);
    list = list.prepend(5);

    println!("Length of linked list: {}", list.len());
    println!("{}", list.stringify());
}
