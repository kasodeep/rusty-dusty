use std::collections::BTreeMap;

pub fn demo(){
    println!("BTreeMaps");

    btreemap_basics();
    btreemap_advanced();
    btreemap_performance();
}

fn btreemap_basics() {
    let mut map: BTreeMap<i32, String> = BTreeMap::new();
    
    map.insert(1, String::from("one"));
    map.insert(2, String::from("two"));
    
    let _value = map.get(&1);
    map.remove(&1);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

fn btreemap_advanced() {
    let mut map = BTreeMap::new();
    
    map.insert(1, "One");
    map.insert(2, "Two");
    map.insert(3, "Three");
    
    for (key, value) in map.range(1..3) {
        println!("{}: {}", key, value);
    }
    
    if let Some((key, value)) = map.first_key_value() {
        println!("First: {}: {}", key, value);
    }
}

fn btreemap_performance() {
    let mut map = BTreeMap::new();
    
    for i in 0..1000 {
        map.insert(i, i.to_string());
    }
    
    let range_sum: i32 = map.range(100..200)
                           .map(|(k, _)| k)
                           .sum();
    println!("Range sum: {}\n", range_sum);
}
