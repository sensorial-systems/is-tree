use ::is_tree::*;

pub mod library;
pub mod module;
pub mod visitors;

mod access;
mod iterator;

use library::*;
use module::*;

fn hierarchy() -> Library {
    Library {
        name: String::from("a"),
        root_module: Module {
            name: String::from("b"),
            children: vec![
                Module {
                    name: String::from("c"),
                    children: vec![
                        Module {
                            name: String::from("d"),
                            children: vec![
                                Module {
                                    name: String::from("1"),
                                    children: vec![]
                                },
                                Module {
                                    name: String::from("2"),
                                    children: vec![]
                                },
                                Module {
                                    name: String::from("3"),
                                    children: vec![]
                                }
                            ]
                        }
                    ]
                }
            ]
        }
    }
}
