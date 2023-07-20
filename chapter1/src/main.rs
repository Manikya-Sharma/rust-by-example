fn main() {
    // 1.1
    // println!("Hello, world!");
    // println!("I am a Rustacean!");
    
    // let x = 5 + /* 90 + */ 5;
    // println!("x = {x}");

    // formatted print
    // format! -> String
    // print!
    // println!
    // eprint! -> output in srderr
    // eprintln!

    println!("{0} this is {1}, {1} this is {0}", "Alice", "Bob");
    println!("{fruit} eaten by {creature}", fruit="mango", creature="monkey");

    println!("Base 10: {}", 12345);
    println!("Base 2: {:b}", 12345);
    println!("Base 8: {:o}", 12345);
    println!("Base 16: {:x}", 12345);

    println!("Right justified:-");
    println!("{number:>5}", number=1);

    println!("Padding of zeroes:-");
    println!("{number:0>5}", number=1);

    println!("padding of zeroes at right");
    println!("{number:0<5}", number=5);

    println!("|{:=^21}|", "heading");

    println!("{0} is {1:.3}", "pi", 3.1415926535);
    // escape { with {{

}
