#![feature(proc_macro, custom_derive, custom_attribute)]
#![feature(box_syntax)]
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
pub mod dictionaries;
pub mod thesaureses;
pub use dictionaries::{Definition, Dictionary};
pub use thesaureses::Thesaurus;
mod error {
    use std::error;
    use std::fmt;
    #[derive(Debug)]
    pub enum Error {
        Hyper(::hyper::Error),
        Serde(::serde_json::Error),
        Error(Box<error::Error>)
    }
    impl fmt::Display for Error {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            fmt.write_str(self.description())?;
            Ok(())
        }
    }
    impl error::Error for Error {
        fn description(&self) -> &str {
            match *self {
                Error::Hyper(ref hyper) => hyper.description(),
                Error::Serde(ref serde) => serde.description(),
                Error::Error(ref err) => err.description()
            }
        }
        fn cause(&self) -> Option<&::std::error::Error> { None }
    }
}
        
pub use error::Error;

