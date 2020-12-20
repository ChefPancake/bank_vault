#![feature(option_unwrap_none)]
pub mod bank_vault {
    use uuid::Uuid;
    use std::collections::HashMap;

    #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
    pub struct VaultKey {
        key: Uuid,
    }

    impl VaultKey {
        pub fn new() -> VaultKey {
            VaultKey {key: Uuid::new_v4()}
        }
        pub fn zero() -> VaultKey {
            VaultKey {key: Uuid::nil()}
        }
    }
    
    pub struct Vault<T> {
        items: HashMap<VaultKey, T>,
    }

    impl<T> Vault<T> {
        pub fn new() -> Vault<T>{
            let map = HashMap::new();
            Vault {items: map}
        }

        pub fn remove(&mut self, key: &VaultKey) -> Option<T>{
            self.items.remove(key)
        }

        pub fn add(&mut self, to_add: T) -> VaultKey {
            let key = VaultKey::new();
            self.items.insert(key, to_add);
            key
        }

        pub fn has_item(&self, key: &VaultKey) -> bool {
            self.items.contains_key(key)
        }

        pub fn add_with_key(&mut self, to_add: T, key: &VaultKey) -> bool {
            self.items.insert(*key, to_add).is_none()
        }

        pub fn update_item<F>(&mut self, key: &VaultKey, mut operation: F) -> bool
                where F: FnMut(&mut T) -> () {
            self.remove(key).map(|mut i| {
                operation(&mut i);
                self.add_with_key(i, key)
            }).is_some()
        }

        pub fn clear(&mut self) {
            self.items.clear()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bank_vault::*;

    #[test]
    fn add_has_item() {
        let mut vault = Vault::new();
        let to_add = "stuff";
        let key = vault.add(to_add);
        let has_item = vault.has_item(&key);
        assert_eq!(true, has_item);
    }

    #[test]
    fn has_item_wrong_key() {
        let mut vault = Vault::new();
        let to_add = "garbage";
        vault.add(to_add);
        let wrong_key = VaultKey::new();
        let has_item = vault.has_item(&wrong_key);
        assert_eq!(false, has_item);
    }

    #[test]
    fn add_duplicates_unique_keys() {
        let mut vault = Vault::new();
        let to_add = 1.0;
        let key_1 = vault.add(to_add);
        let key_2 = vault.add(to_add);
        assert_ne!(key_1, key_2);
    }

    #[test]
    fn add_with_key_has_item() {
        let mut vault = Vault::new();
        let to_add = 1.0;
        let key = VaultKey::new();
        let added = vault.add_with_key(to_add, &key);
        let has_item = vault.has_item(&key);
        assert_eq!(true, added);
        assert_eq!(true, has_item);
    }

    #[test]
    fn add_with_key_duplicate_fails() {
        let mut vault = Vault::new();
        let to_add = 1.0;
        let key = VaultKey::new();
        let added_1 = vault.add_with_key(to_add, &key);
        let added_2 = vault.add_with_key(to_add, &key);
        assert_eq!(true, added_1);
        assert_eq!(false, added_2);
    }

    #[test]
    fn add_remove() {
        let mut vault = Vault::new();
        let to_add = 1;
        let key = vault.add(to_add);
        let retrieved = vault.remove(&key).unwrap();
        assert_eq!(to_add, retrieved);
    }

    #[test]
    fn remove_before_add() {
        let mut vault = Vault::<i32>::new();
        let key = VaultKey::new();
        let retrieved = vault.remove(&key);
        assert_eq!(None, retrieved);
    }

    #[test]
    fn add_update() {
        let mut vault = Vault::new();
        let to_add = 1.0;
        let key = vault.add(to_add);
        let double = |i:&mut f64| *i = *i * 2.0;
        let expected = 2.0;
        let updated = vault.update_item(&key, double);
        let retrieved = vault.remove(&key).unwrap();
        assert_eq!(true, updated);
        assert_eq!(expected, retrieved);
    }

    #[test]
    fn add_remove_twice() {
        let mut vault = Vault::new();
        let to_add = 3;
        let key = vault.add(to_add);
        vault.remove(&key);
        let second = vault.remove(&key);
        assert_eq!(None, second);
    }

    #[test] 
    fn add_clear_doesnt_have_item() {
        let mut vault = Vault::new();
        let to_add = "thing";
        let key = vault.add(to_add);
        vault.clear();
        let has_item = vault.has_item(&key);
        assert_eq!(false, has_item);
    }

    #[test]
    fn add_remove_doesnt_have_item() {
        let mut vault = Vault::new();
        let to_add = "an item";
        let key = vault.add(to_add);
        vault.remove(&key);
        let has_item = vault.has_item(&key);
        assert_eq!(false, has_item);
    }
}
