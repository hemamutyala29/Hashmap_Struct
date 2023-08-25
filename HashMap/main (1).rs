use std::collections::HashMap;
use std::cmp::Ord;

// Define the SortByKey trait
trait SortByKey<K: Ord, V> {
    fn sort_by_key(&self) -> Vec<(&K, &V)>;
}

// Implement the SortByKey trait for HashMap
impl<K: Ord, V> SortByKey<K, V> for HashMap<K, V> {
    fn sort_by_key(&self) -> Vec<(&K, &V)> {
        let mut sorted_pairs: Vec<_> = self.iter().collect();
        sorted_pairs.sort_by_key(|(key, _)| *key);
        sorted_pairs
    }
}

fn main() {
    // Create a new HashMap instance
    let mut my_map: HashMap<i32, &str> = HashMap::new();

    // Add key-value pairs to the map
    my_map.insert(3, "apple");
    my_map.insert(1, "banana");
    my_map.insert(2, "orange");
    my_map.insert(5, "grape");
    my_map.insert(4, "cherry");

    println!("Original map: {:?}", my_map);

    // Get and display sorted key-value pairs
    let sorted_pairs = my_map.sort_by_key();
    println!("Sorted map: {:?}", sorted_pairs);
}
