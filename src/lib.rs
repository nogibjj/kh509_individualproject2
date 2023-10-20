use rand::Rng;
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

pub fn get_fruits(count: u32) -> Vec<String> {
    let fruits = FRUITS.read().unwrap();
    let mut result = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let fruit_idx = rng.gen_range(0..fruits.len());
        result.push(fruits[fruit_idx].to_string());
    }
    result
}

pub fn add_fruit(fruit: &str) {
    let mut fruits = FRUITS.write().unwrap();
    fruits.push(fruit.to_string());
}

pub fn remove_fruit(fruit: &str) {
    let mut fruits = FRUITS.write().unwrap();
    fruits.retain(|x| x != fruit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fruits() {
        let fruits = get_fruits(5);
        assert_eq!(fruits.len(), 5);
    }
}
