use std::{fmt::format, hash::Hash};

pub struct HashTable<U, T: PartialEq> {
    pub items: Vec<HashItem<U, T>>,
}

pub struct HashItem<U, T> {
    hash: U,
    value: T,
}

impl<U: Copy, T: Copy> HashItem<U, T> {
    pub fn new(key: &U, value: &T) -> Self {
        HashItem {
            hash: *key,
            value: *value,
        }
    }
}

impl<U: Copy, T: PartialEq + Copy> HashTable<U, T> {
    pub fn new() -> Self {
        return HashTable { items: Vec::new() };
    }

    pub fn add(&mut self, key: &U, value: &T) {
        let hash_item = HashItem::new(key, value);
        self.items.push(hash_item);
    }

    pub fn is_in(&self, value: &T) -> bool {
        for (_, v) in self.items.iter().enumerate() {
            if *value == v.value {
                return true;
            }
        }
        return false;
    }

}

pub fn find_pair_given_sum(arr: Vec<u8>, target: &u8) -> String {
    let mut map = HashTable::new();
    for (i, v) in arr.iter().enumerate() {
        map.add(&i, v);
        if map.is_in(&(target - v)) {
            return format!("is in ({},{})", v, target - v);
        }
    }
    return String::from("is not in");
}

pub fn algorithms() {
    let arr = [1, 2, 4, 3, 7, 6, 8, 9];
    let target = 10;
    println!("{}", find_pair_given_sum(arr.to_vec(), &target));
}
