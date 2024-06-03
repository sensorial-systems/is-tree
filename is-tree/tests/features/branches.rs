use enum_as_inner::EnumAsInner;
use is_tree::*;

#[derive(Debug, IsTree)]
pub struct Library {
    #[tree(path_segment)]
    pub name: String,
    #[tree(branch(Module))]
    pub root_module: Module
}

impl Library {
    pub fn mock() -> Self {
        let name = String::from("library");
        let root_module = Module::mock();
        Self { name, root_module }
    }
}

impl AddBranch<Module> for Library {
    fn add_branch(&mut self, module: Module) -> &mut Module {
        self.root_module = module;
        &mut self.root_module
    }
}

impl<'a> HasBranches<&'a String> for &'a Library {
    fn branches_impl(self) -> impl Iterator<Item = &'a String> {
        std::iter::once(&self.name)
            .chain((&self.root_module).branches::<&String>())
    }
}

impl<'a> HasBranches<&'a mut String> for &'a mut Library {
    fn branches_impl(self) -> impl Iterator<Item = &'a mut String> {
        std::iter::once(&mut self.name)
            .chain((&mut self.root_module).branches::<&mut String>())
    }
}

impl AddBranch<String> for Library {
    fn add_branch(&mut self, name: String) -> &mut String {
        self.name = name;
        &mut self.name
    }
}

impl From<&str> for Library {
    fn from(name: &str) -> Self {
        let name = name.into();
        let root_module = Default::default();
        Self { name, root_module }
    }
}

#[derive(Debug, Default, IsTree)]
pub struct Function {
    #[tree(path_segment)]
    pub name: String
}

impl From<&str> for Function {
    fn from(name: &str) -> Self {
        let name = name.into();
        Self { name }
    }
}

#[derive(Debug, Default, IsTree)]
pub struct Module {
    #[tree(path_segment)]
    pub name: String,
    #[tree(branch(Module))]
    pub modules: Vec<Module>,
    #[tree(branch(Function))]
    pub functions: Vec<Function>
}

impl Module {
    pub fn mock() -> Self {
        let mut branch = Self::from("");
        branch.add_branch(String::from("math")); // Rename "" to "math".
        branch.add_branch(Module::from("geometry"))
            .add_branch(Module::from("shapes"));
        branch.add_branch(Module::from("algebra"))
            .add_branch(Function::from("exponential"));
        branch
    }
}

impl From<&str> for Module {
    fn from(name: &str) -> Self {
        let name = name.into();
        let modules = Default::default();
        let functions = Default::default();
        Self { name, modules, functions }
    }
}

impl AddBranch<Module> for Module {
    fn add_branch(&mut self, branch: Module) -> &mut Module {
        self.modules.push(branch);
        self.modules.last_mut().unwrap()
    }
}

impl AddBranch<Function> for Module {
    fn add_branch(&mut self, branch: Function) -> &mut Function {
        self.functions.push(branch);
        self.functions.last_mut().unwrap()
    }
}

impl<'a> HasBranches<&'a String> for &'a Module {
    fn branches_impl(self) -> impl Iterator<Item = &'a String> {
        std::iter::once(&self.name)
            .chain(self.modules.iter().map(|branch| &branch.name))
            .chain(self.functions.iter().map(|branch| &branch.name))
    }
}

impl<'a> HasBranches<&'a mut String> for &'a mut Module {
    fn branches_impl(self) -> impl Iterator<Item = &'a mut String> {
        std::iter::once(&mut self.name)
            .chain(self.modules.iter_mut().map(|branch| &mut branch.name))
            .chain(self.functions.iter_mut().map(|branch| &mut branch.name))
    }
}

impl AddBranch<String> for Module {
    fn add_branch(&mut self, name: String) -> &mut String {
        self.name = name;
        &mut self.name
    }
}

