#![allow(dead_code)]
/* Scoping Rules */

/* RAII: Resource Acquisition is Initialization */
// Box holds memory in heap

fn create_box() {
    let _box1 = Box::new(3i32);
    // _box1 is destroyed
}

fn part1() {
    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4_i32);
    }

    for _ in 1_u32..1_000 {
        create_box();
    }
    // no memory leakage will occur
}

// destructor can be implemented using the Drop trait

struct ToDrop;
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("Dropped");
    }
}

fn part2() {
    let _x = ToDrop;
    println!("ToDrop made");
}

/* Ownership and Moves */

// ownership is transferred (or a move occurs) when
// 1. assignment e.g. let x = y;
// 2. passing function arguments e.g. foo(x)

fn destroy_box(_c: Box<i32>) {
    println!("Destroying the box value");
    // owner scope ends, so memory is freed
}

fn part3() {
    let x = 5_u32; // allocated in stack
    let y = x; // no resources are moved
    println!("x is {} and y is {}", x, y);

    let a = Box::new(5_i32); // allocated in heap
    println!("a contains {}", a);

    let b = a; // move occurs
               // now both a and b point to the same data but b owns the data

    // !Error: println!("a contains {}", a);

    // function takes ownership of b
    destroy_box(b);
    // b can no longer be used
}

/* Mutability */
// mutability can be changed while transferring the data
fn part4() {
    let immutable_box = Box::new(5_u32);

    println!("immutable box contains {}", immutable_box);

    // *immutable_box = 4; // no allowed because immutable

    // the immutable box is now moved and mutable
    let mut mutable_box = immutable_box;

    println!("mutable box contains {}", mutable_box);

    *mutable_box = 4; // allowed

    println!("mutable box now contains {}", mutable_box);
}

/* Partial Moves */
// we move only a few parts during destructuring. As a result, the whole object cannot be used but the values destructured as ref can be used

fn part5() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved but `age` is referenced
    let Person { name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // cannot use complete person
    // println!("The peron struct is {:?}", person); // not allowed

    // person.age was not moved and can be used
    println!("Person's age from person struct is {}", person.age);

    // if age was on stack instead of heap, then there would not have been any need of ref
}

/* Borrowing */

// the compiler ensures that references always point to valid object

// take ownership and destroy
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is {}", borrowed_i32);
}

fn part6() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // borrow contents of box, ownership remains here
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;
        // eat_box_i32(boxed_i32); //not allowed because inner value is borrowed later in scope

        borrow_i32(_ref_to_i32);

        // ref_to_i32 no longer borrowed
    }
    eat_box_i32(boxed_i32);
}

// mutability in borrowing

#[derive(Clone, Copy)]
struct Book {
    // &'static means read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

fn new_edition(book: &mut Book) {
    book.year = 2023;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn part7() {
    let immutable_book = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    // mutable copy
    let mut mutable_book = immutable_book;

    borrow_book(&immutable_book);
    borrow_book(&mutable_book);

    new_edition(&mut mutable_book);

    // new_edition(&mut immutable_book); // not allowed
}

// aliasing

// data can be immutably borrowed multiple times but if it is mutably borrowed then it cannot be borrowed until mutable borrow occurred for the last time

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn part8() {
    let mut point = Point { x: 0, y: 0, z: 0 };
    let borrowed_point = &point;
    let another_borrow = &point;

    println!(
        "Point has location: ({} {} {})",
        point.x, borrowed_point.y, another_borrow.z
    ); // allowed

    // let mutable_borrow = &mut point; // not allowed because immutable borrows again used in next line
    println!(
        "Again, point has location: ({} {} {})",
        point.x, borrowed_point.y, another_borrow.z
    );

    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // let y = &point.y // not allowed because currently borrowed as mutable and mutable borrow will be used

    // cant event print point because print takes immutable reference

    println!(
        "Now point has location: ({} {} {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    let new_borrow = &point;
    println!(
        "Again, point has location: ({} {} {})",
        new_borrow.x, new_borrow.y, new_borrow.z
    );
}

// ref pattern
// to take reference of a field of struct/tuple

#[derive(Clone, Copy)]
struct MyPoint {
    x: i32,
    y: i32,
}

fn part9() {
    let c = 'Q';
    // ref on LHS is same as & on RHS
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 and ref_c2 are equal? {}", *ref_c1 == *ref_c2);

    let point = MyPoint { x: 0, y: 0 };

    let _copy_of_x = {
        let MyPoint {
            x: ref ref_to_x,
            y: _,
        } = point;
        // return reference to x
        *ref_to_x
    };

    let mut mutable_point = point;

    {
        let MyPoint {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;
        *mut_ref_to_y = 1;
    }

    println!("point is ({},{})", point.x, point.y);
    println!("mutable point is ({},{})", mutable_point.x, mutable_point.y);

    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("Tuple is {:?}", mutable_tuple);
}

/* Lifetimes */
// used to ensure that the borrows are valid

/*
fn example() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1` ends. ─────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
    //                                                     │
}   // Lifetime ends. ─────────────────────────────────────┘
*/

// explicit annotation
// compiler automatically annotates using rules of elision
// foo<'a> means foo has lifetime parameter 'a
// foo<'a, 'b> lifetime parameters 'a and 'b
//    here, lifetime of foo cannot exceed either 'a or 'b

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {x} and y is {y}");
}

fn failed_borrow<'a>() {
    let _x = 12;
    // let y:&'a i32 = &_x; // not allowed because x does not live long enough, _x has shorter life than y
}

fn part10() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);
    // the lifetime of four and nine must be grater than that of print_refs

