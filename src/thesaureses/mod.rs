pub trait Thesaurus {
    fn get_synonyms(&mut self, &str) -> Result<Vec<String>, &str>;
}
pub mod wordnik;
