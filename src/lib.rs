#![feature(option_unwrap_none)]
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct VaultKey {
    key: Uuid,
}

impl VaultKey {
    /// Creates a new unique VaultKey
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use bank_vault::VaultKey;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let key_1 = VaultKey::new();
    /// let key_2 = VaultKey::new();
    /// 
    /// assert_ne!(key_1, key_2);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn new() -> VaultKey {
        VaultKey {key: Uuid::new_v4()}
    }

    /// Creates a default, zero-valued VaultKey
    /// # Example
    /// ```rust
    /// # use std::error::Error;
    /// # use bank_vault::VaultKey;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let key_1 = VaultKey::zero();
    /// let key_2 = VaultKey::zero();
    /// 
    /// assert_eq!(key_1, key_2); 
    /// #     Ok(())
    /// # }
    /// ```
    pub fn zero() -> VaultKey {
        VaultKey {key: Uuid::nil()}
    }
}

pub struct Vault<T> {
    items: Mutex<HashMap<VaultKey, T>>
}

impl<T> Vault<T> {
    /// Creates a new, empty Vault instance.
    /// # Example
    /// 
    /// ```rust
    /// # use std::error::Error;
    /// # use bank_vault::Vault;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let vault = Vault::<i32>::new();
    /// #     Ok(())
    /// # }
    /// ```
    pub fn new() -> Vault<T>{
        let map = HashMap::new();
        let mutex = Mutex::from(map);
        Vault {items: mutex}
    }