#[test]
fn branches() {
    let mut library = Library::mock();

    (&mut library).branches::<&mut String>().for_each(|s| *s = s.to_uppercase());
    assert_eq!((&library).branches::<&String>().map(|s| s.as_str()).collect::<Vec<_>>(), vec!["LIBRARY", "MATH", "GEOMETRY", "ALGEBRA"]);
    assert_eq!((&library).branches::<&Module>().map(|module| module.name.as_str()).collect::<Vec<_>>(), vec!["MATH"]);

    assert_eq!((&library.root_module).branches::<&String>().map(|s| s.as_str()).collect::<Vec<_>>(), vec!["MATH", "GEOMETRY", "ALGEBRA"]);
    assert_eq!((&library.root_module).branches::<&Module>().map(|module| module.name.as_str()).collect::<Vec<_>>(), vec!["GEOMETRY", "ALGEBRA"]);
}

#[test]
fn get() {
    let mut library = Library::mock();
    assert_eq!((&library).branches::<&Module>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["math"]);
    if let Some(s) = (&mut library).get::<&mut String>("math") { *s = s.to_uppercase() }
    assert_eq!((&library).branches::<&Module>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["MATH"]);

    assert_eq!((&library.root_module).branches::<&Module>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["geometry", "algebra"]);
    if let Some(s) = (&mut library.root_module).get::<&mut String>("geometry") { *s = s.to_uppercase() }
    assert_eq!((&library.root_module).branches::<&Module>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["GEOMETRY", "algebra"]);

    assert_eq!(((&library.root_module).get::<&Module>("algebra").unwrap()).branches::<&Function>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["exponential"]);
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum Visitors<'a> {
    Library(Visitor<(), &'a Library>),
    Module(Visitor<Box<Visitors<'a>>, &'a Module>),
    Function(Visitor<Box<Visitors<'a>>, &'a Function>),
}

unsafe impl<'a> UnsafeClone for Visitors<'a> {
    unsafe fn unsafe_clone(&self) -> Self {
        self.clone()
    }
}

unsafe impl<'a> UnsafeBorrow<'a> for Visitors<'a> {
    type Borrow = &'a Visitors<'a>;
    unsafe fn borrow(&'a self) -> Self::Borrow {
        self
    }
}

impl<'a> From<&'a Library> for Visitors<'a> {
    fn from(branch: &'a Library) -> Self {
        Self::Library(Visitor::new((), branch))
    }
}

impl<'a> From<Visitor<Box<Visitors<'a>>, &'a Module>> for Visitors<'a> {
    fn from(visitor: Visitor<Box<Visitors<'a>>, &'a Module>) -> Self {
        Self::Module(visitor)
    }
}

impl<'a> From<Visitor<Box<Visitors<'a>>, &'a Function>> for Visitors<'a> {
    fn from(visitor: Visitor<Box<Visitors<'a>>, &'a Function>) -> Self {
        Self::Function(visitor)
    }
}

impl<'a> HasPathSegment for Visitors<'a> {
    fn path_segment(&self) -> String {
        match self {
            Visitors::Library(visitor) => visitor.path_segment(),
            Visitors::Module(visitor) => visitor.path_segment(),
            Visitors::Function(visitor) => visitor.path_segment(),
        }
    }
}

impl<'a> HasBranches<Visitors<'a>> for &'a Visitors<'a> {
    fn branches_impl(self) -> impl Iterator<Item = Visitors<'a>> {
        match self {
            Visitors::Library(visitor) => Box::new(visitor.value.branches::<&Module>().map(|branch| Visitor::new(self.clone().into(), branch).into())) as Box<dyn Iterator<Item = _>>,
            Visitors::Module(visitor) => {
                let iterator = visitor.value.branches::<&Module>().map(|branch| Visitor::new(self.clone().into(), branch).into())
                    .chain(visitor.value.branches::<&Function>().map(|branch| Visitor::new(self.clone().into(), branch).into()));
                Box::new(iterator) as Box<dyn Iterator<Item = _>>
            },
            Visitors::Function(_) => Box::new(std::iter::empty()),
        }
    }
}

