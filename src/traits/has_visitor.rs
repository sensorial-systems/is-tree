use crate::traits::*;

pub trait KnowsVisitor<'a> {
    type Visitor: IsVisitor<'a>;
}

pub trait HasVisitor<'a>: KnowsVisitor<'a> {
    fn visit(self) -> Self::Visitor;
}

impl<'a, T> HasVisitor<'a> for T
where T: HasRootVisitor
{
    fn visit(self) -> Self::Visitor {
        Self::Visitor::new(self)
    }
}
