use std::sync::RwLock;

lazy_static::lazy_static! {
    static ref FRUITS: RwLock<Vec<String>> = {
        let fruits: Vec<String> = vec![
            "banana".to_string(),
            "apple".to_string(),
        ];
        RwLock::new(fruits)
    };
}

pub fn add_fruit(fruit: &str) {
    let mut fruits = FRUITS.write().unwrap(); // Use write lock for exclusive access
    fruits.push(fruit.to_string());
}

pub fn remove_fruit(fruit: &str) {
    let mut fruits = FRUITS.write().unwrap(); // Use write lock for exclusive access
    fruits.retain(|x| x != fruit);
}

pub fn get_fruits() -> Vec<String> {
    let fruits = FRUITS.read().unwrap();
    fruits.clone()
}
