use std::fmt;

fn main() {
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
