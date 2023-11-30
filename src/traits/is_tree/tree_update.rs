use crate::HasPathSegment;

pub trait TreeUpdate: HasPathSegment {
    fn add_branch(&mut self, _child: impl Into<Self>) -> &mut Self where Self: Sized {
        self
    }

    fn remove_branch(&mut self, _identifier: &Self::PathSegment) {
    }
}