// visitor!{
//     #[derive(Debug)]
//     pub enum Visitors {
//         Library(Visits = String, Module, Function),
//         Module(Visits = String, Module, Function), 
//         Function(Visits = Function)
//     }
// }

#[derive(Debug, EnumAsInner)]
pub enum VisitorsMut<'a> {
    Library(Visitor<(), &'a mut Library>),
    Module(Visitor<Box<Visitors<'a>>, &'a mut Module>),
    Function(Visitor<Box<Visitors<'a>>, &'a mut Function>),
}

unsafe impl<'a> UnsafeClone for VisitorsMut<'a> {
    unsafe fn unsafe_clone(&self) -> Self {
        let visitor: &Visitors = std::mem::transmute(self);
        let visitor = visitor.clone();
        std::mem::transmute(visitor)
    }
}

unsafe impl<'a> UnsafeBorrow<'a> for VisitorsMut<'a> {
    type Borrow = &'a mut VisitorsMut<'a>;
    unsafe fn borrow(&'a self) -> Self::Borrow {
        #[allow(mutable_transmutes)]
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> From<&'a mut Library> for VisitorsMut<'a> {
    fn from(branch: &'a mut Library) -> Self {
        Self::Library(Visitor::new((), branch))
    }
}

impl<'a> From<Visitor<Box<Visitors<'a>>, &'a mut Module>> for VisitorsMut<'a> {
    fn from(visitor: Visitor<Box<Visitors<'a>>, &'a mut Module>) -> Self {
        Self::Module(visitor)
    }
}

impl<'a> From<Visitor<Box<Visitors<'a>>, &'a mut Function>> for VisitorsMut<'a> {
    fn from(visitor: Visitor<Box<Visitors<'a>>, &'a mut Function>) -> Self {
        Self::Function(visitor)
    }
}

impl<'a> From<&&'a mut VisitorsMut<'a>> for Visitors<'a> {
    fn from(visitor: &&'a mut VisitorsMut<'a>) -> Self {
        unsafe {
            (*(std::mem::transmute::<_, &&Visitors<'a>>(visitor))).clone()
        }
    }
}

impl<'a> HasPathSegment for VisitorsMut<'a> {
    fn path_segment(&self) -> String {
        match self {
            VisitorsMut::Library(visitor) => visitor.path_segment(),
            VisitorsMut::Module(visitor) => visitor.path_segment(),
            VisitorsMut::Function(visitor) => visitor.path_segment(),
        }
    }
}

impl<'a> HasBranches<VisitorsMut<'a>> for &'a mut VisitorsMut<'a> {
    fn branches_impl(self) -> impl Iterator<Item = VisitorsMut<'a>> {
        let parent = Box::new(Visitors::from(&self));
        match self {
            VisitorsMut::Library(visitor) => {
                let parent_clone = parent.clone();
                Box::new(visitor.value.branches::<&mut Module>().map(move |branch| Visitor::new(parent_clone.clone(), branch).into())) as Box<dyn Iterator<Item = _>>
            },
            VisitorsMut::Module(visitor) => {
                let parent_clone = parent.clone();
                let other_visitor = unsafe { longer_mut(visitor) };
                let iterator = visitor.value.branches::<&mut Module>().map(move |branch| Visitor::new(parent_clone.clone(), branch).into());
                let parent_clone = parent.clone();
                let visitor = other_visitor;
                let iterator = iterator.chain(visitor.value.branches::<&mut Function>().map(move |branch| Visitor::new(parent_clone.clone(), branch).into()));
                Box::new(iterator) as Box<dyn Iterator<Item = _>>
            },
            VisitorsMut::Function(_) => Box::new(std::iter::empty()),
        }
    }
}

#[test]
fn visitor() {
    let mut branch = Library::mock();

    let root_visitor = Visitors::from(&branch);
    assert_eq!(root_visitor.as_library().unwrap().value.name, "library");
    assert_eq!((&root_visitor).branches::<Visitors>().map(|visitor| &visitor.as_module().unwrap().value.name).collect::<Vec<_>>(), vec!["math"]);

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["exponential", "algebra", "shapes", "geometry", "math", "library"]);

    let mut root_visitor = VisitorsMut::from(&mut branch);
    (&mut root_visitor).branches::<VisitorsMut>().for_each(|mut visitor| {
        match &mut visitor {
            VisitorsMut::Library(visitor) => visitor.value.name = visitor.value.name.to_uppercase(),
            VisitorsMut::Module(visitor) => visitor.value.name = visitor.value.name.to_uppercase(),
            VisitorsMut::Function(visitor) => visitor.value.name = visitor.value.name.to_uppercase()
        }
    });

    let root_visitor = Visitors::from(&branch);
    assert_eq!(root_visitor.as_library().unwrap().value.name, "library");
    assert_eq!((&root_visitor).branches::<Visitors>().map(|visitor| &visitor.as_module().unwrap().value.name).collect::<Vec<_>>(), vec!["MATH"]);

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["exponential", "algebra", "shapes", "geometry", "MATH", "library"]);

    let iterator: TreeIterator<VisitorsMut> = TreeIterator::new(&mut branch);
    iterator.for_each(|mut visitor| {
        match &mut visitor {
            VisitorsMut::Library(visitor) => visitor.value.name = visitor.value.name.to_uppercase(),
            VisitorsMut::Module(visitor) => visitor.value.name = visitor.value.name.to_uppercase(),
            VisitorsMut::Function(visitor) => visitor.value.name = visitor.value.name.to_uppercase()
        }
    });

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["EXPONENTIAL", "ALGEBRA", "SHAPES", "GEOMETRY", "MATH", "LIBRARY"]);
}

