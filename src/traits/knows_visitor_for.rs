use crate::IsVisitor;

pub trait KnowsVisitorFor<'a, Base> {
    type Visitor: IsVisitor<'a>;
}
