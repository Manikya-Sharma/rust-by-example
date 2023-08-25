pub fn public_function() {
    println!("called rary's `public_function`");
}

fn private_function() {
    println!("called private function");
}

pub fn indirect_access() {
    println!("Called indeirect access which > ");
    private_function();
}
