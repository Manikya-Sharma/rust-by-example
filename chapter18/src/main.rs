#![allow(dead_code)]

/* Error handling in Rust */

// panic is useful only for tests and unrecoverable errors.
// Option is useful if lack of value is not an error
// if there are chances that things can go wrong and caller has to deal with problem, Result is used.

/* panic */

use std::fmt::format;

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("Don't drink too much sugar!");
    }
    println!("Some refreshing {} is all I need", beverage);
}

fn part1() {
    drink("water");
    drink("lemonade");
    drink("water");
}

/* abort and unwind */

fn drink2(beverage: &str) {
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("This is not your party. Run!");
        } else {
            println!("Spit it out!");
        }
    } else {
        println!("Some refreshing {} is all I need", beverage);
    }
}

fn part2() {
    drink2("water");
    drink2("lemonade");
}

#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party, run!");
}

fn drink3(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

fn part3() {
    drink3("water");
    drink3("lemonade");
}

/* Option and Unwrap */

fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary"),
        Some(inner) => println!("{}? How nice", inner),
        None => println!("No drink? Oh well."),
    }
}

fn drink4(drink: Option<&str>) {
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("No..")
    }

    println!("I love {}s!", inside);
}

fn part4() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink4(coffee);
    drink4(nothing);
}

// unpacking options with ?
// x? will evaluate to value if Some otherwise terminate function by returning None
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}

fn part5() {
    next_birthday(Some(13));
    next_birthday(None);
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

fn part6() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(16),
                number: 12344556,
            }),
        }),
    };
    assert_eq!(p.work_area_code(), Some(16));
}

/* Combinators */
// map

// heavy usage of match is tedious

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}
#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// this way map can be used
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm, I love {:?}", food),
        None => println!("It wasn't edible!"),
    }
}

fn part7() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

// and_then combinator
// although map allows chaining but it returns Option<T> which results in nested Option<Option<T>>
// therefore, we can use and_then

#[derive(Debug)]
enum Food2 {
    CordonBleu,
    Steak,
    Sushi,
}
#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

fn have_ingredients(food: Food2) -> Option<Food2> {
    match food {
        Food2::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: Food2) -> Option<Food2> {
    match food {
        Food2::CordonBleu => None,
        _ => Some(food),
    }
}

fn cookable_v1(food: Food2) -> Option<Food2> {
    match have_recipe(food) {
        None => None,
        Some(food) => have_ingredients(food),
    }
}

fn cookable_v3(food: Food2) -> Option<Food2> {
    have_recipe(food).and_then(have_ingredients)
}

fn cookable_v2(food: Food2) -> Option<Food2> {
    have_recipe(food).map(have_ingredients).flatten()
}

fn eat2(food: Food2, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("We can eat {:?} on {:?}", food, day),
        None => println!("We cant eat on {:?}", day),
    }
}

fn part8() {
    let (cordon_bleu, steak, sushi) = (Food2::CordonBleu, Food2::Steak, Food2::Sushi);

    eat2(cordon_bleu, Day::Monday);
    eat2(steak, Day::Tuesday);
    eat2(sushi, Day::Wednesday);
}

// unpacking options and defaults

// or() -> chainable, eager evaluation, keeping empty value intact
#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}

fn part9() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("First available fruit: {:?}", first_available_fruit);
}

// or_else() -> chainable, lazy evaluation, empty value intact

fn part10() {
    let apple = Some(Fruit::Apple);
    let no_fruit: Option<Fruit> = None;
    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as a fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };
    let first_available_fruit = no_fruit.or_else(get_kiwi_as_fallback).or_else(get_lemon_as_fallback);

    println!("first_available_fruit: {:?}", first_available_fruit);
}

// get_or_insert() -> eager evaluation, modify empty value in place
// it ensures Option will give a value

fn part11() {
    let mut my_fruit:Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);

    println!("first_available_fruit: {:?}", first_available_fruit);
    println!("my_fruit: {:?}", my_fruit);
}

// get_or_insert_with() -> evaluates lazy, modify in place

fn part12() {
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = ||{
        println!("Providing lemon as a fallback");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);

    // if Option has a value, then the closure is not called
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
    part12();
}
