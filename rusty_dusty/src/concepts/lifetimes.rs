pub fn demo() {
    println!("\n1. Basic Reference Example:");
    {
        let original = String::from("hello");
        let reference = &original;
        println!("Original: {}", original);
        println!("Reference: {}", reference);
    } 

    println!("\n2. Function with Lifetime:");
    {
        let string1 = String::from("short string");
        let string2 = String::from("longer string");
        
        // The compiler knows these references must have the same lifetime.
        let result = longest(&string1, &string2);
        println!("Longest string is: {}", result);
    }

    println!("\n3. Struct with Lifetime:");
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = get_first_sentence(&novel);
        let excerpt = ImportantExcerpt {
            part: first_sentence,
        };
        println!("Excerpt: {}", excerpt.part);
        println!("Importance level: {}", excerpt.level());
    }

    println!("\n4. Static Lifetime Example:");
    {
        let static_str: &'static str = "I have a static lifetime";
        print_static(static_str);
    }
}


// Here x and y must live as long as the returned reference.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// The 'a annotation means the reference in part can't outlive the struct.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part<'b>(&self, announcement: &'b str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn print_static(s: &'static str) {
    println!("Static string: {}", s);
}

// Helper function: Get first sentence of a string.
fn get_first_sentence(text: &str) -> &str {
    match text.find('.') {
        Some(idx) => &text[..=idx],
        None => text,
    }
}