impl<'a> HasParent for Visitors<'a> {
    fn parent(&self) -> Option<Self> {
        match self {
            Visitors::Library(_) => None,
            Visitors::Module(visitor) => Some((*visitor.parent).clone()),
            Visitors::Function(visitor) => Some((*visitor.parent).clone())
        }
    }
}

impl<'a> HasRoot for Visitors<'a> {
    fn root(&self) -> Self {
        match self {
            Visitors::Library(_) => self.clone(),
            Visitors::Module(visitor) => visitor.parent.root(),
            Visitors::Function(visitor) => visitor.parent.root()
        }
    }
}

#[test]
fn relative_access() {
    let branch = Library::mock();

    let library_visitor = Visitors::from(&branch);
    let math_visitor = (&library_visitor).branches::<Visitors>().next().unwrap();
    let geometry_visitor = (&math_visitor).branches::<Visitors>().next().unwrap();
    let shapes_visitor = (&geometry_visitor).branches::<Visitors>().next().unwrap();
    assert_eq!(shapes_visitor.path_segment(), "shapes");
    assert_eq!(shapes_visitor.parent().unwrap().path_segment(), "geometry");
    assert_eq!(shapes_visitor.parent().unwrap().parent().unwrap().path_segment(), "math");
    
    assert_eq!(shapes_visitor.root().path_segment(), "library");
    
    assert_eq!((&math_visitor).get("geometry").unwrap().path_segment(), "geometry");

    assert!(library_visitor.relative(vec!["super"]).is_none());
    assert_eq!(library_visitor.relative(Vec::<String>::new()).unwrap().path_segment(), "library");
    assert_eq!(library_visitor.relative(vec!["self"]).unwrap().path_segment(), "library");
    assert_eq!(library_visitor.relative(vec!["root"]).unwrap().path_segment(), "library");
    assert_eq!(library_visitor.relative(vec!["math"]).unwrap().path_segment(), "math");
    assert_eq!(library_visitor.relative(vec!["math", "geometry", "shapes"]).unwrap().path_segment(), "shapes");

    assert_eq!(math_visitor.relative(vec!["super"]).unwrap().path_segment(), "library");

    assert_eq!(shapes_visitor.relative(vec!["super", "super"]).unwrap().path_segment(), "math");
    assert_eq!(shapes_visitor.relative(vec!["root"]).unwrap().path_segment(), "library");

    assert_eq!(library_visitor.relative(vec!["math", "algebra", "exponential"]).unwrap().path_segment(), "exponential");
}

