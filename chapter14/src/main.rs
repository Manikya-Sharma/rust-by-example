// generics

use std::fmt::Display;

struct A;

struct Single(A);

struct SingleGen<T>(T);

// generic structs
fn part1() {
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('c');
}

// generic functions
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn gen_spec_generic<T>(_s: SGen<T>) {}

fn part2() {
    reg_fn(S(A));
    gen_spec_i32(SGen(12));

    gen_spec_generic::<char>(SGen('a'));
    gen_spec_generic(SGen('a'))
}

// generic implementation
struct St;
struct GenericVal<T>(T);

// specific implementation of generic struct
impl GenericVal<f32> {}
impl GenericVal<St> {}

// generic implementation of generic struct
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn part3() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{} {}", x.value(), y.value());
}

// generic traits

struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn part4() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
    // deallocates empty and null both
    // null; // error
}

// bounds
fn printer<T: Display>(t: T) {
    println!("{}", t);
}

struct Str<T: Display>(T);

// we can use methods if we know that particular trait is implemented

use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn part5() {
    // let s = Str(vec![1]); // not allowed because vector does not implement Display trait
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };
    print_debug(&rectangle);
    println!("Area = {}", area(&rectangle));
}

// empty bounds

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn part6() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
}

// multiple bounds
// applied using `+`

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

fn part7() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];
    compare_prints(&string);
    // will not work because Display not implemented by array
    // compare_prints(&array);
    compare_types(&array, &vec);
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
