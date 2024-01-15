//! # Do not force borrow on self
//! 
//! Write traits as:
//! ```rust
//! trait Trait {
//!     type Type;
//!     fn method(self) -> Self::Type;
//! }
//! ```
//! 
//! Considering the structure:
//! ```rust
//! #[derive(Default)]
//! struct Structure {
//!     string: String
//! }
//! ```
//! 
//! We can implement it as &self:
//! 
//! ```
//! impl<'a> Trait for &'a Structure {
//!     type Type = &'a str;
//!     fn method(self) -> Self::Type {
//!         &self.string
//!     }
//! }
//! ```
//! 
//! Which can be used like:
//! 
//! ```rust
//! let structure = Structure::default();
//! let string: &str = structure.method();
//! ```
//! 
//! If we add more implementations:
//! 
//! ```rust
//! impl Trait for Structure {
//!     type Type = String;
//!     fn method(self) -> Self::Type {
//!         self.string
//!     }
//! }
//! 
//! impl<'a> Trait for &'a mut Structure {
//!     type Type = &'a mut String;
//!     fn method(self) -> Self::Type {
//!         &mut self.string
//!     }
//! }
//! ```
//! 
//! It changes the usage to:
//! ```rust
//! let mut structure = Structure::default();
//! let string: &str = (&structure).method();
//! let string: &mut String = (&mut structure).method();
//! let string: String = structure.method();
//! ```
//! 

#[derive(Default)]
pub struct Structure {
    string: String
}

pub trait Trait {
    type Type;
    fn string(self) -> Self::Type;
}

impl Trait for Structure {
    type Type = String;
    fn string(self) -> Self::Type {
        self.string
    }
}

impl<'a> Trait for &'a Structure {
    type Type = &'a String;
    fn string(self) -> Self::Type {
        &self.string
    }
}

impl<'a> Trait for &'a mut Structure {
    type Type = &'a mut String;
    fn string(self) -> Self::Type {
        &mut self.string
    }
}

#[test]
fn test() {
    let mut structure = Structure::default();
    let _: &String = (&structure).string();
    let _: &mut String = (&mut structure).string();
    let _: String = structure.string();
}
