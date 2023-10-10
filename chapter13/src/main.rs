#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are not on linux");
}

fn main() {
    println!("Hello, world!");
    are_you_on_linux();
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Certainly I am on linux");
    } else {
        println!("Certainly I am not on linux");
    }
}
