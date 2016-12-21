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
    use std::error::Error as ErrorT;
    use std::fmt;
    #[derive(Debug)]
    pub enum Error {
        Hyper(::hyper::Error),
        Serde(::serde_json::Error),
        Error(Box<error::Error>)
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
    impl fmt::Display for Error {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            fmt.write_str(self.description())?;
            Ok(())
        }
    }
    impl From<::hyper::Error> for Error {
        fn from(val: ::hyper::Error) -> Self {
            Error::Hyper(val)
        }
    }
    impl From<::serde_json::Error> for Error {
        fn from(val: ::serde_json::Error) -> Self {
            Error::Serde(val)
        }
    }
    impl From<::std::io::Error> for Error {
        fn from(val: ::std::io::Error) -> Self {
            Error::Error(Box::new(val))
        }
    }
}
        
pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
