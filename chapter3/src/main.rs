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

fn main() {
    let rect = Rectangle {
        top_left: Point { x: 1, y: 2 },
        bottom_right: Point { x: 3, y: 5 },
    };
    println!("Area = {}", rect.area());
    let my_square = square(Point { x: 1, y: 2 }, 5);
    println!("Area of square: {}", my_square.area());
}

struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> i32 {
        let Point { x: x1, y: y1 } = self.top_left;
        let Point { x: x2, y: y2 } = self.bottom_right;
        (x2 - x1) * (y2 - y1)
    }
}

fn square(top_left: Point, side: i32) -> Rectangle {
    let Point { x, y } = top_left;
    Rectangle {
        top_left: Point { x, y },
        bottom_right: Point {
            x: x + side,
            y: y + side,
        },
    }
}
