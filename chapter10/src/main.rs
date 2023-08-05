#![allow(dead_code)]
// modules

// only public items can be accessed by other modules

use crate::{my_mod::nested::function, my_new_mod::ClosedBox};

mod my_mod {
    fn private_function() {
        println!("private function called");
    }

    pub fn public_function() {
        println!("public function called");
    }
    pub fn indirect_access() {
        println!("Get ready for indirectly accessed call");
        private_function();
    }

    // nested module
    pub mod nested {
        pub fn function() {
            println!("public function from nested module called");
        }
        #[allow(dead_code)]
        fn private_function() {
            println!("called my_mod::nested::private_function()");
        }

        // we can specify if function is public only at a given path
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("Called public function in my_mod::nested");
            println!("Another call:-");
            public_yet_private_function();
        }

        // pub(self) is same as private (default)
        pub(self) fn public_yet_private_function() {
            println!("I am public function but only in my own module");
        }

        pub(super) fn public_function_for_super() {
            println!("called my_mod::nested::public_function_for_super()");
        }
    }

    pub fn call_public_functions() {
        println!("Calling some public functions");
        nested::public_function_in_my_mod();
        println!(">");
        nested::public_function_for_super();
    }

    pub(crate) fn public_function_for_crate() {
        println!("called function public for crate only");
    }
}

fn part1() {
    my_mod::call_public_functions();
    my_mod::indirect_access();
    my_mod::public_function();
    my_mod::public_function_for_crate();
    my_mod::nested::function();
    // my_mod::nested::public_yet_private_function();
}

// structs visibility
// we can also choose if field to be public or private

mod my_new_mod {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}

fn part2() {
    let open_box = my_new_mod::OpenBox {
        contents: "public info",
    };
    println!("OpenBox has {}", open_box.contents);

    // cant initialize closed box directly from struct
    // also cannot access
    let _closed_box = ClosedBox::new("private data");
}

// use declaration
// we can use `as` keyword to bind imports with a different name

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("I am a deeply nested function!");
        }
    }
}

use deeply::nested::function as another_function;

fn part3() {
    another_function();
    println!("Entering block");
    {
        use crate::deeply::nested::function;
        // use bindings have local scope
        function();
    }
    println!("Left block");
    function(); // different function is called
}

// super and self -> to prevent hardcoding of paths

fn cool_function() {
    println!("A cool function was called");
}

mod cool {
    pub fn cool_function() {
        println!("Called cool::cool_function()");
        println!("Calling my parent's function");
        super::cool_function();
    }
}

fn part4() {
    self::cool_function();
    cool::cool_function();
}

fn main() {
    // part1();
    // part2();
    // part3();
    part4();
}