    /// Adds an object to the vault and returns a key.
    /// # Example
    /// 
    /// ```rust
    /// # use std::error::Error;
    /// # use bank_vault::Vault;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let vault = Vault::<i32>::new();
    /// 
    /// let key = vault.add(1);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn add(&self, to_add: T) -> VaultKey {
        let mut unlocked = self.items.try_lock().unwrap();
        let key = VaultKey::new();
        unlocked.insert(key, to_add);
        key
    }

    /// Removes and returns the stored object with a matching key, if it exists, otherwise returns None.
    /// # Example
    /// 
    /// ```rust
    /// # use std::error::Error;
    /// # use bank_vault::Vault;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let vault = Vault::<i32>::new();
    /// let key = vault.add(1);
    /// 
    /// let item = vault.remove(&key);
    /// assert_eq!(Some(1), item);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn remove(&self, key: &VaultKey) -> Option<T>{
        self.items.try_lock().unwrap().remove(key)
    }

    /// Returns true if there exists an item in the vault with the provided key, otherwise returns false.
    /// # Example
    /// 
    /// ```rust
    /// # use std::error::Error;
    /// # use bank_vault::Vault;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let vault = Vault::<i32>::new();
    /// let key = vault.add(1);
    /// 
    /// let has_item = vault.has_item(&key);
    /// assert_eq!(true, has_item);
    /// #     Ok(())
    /// # }
    /// ```    
    pub fn has_item(&self, key: &VaultKey) -> bool {
        self.items.try_lock().unwrap().contains_key(key)
    }

    /// Adds an item to the vault with the specified key. If the key already is in use, the item is not added and this returns false. If the key is not already in use, the item is added and returns true.
    /// # Example
    /// 
    /// ```rust
    /// # use std::error::Error;
    /// # use bank_vault::{Vault, VaultKey};
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let vault = Vault::<i32>::new();
    /// let key = VaultKey::new();
    /// 
    /// let is_true = vault.add_with_key(1, &key);
    /// assert_eq!(true, is_true);
    /// let is_false = vault.add_with_key(2, &key);
    /// assert_eq!(false, is_false);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn add_with_key(&self, to_add: T, key: &VaultKey) -> bool {
        self.items.try_lock().unwrap().insert(*key, to_add).is_none()
    }

    /// Updates an item in the vault with the specified key by applying the operation to it. Returns false if an item with the key is not found, otherwise returns true.
    /// # Example
    /// 
    /// ```rust
    /// # use std::error::Error;
    /// # use bank_vault::{Vault, VaultKey};
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let vault = Vault::<i32>::new();
    /// let key = vault.add(1);
    /// 
    /// let double_me = |i:i32| i * 2;
    /// 
    /// let updated = vault.update_item(&key, double_me);
    /// assert_eq!(true, updated);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn update_item<F>(&self, key: &VaultKey, mut operation: F) -> bool
            where F: FnMut(T) -> T {
        self.remove(key).map(|i| {
            let updated = operation(i);
            self.add_with_key(updated, key)
        }).is_some()
    }

    /// Clears the contents of the vault.
    /// # Example
    /// 
    /// ```rust
    /// # use std::error::Error;
    /// # use bank_vault::{Vault, VaultKey};
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let vault = Vault::<i32>::new();
    /// let key = vault.add(1);
    /// 
    /// vault.clear();
    /// 
    /// let has_item = vault.has_item(&key);
    /// assert_eq!(false, has_item);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn clear(&self) {
        self.items.try_lock().unwrap().clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_has_item() {
        let vault = Vault::new();
        let to_add = "stuff";
        let key = vault.add(to_add);
        let has_item = vault.has_item(&key);
        assert_eq!(true, has_item);
    }

    #[test]
    fn has_item_wrong_key() {
        let vault = Vault::new();
        let to_add = "garbage";
        vault.add(to_add);
        let wrong_key = VaultKey::new();
        let has_item = vault.has_item(&wrong_key);
        assert_eq!(false, has_item);
    }

    #[test]
    fn add_duplicates_unique_keys() {
        let vault = Vault::new();
        let to_add = 1.0;
        let key_1 = vault.add(to_add);
        let key_2 = vault.add(to_add);
        assert_ne!(key_1, key_2);
    }

    #[test]
    fn add_with_key_has_item() {
        let vault = Vault::new();
        let to_add = 1.0;
        let key = VaultKey::new();
        let added = vault.add_with_key(to_add, &key);
        let has_item = vault.has_item(&key);
        assert_eq!(true, added);
        assert_eq!(true, has_item);
    }

    #[test]
    fn add_with_key_duplicate_fails() {
        let vault = Vault::new();
        let to_add = 1.0;
        let key = VaultKey::new();
        let added_1 = vault.add_with_key(to_add, &key);
        let added_2 = vault.add_with_key(to_add, &key);
        assert_eq!(true, added_1);
        assert_eq!(false, added_2);
    }

    #[test]
    fn add_remove() {
        let vault = Vault::new();
        let to_add = 1;
        let key = vault.add(to_add);
        let retrieved = vault.remove(&key).unwrap();
        assert_eq!(to_add, retrieved);
    }

    #[test]
    fn remove_before_add() {
        let vault = Vault::<i32>::new();
        let key = VaultKey::new();
        let retrieved = vault.remove(&key);
        assert_eq!(None, retrieved);
    }

    #[test]
    fn add_update() {
        let vault = Vault::new();
        let to_add = 1.0;
        let key = vault.add(to_add);
        let double = |i:f64| i * 2.0;
        let expected = 2.0;
        let updated = vault.update_item(&key, double);
        let retrieved = vault.remove(&key).unwrap();
        assert_eq!(true, updated);
        assert_eq!(expected, retrieved);
    }

    #[test]
    fn add_remove_twice() {
        let vault = Vault::new();
        let to_add = 3;
        let key = vault.add(to_add);
        vault.remove(&key);
        let second = vault.remove(&key);
        assert_eq!(None, second);
    }

    #[test] 
    fn add_clear_doesnt_have_item() {
        let vault = Vault::new();
        let to_add = "thing";
        let key = vault.add(to_add);
        vault.clear();
        let has_item = vault.has_item(&key);
        assert_eq!(false, has_item);
    }

    #[test]
    fn add_remove_doesnt_have_item() {
        let vault = Vault::new();
        let to_add = "an item";
        let key = vault.add(to_add);
        vault.remove(&key);
        let has_item = vault.has_item(&key);
        assert_eq!(false, has_item);
    }
}
