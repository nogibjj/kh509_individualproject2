use std::sync::RwLock;


lazy_static::lazy_static! {
    static ref FRUITS: RwLock<Vec<&'static str>> = RwLock::new(Vec::new());
}


pub fn add_fruit(fruit: &'static str) {
    let mut fruits = FRUITS.write().unwrap(); // Use write lock for exclusive access
    fruits.push(fruit);
}

pub fn remove_fruit(fruit: &'static str) {
    let mut fruits = FRUITS.write().unwrap(); // Use write lock for exclusive access
    fruits.retain(|&x| x != fruit);
}


pub fn get_fruits() -> Vec<String> {
    let fruits = FRUITS.read().unwrap();
    fruits.clone()
}
