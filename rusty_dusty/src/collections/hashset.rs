use std::collections::HashSet;

pub fn demo(){
    println!("Hashsets");

    hashset_basics();
    hashset_operations();
    hashset_optimization();
}

fn hashset_basics() {
    let mut set: HashSet<i32> = HashSet::new();
    
    set.insert(1);
    set.insert(2);
    
    if set.contains(&1) {
        println!("Contains 1");
    }
    set.remove(&1);
}

fn hashset_operations() {
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    
    set1.insert(1);
    set1.insert(2);
    set2.insert(2);
    set2.insert(3);
    
    let _union: HashSet<_> = set1.union(&set2).cloned().collect();
    let _intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    let _difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    let _sym_difference: HashSet<_> = set1.symmetric_difference(&set2).cloned().collect();
}

fn hashset_optimization() {
    let mut set = HashSet::with_capacity(1000);
    set.reserve(1000);
    
    set.extend(vec![1, 2, 3, 4, 5]);
}