pub mod traits {
    pub trait Thesaurus {
        fn get_synonyms(&mut self, &str) -> Result<Vec<String>, &str>;
    }
}

pub use thesaureses::traits::Thesaurus;

pub mod wordnik;

