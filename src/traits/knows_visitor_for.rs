use crate::IsVisitor;

pub trait KnowsVisitorOf<'a, Type> {
    type Visitor: IsVisitor<'a>;
}
