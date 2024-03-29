use ::is_tree::*;

use super::Module;

// TODO: derive HasGet
impl<'a> HasGet<'a> for &'a Module {}
impl<'a> HasGet<'a> for &'a mut Module {}

// TODO: How to create a blanket implementation for this?
impl<'a> HasGetOrCreate<'a> for &'a mut Module
where Self::Branches: KnowsPathSegment<PathSegment = String> + KnowsOwned<Owned = Module>
{
    fn branch<PathSegment>(self, segment: PathSegment) -> &'a mut <Self::Branches as KnowsOwned>::Owned
    where Self::Branches: KnowsPathSegment + KnowsOwned,
          PathSegment: Into<<Self::Branches as KnowsPathSegment>::PathSegment>
    {
        let segment = segment.into();
        // This works and it's safe, but the borrow checker doesn't like it.
        // https://rust-lang.github.io/rfcs/2094-nll.html#problem-case-3-conditional-control-flow-across-functions
        let myself = unsafe { &mut *(self as *mut Module) };
        if let Some(branch) = myself.get(segment.clone()) {
            branch
        } else {
            self.add_branch(segment)
        }
    }
}