unsafe impl<'a> UnsafeHasParent for VisitorsMut<'a> {
    unsafe fn parent_mut(&mut self) -> Option<Self> {
        match self {
            VisitorsMut::Library(_) => None,
            VisitorsMut::Module(visitor) => {
                let visitor: Visitors = *visitor.parent.clone();
                let visitor = std::mem::transmute(visitor);
                Some(visitor)
            },
            VisitorsMut::Function(visitor) => {
                let visitor: Visitors = *visitor.parent.clone();
                let visitor = std::mem::transmute(visitor);
                Some(visitor)
            },
        }
    }
}

unsafe impl<'a> UnsafeHasRoot for VisitorsMut<'a> {
    unsafe fn root_mut(&mut self) -> Option<Self> {
        match self {
            VisitorsMut::Library(_) => None,
            VisitorsMut::Module(visitor) => {
                let visitor: Visitors = visitor.parent.root();
                let visitor = std::mem::transmute(visitor);
                Some(visitor)
            },
            VisitorsMut::Function(visitor) => {
                let visitor: Visitors = visitor.parent.root();
                let visitor = std::mem::transmute(visitor);
                Some(visitor)
            },
        }
    }
}


#[test]
fn unsafe_relative_access() {
    let mut branch = Library::mock();

    unsafe {
        let mut root_visitor = VisitorsMut::from(&mut branch);
        let mut geometry_visitor = (&mut root_visitor).branches::<VisitorsMut>().next().unwrap();
        let mut shapes_visitor = (&mut geometry_visitor).branches::<VisitorsMut>().next().unwrap();

        let mut geometry = shapes_visitor.parent_mut().unwrap();
        let geometry = geometry.as_module_mut().unwrap();
        geometry.value.name = geometry.value.name.to_uppercase();

        let mut math = shapes_visitor.root_mut().unwrap();
        let math = math.as_library_mut().unwrap();
        math.value.name = math.value.name.to_uppercase();
    }

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["exponential", "algebra", "shapes", "geometry", "MATH", "LIBRARY"]);


    let mut branch = Library::mock();

    unsafe {
        let mut root_visitor = VisitorsMut::from(&mut branch);
        if let Some(mut visitor) = root_visitor.relative_mut(vec!["math", "geometry", "shapes"]) {
            let branch_visitor = visitor.as_module_mut().unwrap();
            branch_visitor.value.name = branch_visitor.value.name.to_uppercase();

            if let Some(mut visitor) = visitor.relative_mut(vec!["root"]) {
                let visitor = visitor.as_library_mut().unwrap();
                visitor.value.name = visitor.value.name.to_uppercase();
            }
        }
    }

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["exponential", "algebra", "SHAPES", "geometry", "math", "LIBRARY"]);

    let mut branch = Library::mock();

    unsafe {
        let mut root_visitor = VisitorsMut::from(&mut branch);
        if let Some(mut visitor) = root_visitor.relative_mut(vec!["self"]) {
            let branch_visitor = visitor.as_library_mut().unwrap();
            branch_visitor.value.name = branch_visitor.value.name.to_uppercase();

            if let Some(mut visitor) = visitor.relative_mut(vec!["math", "geometry", "shapes", "super"]) {
                let visitor = visitor.as_module_mut().unwrap();
                visitor.value.name = visitor.value.name.to_uppercase();
            }
        }
    }

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["exponential", "algebra", "shapes", "GEOMETRY", "math", "LIBRARY"]);
}