/* HashMap */
// stores values by keys
// growable and shrinkable

use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => {
            "We're sorry, the call cannot be completed as dialed.
            Please hang up and try again."
        }
        "645-7689" => {
            "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?"
        }
        _ => "Hi! Who is this again?",
    }
}

fn part1() {
    let mut contacts = HashMap::with_capacity(4);

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel {}", call(number)),
        _ => println!("Don't have Daniel's number"),
    }

    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&"Ashley");

    // `HashMap::iter()` returns an iterator that yields
    // (&'a key, &'a value) pairs in arbitrary order.
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }

    // for a key to be valid, it must implement Eq and Hash traits
    // e.g. bool, int, uint, String, &str
    // f32 and f64 don't implement Hash due to precision errors
    // #define[PartialEq, Hash] can be used
}

#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_login<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting login ... ");

    let login = Account { username, password };

    match accounts.get(&login) {
        Some(account_info) => {
            println!("Successful Login");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        }
        _ => println!("Login failed"),
    }
}

fn part2() {
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    try_login(&accounts, "j.everyman", "psasword123");

    try_login(&accounts, "j.everyman", "password123");
}

/* HashSet */
// It is like a Hash without value
// It is like a set which does not contain any duplicate values
// 4 operations can be performed:-
// union
// difference
// intersection
// symmetric_difference

use std::collections::HashSet;


fn part3() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    println!("Did it insert 4 again?: {}", b.insert(4));

    println!("A: {:?}", a);
    println!("B: {:?}", b);

    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    println!("intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

    println!("difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    println!("symmetric_difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());

}

fn main() {
    // part1();
    // part2();
    part3();
}
