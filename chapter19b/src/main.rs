/* Vectors */
// vector can be resized

// if length of vector surpasses its capacity, it needs to be reallocated with larger capacity

fn main() {
    let collected_iterator: Vec<i32> = (0..10).collect();

    println!("Collected (0..10) into {:?}", collected_iterator);

    let mut xs = vec![1_i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    println!("After pushing 4");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // collected_iterator.push(0); // not allowed

    // we can get, find element in similar manner
    // vec allowed indexing but panic beyond size

    // iterator can be used
    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("We have {} at position {}", x, i);
    }

    // we can also modify using iter_mut
    for x in xs.iter_mut() {
        *x += 3;
    }
    println!("Updated vector: {:?}", xs);
}
