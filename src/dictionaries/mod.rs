pub mod wordnik;
pub mod example;

pub mod traits {
    pub struct Definition {
        pub text: String,
        pub word: String,
    }

    pub trait Dictionary {
        fn get_definitions(&mut self, &str) -> Result<Vec<Definition>, &str>;
    }
}

pub use dictionaries::traits::{Dictionary, Definition};