    failed_borrow();
}

// lifetimes of functions
// ignoring elision, following constraints must be followed
//    references must have annotated lifetime
//    returned reference must have same lifetime as input or static
//    returning reference without input is banned

fn print_one<'a>(x: &'a i32) {
    println!("print_one: x is {x}");
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_multi: x is {x} and y is {y}");
}

// returning lifetime must be correct
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

// invalid
/* fn invalid_output<'a>() -> &'a String {
    &String::from("foo")
} */

fn part11() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

// lifetimes of methods
// similar annotation as function

struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("print: {}", self.0);
    }
}

fn part12() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}

// lifetime of struct
// again similar to function

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrow<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn part13() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrow { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

// lifetimes for traits
#[derive(Debug)]
struct MyBorrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for MyBorrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

fn part14() {
    let b: MyBorrowed = Default::default();
    println!("b is {:?}", b);
}

// bounds in lifetimes
// T: 'a => means all references in T must outlive lifetime 'a
// T: Trait+'a => means T must implement Trait and all references in T must outlive 'a

#[derive(std::fmt::Debug)]
struct MyRef<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has an unknown lifetime `'a`. `T` is bounded such that any *references* in `T` must outlive `'a`. Additionally, the lifetime of `Ref` may not exceed `'a`.

fn print<T>(t: T)
where
    T: std::fmt::Debug,
{
    println!("print: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T)
where
    T: std::fmt::Debug + 'a,
{
    println!("print ref: t is {:?}", t);
}

fn part15() {
    let x = 7;
    let ref_x = MyRef(&x);

    print_ref(&ref_x);
    print(ref_x);
}

// coercion

// two references are coerced into same lifetime
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// means 'a is at least as long as 'b
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn part16() {
    let first = 2; // longer lifetime

    {
        let second = 3; // shorter lifetime
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}

// static

//    reference lifetime
// the data pointed by reference lives for entire lifetime of the program. It can still be coerced to a shorter lifetime
// they are stored in read only memory

static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn part17() {
    {
        let static_string = "I am in read only memory";
        println!("static string: {}", static_string);

        // the data will remain but static_string will no longer be usable
    }

    {
        let lifetime_num = 9;

        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {} is still accessible", NUM);
}

// trait bound: owned data always contains a static lifetime but references need not

fn print_it(input: impl std::fmt::Debug + 'static) {
    println!("static value passed in is {:?}", input);
}

fn part18() {
    let i = 5;
    print_it(i); // valid

    // print_it(&i); invalid because its lifetime is defined by the scope of main, and after main `i` is dropped but borrowed by `print_it`
}

// elision
// compiler automatically adds lifetime parameters which are too obvious to be included

// rules:-
// 1. every reference is given separate named lifetime parameter by the compiler
// 2. if only one input is present, output reference also gets the same lifetime parameter
// 3. if `&self` or `&mut self` is a parameter, output has the same reference as them

fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn part19() {
    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
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
    // part15();
    // part16();
    // part17();
    // part18();
    part19();
}
