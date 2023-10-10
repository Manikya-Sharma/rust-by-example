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

// where clause
// another way to apply bounds

// example

trait MyTrait<A, D> {}

trait TraitB {}
trait TraitC {}
trait TraitE {}
trait TraitF {}

struct MyType;

impl<A, D> MyTrait<A, D> for MyType
where
    A: TraitB + TraitC,
    D: TraitE + TraitF,
{
}

// another example
trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn part8() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}

// new type idiom
// a way to implement guarantees in types
struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
        // .0 will give the primitive type
    }
}
impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn part9() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old Enough: {}", old_enough(&age));
    println!("Old Enough: {}", old_enough(&age_days.to_years()));
}

/* Associated Items */
// problem:-

/* struct Container(i32, i32);
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // explicit requirement of A and B
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

// C contains A and B but still we need to express them

fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

fn part10() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("last number: {}", container.last());

    println!("Difference: {}", difference(&container));
} */

// associated types will move inner types locally into trait as output types

struct Container(i32, i32);

trait Contains {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;

    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        &self.0 == number_1 && &self.1 == number_2
    }

    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn part10() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("last number: {}", container.last());

    println!("Difference: {}", difference(&container));
}

/* Phantom type parameters */
// type parameter that doesn't show up at run time but is checked only at compile time

use std::marker::PhantomData;

// generic over A with hidden(phantom) parameter B
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

// no storage is allocated for B and cannot be used for computations

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

fn part11() {
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // we cant compare _tuple1 and _tuple2 or _struct1 and _struct 2 at compile time and it will show error;
    // _tuple1 == _tuple2;
}

// example of phantom data

use std::ops::Add;

#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn part12() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // now there is type safety in such unit conversion
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
