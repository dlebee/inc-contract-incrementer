#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod incrementer {

    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Incrementer {
        // Storage Declaration
        value: i32,
        my_value: Mapping<AccountId, i32>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            let mut my_map = Mapping::default();
            let caller = Self::env().caller();
            my_map.insert(&caller, &0);

            Self {
                value: init_value,
                my_value: my_map,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                value: 0,
                my_value: Mapping::default()
            }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            self.my_value.get(&self.env().caller()).unwrap_or_default()
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }

        #[ink(message)]
        pub fn inc_mine(&mut self, by: i32) {
            let existing_value = self.my_value.get(&self.env().caller());
            match existing_value {
                Some(existing_value) => {
                    self.my_value.insert(&self.env().caller(), &(existing_value + by));
                },
                None => {
                    self.my_value.insert(&self.env().caller(), &by);
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            // Test Your Contrac
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn inc_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }

        #[ink::test]
        pub fn get_mine_works() {
            let contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
        }

        #[ink::test]
        pub fn set_mine_w() {
            let mut contract = Incrementer::new(0);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(10);
            assert_eq!(contract.get_mine(), 10);
        }
    }
}
