#[derive(Copy, Clone)]
pub struct ExampleDictionary;
use dictionaries::{Dictionary, Definition};

impl Dictionary for ExampleDictionary {
    fn get_definitions(&mut self, word: &str) -> Result<Vec<Definition>, &str> {
        let mut definitions = Vec::new();
        definitions.push(Definition {
            text: "This word means something".to_string(),
            word: word.to_string(),
        });
        Ok(definitions)
    }

    fn clone_to_box(&self) -> Box<Dictionary> { Box::new(*self) }
}
