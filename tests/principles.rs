//! No access variants principle.
//! object.property() instead of object.take_property()
//! (&object).property() instead of object.property_ref()
//! (&mut object).property() instead of object.property_mut()

pub struct Structure {
    string: String
}

impl Default for Structure {
    fn default() -> Self {
        Self { string: "borrow".into() }
    }
}

pub trait Trait<'a> {
    type Type;
    fn string(self) -> Self::Type;
}

impl Trait<'_> for Structure {
    type Type = String;
    fn string(mut self) -> Self::Type {
        self.string = "move".into();
        self.string
    }
}

impl<'a> Trait<'a> for &'a Structure {
    type Type = &'a String;
    fn string(self) -> Self::Type {
        &self.string
    }
}

impl<'a> Trait<'a> for &'a mut Structure {
    type Type = &'a mut String;
    fn string(self) -> Self::Type {
        self.string = "mutable borrow".into();
        &mut self.string
    }
}

#[test]
fn test() {
    let mut structure = Structure::default();
    let borrow: &String = (&structure).string();
    assert_eq!(borrow, "borrow");
    let mutable_borrow: &mut String = (&mut structure).string();
    assert_eq!(mutable_borrow, "mutable borrow");
    let move_ = structure.string();
    assert_eq!(move_, "move");
}
