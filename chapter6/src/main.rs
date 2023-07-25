// conversion

// From and Into traits

/* use std::convert::From;

use std::fmt;

struct Number {
    value: i32,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{StructuralNumber-{}}}", self.value)
    }
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
// note that Into is automatically implemented by primitive type if From is implemented by desired type.
// One will have to specially typecast to get value

fn main() {
    let number = 5i32;
    let my_number = Number::from(number);
    println!("My number: {}", my_number);

    let int = 5;
    let num: Number = int.into();
    println!("Now my number is {}", num);
}
*/

// TryFrom and TryInto

// they are fallable version of From and Into which return Result

/* use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
*/

// To and From Strings

// we can implement ToString trait or instead Display trait automatically adds ToString trait

use std::fmt::{self, Formatter};

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 5 };
    println!("{}", circle.to_string());

    // to get number from a string

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    println!("{}, {}", parsed, turbo_parsed);
}
