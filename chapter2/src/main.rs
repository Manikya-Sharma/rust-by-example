use std::fmt;
//
fn part1() {
    // Simple Operators
    println!("1i32 - 2 = {}", 1i32 - 2);
    println!("1 - 2 = {}", 1 - 2);
    // println!("1u32 - 2 = {}", 1u32-2); // overflow
    println!("1e4={}", 1e4);

    // tuples
    let tuple = (1u8, 2u16, 3u32);
    println!("tuple first value: {}", tuple.0);
    println!("tuple second value: {}", tuple.1);

    let tuple_of_tuples = ((1, 2, 3), (4, 5, 6));
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    // tuples beyond size of 12 cannot be printed
    let t2 = (1, 2);
    println!("Reversed pair = {:?}", reverse(t2));

    let my_matrix = Matrix(1.1, 2.2, 3.3, 2.2);
    println!("{}", my_matrix);
    println!("Transpose:\n{}", transpose(&my_matrix));
}

fn reverse(pair: (i32, i32)) -> (i32, i32) {
    (pair.1, pair.0)
}

struct Matrix(f32, f32, f32, f32);
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}
fn transpose(matrix: &Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

// Arrays and slices
use std::mem;
fn part2() {
    // a slice is kind of array whose length is not known at compile time
    let array /*: [i32; 5]*/ = [1, 2, 3, 4, 5];
    let array2 = [0; 500]; // 500 elements with value 0
    println!("array occupies {} bytes", mem::size_of_val(&array));
    println!("array2 occupies {} bytes", mem::size_of_val(&array2));

    // how we can take slices
    println!("Whole array as slice:-");
    analyze_slice(&array);

    println!("Section of array as slice:-");
    analyze_slice(&array[1..4]);

    // empty array: []

    // safe accession of data -> get method
    for i in 0..array.len() + 1 {
        match array.get(i) {
            Some(val) => println!("{}: {}", i, val),
            None => println!("Some non existing element was asked!"),
        }
    }
}

fn analyze_slice(slice: &[i32]) {
    println!("First element: {}", slice[0]);
    println!("Slice has {} elements", slice.len());
}

fn main() {
    part1();
    part2();
}
