#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
#[ink::contract]
mod test {

    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::{string::{String},vec::Vec};
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Test {
        /// Stores a single `bool` value on the storage.
        map: ink_storage::Mapping<AccountId, Vec<u8>>,
    }

    impl Test {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(name: Vec<u8>) -> Self {
            
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.map.insert(&caller, &name);
            })    
        
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn set(&mut self, name: Vec<u8>) {
            self.map.insert(&self.env().caller(), &name);
        }
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> String {
            let caller = self.env().caller();
            String::from_utf8(self.map.get(&caller).unwrap()).unwrap()
        }
    }
    
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let test = Test::default();
            assert_eq!(test.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut test = Test::new(false);
            assert_eq!(test.get(), false);
            test.flip();
            assert_eq!(test.get(), true);
        }
    }
}
