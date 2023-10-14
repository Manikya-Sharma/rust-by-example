#![allow(dead_code, unused_variables)]
/* Traits */

// a collection of methods on an unknown type Self

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // we can also provide default definition
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn sheer(&mut self) {
        if self.is_naked() {
            println!("{} is already naked", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn part1() {
    // type annotation compulsory in such method
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.sheer();
    dolly.talk();
}

// derive
// compiler can provide basic implementation of trait via [derive].
// for more complex implementation, they can be derived manually

// derivable traits:-
// 1. Eq, PartialEq, Ord, PartialOrd
// 2. Clone
// 3. Copy
// 4. Hash
// 5. Default (to create empty instance)
// 6. Debug

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

fn part2() {
    let one_second = Seconds(1);
    // println!("{:?}", one_second); // not allowed
    // let _is_is_true = (one_second == one_second);// not allowed

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("One foot is {} than one meter", cmp);
}

/*  returning traits with dyn */
// return type of a function must have concrete type to ensure the amount of memory which will be returned
// Instead of returning trait, a Box which contains Animal can be returned, because it is reference to memory in heap and has known size
// we need to explicitly tell by using dyn keyword e.g. Box<dyn Animal>

struct MySheep {}
struct MyCow {}
trait MyAnimal {
    fn noise(&self) -> &'static str;
}

impl MyAnimal for MySheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

impl MyAnimal for MyCow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn MyAnimal> {
    if random_number < 0.5 {
        Box::new(MySheep {})
    } else {
        Box::new(MyCow {})
    }
}

fn part3() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You have chosen an animal which says {}", animal.noise());
}

/* Operator Overloading */
// operators are just syntactic sugar for method calls
use std::ops;

struct Foo;
struct Bar;
#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, rhs: Bar) -> Self::Output {
        println!("Foo.add(Bar) was called");
        FooBar
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, rhs: Foo) -> Self::Output {
        println!("Bar.add(Foo) was called");
        BarFoo
    }
}

fn part4() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

// Drop trait -> like Destructor
// Box, Vec, String, File and Process implement Drop trait

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn part5() {
    let _a = Droppable { name: "a" };
    {
        let _b = Droppable { name: "b" };
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting Block B");
        }
        println!("Just exited Block B");

        println!("Exiting Block A");
    }
    println!("Just exited Block  A");

    drop(_a); // manually drop

    println!("End of main");
}

/* Iterators */
// only definition for next method is needed
// for construct converts some collections using .into_iter() method

struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32; // referred to as Self::Item
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;
        Some(current)
        // Fibonacci series in infinite and never returns None
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {
        current: 0,
        next: 1,
    }
}

fn part6() {
    let mut sequence = 0..3;
    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}

/* impl keyword */
// it can be used as:-
// 1. argument type
// 2. return type

// as an argument type

// R is a generic type which implements BufRead e.g. BufReader<File> or [u8]
fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            line.map(|line| {
                line.split(',')
                    .map(|entry| String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()
}
// header can also be written as
// fn parse_csv(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>>;

// as a return type
use std::iter;
use std::vec::IntoIter;

fn combine_vectors_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// same function with impl return type
fn combine_vectors(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn part7() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vectors(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");
}

// some Rust types can't be written e.g. type of each closure is different

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| x + y;
    closure
}

fn part8() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}

// we can return Iterator that implement map and filter

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}

fn part9() {
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}

/* Clone */

#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn part10() {
    let unit = Unit;
    let copied_unit = unit; // copied by default because no resources to move

    println!("original: {:?}", unit);
    println!("copied: {:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair; // not copied
    println!("moved: {:?}", moved_pair);

    // println!("{:?}", pair); // not allowed

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    println!("clone: {:?}", cloned_pair);
}

// Supertraits
// loosely related to inheritance

trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait ITStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn it_student_greeting(student: &dyn ITStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct Me {
    name: &'static str,
    university: &'static str,
    lang: &'static str,
    username: &'static str,
}

impl ITStudent for Me {
    fn git_username(&self) -> String {
        self.username.to_string()
    }
}

impl Student for Me {
    fn university(&self) -> String {
        self.university.to_string()
    }
}

impl Programmer for Me {
    fn fav_language(&self) -> String {
        self.lang.to_string()
    }
}

impl Person for Me {
    fn name(&self) -> String {
        self.name.to_string()
    }
}

fn part11() {
    let me = Me {
        name: "manikya",
        lang: "rust",
        university: "dtu",
        username: "Manikya-Sharma",
    };
    println!("{}", it_student_greeting(&me));
}

/* disambiguating overlapping traits */

// if two different traits implemented by a type have same method name
// each method has its own implementation block, therefore, no ambiguity occurs.
// To call such method, we need Fully Qualified Syntax

trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}
impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn part12() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    let username = <Form as UsernameWidget>::get(&form);
    let age = <Form as AgeWidget>::get(&form);

    assert_eq!("rustacean".to_owned(), username);
    assert_eq!(28, age);

}


fn main() {
    // part1();
    // part2();
    // part3();
    // part4();
    // part5();
    // part6();
    // part8();
    // part9();
    // part10();
    // part11();
    part12();
}
