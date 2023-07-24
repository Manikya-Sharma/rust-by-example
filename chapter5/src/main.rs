// Types

// casting

/* fn main() {
    // rust does not implicitly cast
    let decimal = 654.432__f32;

    let integer = decimal as u8;
    // floor for decimal
    // takes max value in case of overflow
    println!("Decimal: {}", decimal);
    println!("Integer: {}", integer);

    unsafe {
        println!("300.0 as u8 = {}", 300.0_f32.to_int_unchecked::<u8>());
    }
} */

// aliasing

type Nanosecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    let nanoseconds: Nanosecond = 5 as U64;
    let inches: Inch = 7 as U64;
    // alias is just another name and not a new type
    println!(
        "{} nanoseconds + {} inches = {} -- no type safety",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
