#[macro_export]
macro_rules! chain {
    () => {
        std::iter::empty()
    };

    ($x:expr) => {
        $x
    };

    ($x:expr, $($xs:expr),+) => {
        $x.chain(chain!($($xs),+))
    };
}

///! A macro for generating visitor types.
/// Example:
/// ```rust,ignore
/// use ::is_tree::*;
/// visitor! {
///    pub enum Visitors, VisitorMut {
///       Root(Root visits [Type1]),
///       Branches(
///         Branch1 visits [Type1, Type2, Type3],
///         Branch2 visits [Type2, Type4],
///         Branch3
///       )
///    }
/// }
/// ```
#[macro_export]
macro_rules! visitor {
    (
        $($access:tt)? enum $name:ident, $name_mut:ident {
            Root($root:ident $(visits [$($root_host:ident),*])?),
            Branches(
                $($branch:ident $(visits [$($branch_host:ident),*])?),*
            )
        }
    ) => {
        use $crate::*;

        #[derive(Clone, $crate::prelude::EnumAsInner)]
        $($access)? enum $name<'a> {
            $root($crate::Visitor<(), &'a $root>),
            $($branch($crate::Visitor<Box<$name<'a>>, &'a $branch>)),*
        }

        #[derive($crate::prelude::EnumAsInner)]
        $($access)? enum $name_mut<'a> {
            $root($crate::Visitor<(), &'a mut $root>),
            $($branch($crate::Visitor<Box<$name<'a>>, &'a mut $branch>)),*
        }

        impl<'a> From<&&'a mut $name_mut<'a>> for $name<'a> { // FIXME: This is unsafe. We should have a UnsafeFrom trait.
            fn from(visitor: &&'a mut $name_mut<'a>) -> Self {
                unsafe {
                    (*(std::mem::transmute::<_, &&$name<'a>>(visitor))).clone()
                }
            }
        }        

        impl<'a> From<$crate::Visitor<(), &'a $root>> for $name<'a> {
            fn from(visitor: $crate::Visitor<(), &'a $root>) -> Self {
                Self::$root(visitor)
            }
        }

        impl<'a> From<$crate::Visitor<(), &'a mut $root>> for $name_mut<'a> {
            fn from(visitor: $crate::Visitor<(), &'a mut $root>) -> Self {
                Self::$root(visitor)
            }
        }

        impl<'a> From<&'a $root> for $name<'a> {
            fn from(branch: &'a $root) -> Self {
                Self::$root($crate::Visitor::new((), branch))
            }
        }

        impl<'a> From<&'a mut $root> for $name_mut<'a> {
            fn from(branch: &'a mut $root) -> Self {
                Self::$root($crate::Visitor::new((), branch))
            }
        }

        $(
            impl<'a> From<$crate::Visitor<Box<$name<'a>>, &'a $branch>> for $name<'a> {
                fn from(visitor: $crate::Visitor<Box<$name<'a>>, &'a $branch>) -> Self {
                    Self::$branch(visitor)
                }
            }

            impl<'a> From<$crate::Visitor<Box<$name<'a>>, &'a mut $branch>> for $name_mut<'a> {
                fn from(visitor: $crate::Visitor<Box<$name<'a>>, &'a mut $branch>) -> Self {
                    Self::$branch(visitor)
                }
            }
        )*

        unsafe impl<'a> UnsafeClone for $name<'a> {
            unsafe fn unsafe_clone(&self) -> Self {
                self.clone()
            }
        }

        unsafe impl<'a> UnsafeClone for $name_mut<'a> {
            unsafe fn unsafe_clone(&self) -> Self {
                let visitor: &$name = std::mem::transmute(self);
                let visitor = visitor.clone();
                std::mem::transmute(visitor)
            }
        }

        unsafe impl<'a> UnsafeBorrow<'a> for $name<'a> {
            type Borrow = &'a $name<'a>;
            unsafe fn borrow(&'a self) -> Self::Borrow {
                self
            }
        }

        unsafe impl<'a> UnsafeBorrow<'a> for $name_mut<'a> {
            type Borrow = &'a mut $name_mut<'a>;
            unsafe fn borrow(&'a self) -> Self::Borrow {
                #[allow(mutable_transmutes)]
                unsafe { std::mem::transmute(self) }
            }
        }

        impl<'a> HasPathSegment for $name<'a> {
            fn path_segment(&self) -> String {
                match self {
                    $name::$root(visitor) => visitor.path_segment(),
                    $($name::$branch(visitor) => visitor.path_segment()),*
                }
            }
        }

        impl<'a> HasPathSegment for $name_mut<'a> {
            fn path_segment(&self) -> String {
                match self {
                    $name_mut::$root(visitor) => visitor.path_segment(),
                    $($name_mut::$branch(visitor) => visitor.path_segment()),*
                }
            }
        }

        impl<'a> HasPath for $name<'a> {
            fn path(&self) -> Path {
                match self {
                    $name::$root(visitor) => visitor.path(),
                    $($name::$branch(visitor) => visitor.path()),*
                }
            }
        }        

        impl<'a> HasPath for $name_mut<'a> {
            fn path(&self) -> Path {
                match self {
                    $name_mut::$root(visitor) => visitor.path(),
                    $($name_mut::$branch(visitor) => visitor.path()),*
                }
            }
        }        

        impl<'a> HasParent for $name<'a> {
            fn parent(&self) -> Option<Self> {
                match self {
                    $name::$root(_) => None,
                    $($name::$branch(visitor) => Some((*visitor.parent).clone())),*
                }
            }
        }
        
        impl<'a> HasRoot for $name<'a> {
            fn root(&self) -> Self {
                match self {
                    $name::$root(_) => self.clone(),
                    $($name::$branch(visitor) => visitor.parent.root()),*
                }
            }
        }

        unsafe impl<'a> UnsafeHasParent for $name_mut<'a> {
            unsafe fn parent_mut(&mut self) -> Option<Self> {
                match self {
                    $name_mut::Library(_) => None,
                    $($name_mut::$branch(visitor) => {
                        let visitor: $name = *visitor.parent.clone();
                        let visitor = std::mem::transmute(visitor);
                        Some(visitor)
                    }),*
                }
            }
        }
        
        unsafe impl<'a> UnsafeHasRoot for $name_mut<'a> {
            unsafe fn root_mut(&mut self) -> Self {
                match self {
                    $name_mut::Library(_) => self.unsafe_clone(),
                    $($name_mut::$branch(visitor) => {
                        let visitor: $name = visitor.parent.root();
                        let visitor = std::mem::transmute(visitor);
                        visitor
                    }),*
                }
            }
        }

        impl<'a> $crate::HasBranches<$name<'a>> for &'a $name<'a> {
            fn branches_impl(self) -> impl Iterator<Item = $name<'a>> {
                match self {
                    $name::$root(visitor) => Box::new(
                        chain!(
                            $(
                                $(
                                    visitor.value.branches_impl2::<&$root_host>().map(|branch| $crate::Visitor::new(self.clone().into(), branch).into())
                                ),*
                            )?
                        )
                    ) as Box<dyn Iterator<Item = _>>,
                    $($name::$branch(visitor) => {
                        Box::new(
                            chain!(
                                $(
                                    $(
                                        visitor.value.branches_impl2::<&$branch_host>().map(|branch| $crate::Visitor::new(self.clone().into(), branch).into())
                                    ),*
                                )?
                            )
                        ) as Box<dyn Iterator<Item = _>>
                    }),*
                }
            }
        }

        impl<'a> $crate::HasBranches<$name_mut<'a>> for &'a mut $name_mut<'a> {
            fn branches_impl(self) -> impl Iterator<Item = $name_mut<'a>> {
                let parent = Box::new($name::from(&self));
                match self {
                    $name_mut::$root(visitor) => Box::new(
                        chain!(
                            $(
                                $(
                                    {
                                        let parent_clone = parent.clone();
                                        let visitor = unsafe { longer_mut(visitor) };
                                        visitor.value.branches_impl2::<&mut $root_host>().map(move |branch| $crate::Visitor::new(parent_clone.clone(), branch).into())
                                    }
                                ),*
                            )?
                        )
                    ) as Box<dyn Iterator<Item = _>>,
                    $($name_mut::$branch(visitor) => {
                        Box::new(
                            chain!(
                                $(
                                    $(
                                        {
                                            let parent_clone = parent.clone();
                                            let visitor = unsafe { longer_mut(visitor) };
                                            visitor.value.branches_impl2::<&mut $branch_host>().map(move |branch| $crate::Visitor::new(parent_clone.clone(), branch).into())
                                        }
                                    ),*
                                )?
                            )
                        ) as Box<dyn Iterator<Item = _>>
                    }),*
                }
            }
        }        
    };
}
