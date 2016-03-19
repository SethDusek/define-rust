pub mod wordnik;
pub mod example;
pub mod urban;

pub mod traits {
    #[derive(Clone, Debug)]
    pub struct Definition {
        pub text: String,
        pub word: String,
    }

    pub trait Dictionary: Send {
        fn get_definitions(&mut self, &str) -> Result<Vec<Definition>, &str>;
        fn clone_to_box(&self) -> Box<Dictionary>;
    }
}

pub use dictionaries::traits::{Dictionary, Definition};
