use std::{collections::HashMap, hash::{Hash, Hasher}};

pub fn demo(){
    println!("Hashmaps");

    hashmap_basics();
    hashmap_advanced();
    hashmap_optimization();
}

fn hashmap_basics() {
    let mut map: HashMap<String, i32> = HashMap::new();
    
    map.insert(String::from("one"), 1);
    map.insert(String::from("two"), 2);
    
    let _value = map.get("one");
    map.remove("one");

    if map.contains_key("two") {
        println!("Contains key 'two'");
    }
}

fn hashmap_advanced() {
    let mut map = HashMap::new();
    
    map.entry(String::from("Key")).or_insert(0);
    *map.entry(String::from("Counter")).or_insert(0) += 1;
    
    if let Some(value) = map.get_mut("Key") {
        *value += 1;
    }
    
    for (key, value) in map.drain() {
        println!("{}: {}", key, value);
    }
    println!()
}

// demonstrates custom types in hashmaps
#[derive(Eq)]
struct CustomKey {
    id: u64,
    name: String,
}

impl Hash for CustomKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
    }
}

impl PartialEq for CustomKey {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

fn hashmap_optimization() {
    let mut map = HashMap::with_capacity(1000);
    map.reserve(1000);
    
    use std::collections::hash_map::RandomState;
    let custom_hasher = RandomState::new();
    let mut _custom_map: HashMap<String, String> = HashMap::with_hasher(custom_hasher);
    
    map.extend(vec![
        (String::from("a"), 1),
        (String::from("b"), 2),
    ]);
}
