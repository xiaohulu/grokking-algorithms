use std::collections::HashMap;

pub fn new_hash_table() -> HashMap<&'static str, f64> {
    let mut map = HashMap::new();
    map.insert("apple", 0.67);
    map.insert("milk", 1.49);
    map.insert("avocado", 1.49);
    map
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_success() {
        println!("{:?}", new_hash_table());
    }

}