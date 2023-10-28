/* There are two types of strings: String and &str */

// String: Vec<u8> i.e. vector of bytes guaranteed to be valued utf-8
// it is heap-allocated, growable, not-null terminated

// &str: slice of u8 i.e. &[u8] pointing to valid utf-8. It can be used to view in a String

fn part1() {
    // reference to string in Read Only Memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";

    // no new string made while traversing
    println!("Words in reverse:-");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // copy characters in a Vec
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // creating a String
    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }

    // trimmed string is a slice to original string and no new allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");
    println!("Alice says {}", alice);
    println!("Bob says {}", bob);
}

// generally, special characters are escaped with a backslash

fn part2() {
    let byte_escape = "I'm writing \x52\x75\x73\x74";
    println!("What are you doing\x3F (\\x3F means?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    let raw_string = r"No need for escapes here, they wont work: \x3F \u{211D}";
    println!("{}", raw_string);

    let quotes = r#"An then I said:"There is no escape!""#;
    println!("Quotes: {}", quotes);

    // you can use 65535 #s delimiters
    let longer_delimiter = r###"A string with "# in it. And even "##"###;
    println!("Longer delimiter: {}", longer_delimiter);

    // str::from method can be used to convert byte-string into &str

}

fn main() {
    // part1();
    part2();
}
