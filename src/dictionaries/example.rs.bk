pub struct ExampleDictionary;
use dictionaries::traits::{Dictionary, Definition};

impl Dictionary for ExampleDictionary {
    fn get_definitions(&mut self, word: &str) -> Result<Vec<Definition>, &str> {
        let mut definitions = Vec::new();
        definitions.push(Definition {
            text: "This word means something".to_string(),
            word: word.to_string(),
        });
        Ok(definitions)
    }
}
