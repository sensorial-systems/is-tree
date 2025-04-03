pub mod empty;

use is_tree::*;

visitor! {
    pub enum Visitors, VisitorsMut {
        Root(Library visits [Module, String]),
        Branches(
            Module visits [Module, Function, String],
            Function visits [String],
            String
        )
    }
}

#[derive(Debug, Default, IsTree)]
#[tree(branches)]
pub struct Library {
    #[tree(path_segment)]
    pub name: String,
    pub root_module: Module
}

impl Library {
    pub fn mock() -> Self {
        Library {
            name: String::from("library"),
            root_module: Module {
                name: String::from("math"),
                modules: vec![
                    Module {
                        name: String::from("geometry"),
                        modules: vec![Module { name: String::from("shapes"), .. Default::default() }],
                        .. Default::default()
                    },
                    Module {
                        name: String::from("algebra"),
                        functions: vec![Function { name: String::from("exponential") }],
                        .. Default::default()
                    },
                ],
                .. Default::default()
            },
        }
    }
}

#[derive(Debug, Default, IsTree)]
#[tree(branches)]
pub struct Function {
    #[tree(path_segment)]
    pub name: String
}

#[derive(Debug, Default, IsTree)]
#[tree(branches)]
pub struct Module {
    #[tree(path_segment)]
    pub name: String,
    pub modules: Vec<Module>,
    pub functions: Vec<Function>
}

#[test]
fn branches() {
    let mut library = Library::mock();

    library.branches_mut::<&mut String>().for_each(|s| *s = s.to_uppercase());
    assert_eq!(library.branches::<&String>().map(|s| s.as_str()).collect::<Vec<_>>(), vec!["LIBRARY"]);
    assert_eq!(library.branches::<&Module>().map(|module| module.name.as_str()).collect::<Vec<_>>(), vec!["math"]);

    assert_eq!(library.root_module.branches::<&String>().map(|s| s.as_str()).collect::<Vec<_>>(), vec!["math"]);
    assert_eq!(library.root_module.branches::<&Module>().map(|module| module.name.as_str()).collect::<Vec<_>>(), vec!["geometry", "algebra"]);
}

#[test]
fn all_branches() {
    let library = Library::mock();
    let modules = library.all_branches::<&Module>().map(|module| module.name.as_str()).collect::<Vec<_>>();
    assert_eq!(modules, vec!["geometry", "algebra"]);
}

#[test]
fn get() {
    let mut library = Library::mock();
    assert_eq!(library.branches::<&Module>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["math"]);
    if let Some(module) = library.get_mut::<&mut Module>("math") { module.name = module.name.to_uppercase() }
    assert_eq!(library.get::<&Module>("MATH").unwrap().name, "MATH");
    assert_eq!(library.branches::<&Module>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["MATH"]);

    assert_eq!(library.root_module.branches::<&Module>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["geometry", "algebra"]);
    if let Some(module) = library.root_module.get_mut::<&mut Module>("geometry") { module.name = module.name.to_uppercase() }
    assert_eq!(library.root_module.branches::<&Module>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["GEOMETRY", "algebra"]);

    assert_eq!((library.root_module.get::<&Module>("algebra").unwrap()).branches::<&Function>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["exponential"]);
}

#[test]
fn visitor() {
    let mut branch = Library::mock();

    let root_visitor = Visitors::from(&branch);
    assert_eq!(root_visitor.as_library().unwrap().value.name, "library");
    assert_eq!(root_visitor.branches().filter_map(|visitor| visitor.into_module().ok()).map(|visitor| &visitor.value.name).collect::<Vec<_>>(), vec!["math"]);

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.filter(|visitor| !visitor.is_string()).map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["exponential", "algebra", "shapes", "geometry", "math", "library"]);

    let mut root_visitor = VisitorsMut::from(&mut branch);
    root_visitor.branches_mut::<VisitorsMut>().for_each(|mut visitor| {
        match &mut visitor {
            VisitorsMut::Library(visitor) => visitor.value.name = visitor.value.name.to_uppercase(),
            VisitorsMut::Module(visitor) => visitor.value.name = visitor.value.name.to_uppercase(),
            VisitorsMut::Function(visitor) => visitor.value.name = visitor.value.name.to_uppercase(),
            VisitorsMut::String(visitor) => *visitor.value = visitor.value.to_uppercase()
        }
    });

    let root_visitor = Visitors::from(&branch);
    assert_eq!(root_visitor.as_library().unwrap().value.name, "LIBRARY");
    assert_eq!(root_visitor.branches().filter_map(|visitor| visitor.into_module().ok()).map(|visitor| &visitor.value.name).collect::<Vec<_>>(), vec!["MATH"]);

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.filter(|visitor| !visitor.is_string()).map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["exponential", "algebra", "shapes", "geometry", "MATH", "LIBRARY"]);

    let iterator: TreeIterator<VisitorsMut> = TreeIterator::new(&mut branch);
    iterator.for_each(|mut visitor| {
        match &mut visitor {
            VisitorsMut::Library(visitor) => visitor.value.name = visitor.value.name.to_uppercase(),
            VisitorsMut::Module(visitor) => visitor.value.name = visitor.value.name.to_uppercase(),
            VisitorsMut::Function(visitor) => visitor.value.name = visitor.value.name.to_uppercase(),
            VisitorsMut::String(visitor) => *visitor.value = visitor.value.to_uppercase()
        }
    });

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.filter(|visitor| !visitor.is_string()).map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["EXPONENTIAL", "ALGEBRA", "SHAPES", "GEOMETRY", "MATH", "LIBRARY"]);
}

#[test]
fn relative_access() {
    let branch = Library::mock();

    let library_visitor = Visitors::from(&branch);
    let math_visitor = library_visitor.branches::<Visitors>().next().unwrap();
    let geometry_visitor = math_visitor.branches::<Visitors>().next().unwrap();
    let shapes_visitor = geometry_visitor.branches::<Visitors>().next().unwrap();
    assert_eq!(shapes_visitor.path_segment(), "shapes");
    assert_eq!(shapes_visitor.parent().unwrap().path_segment(), "geometry");
    assert_eq!(shapes_visitor.parent().unwrap().parent().unwrap().path_segment(), "math");
    
    assert_eq!(shapes_visitor.root().path_segment(), "library");
    
    assert_eq!(math_visitor.get("geometry").unwrap().path_segment(), "geometry");

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

#[test]
fn unsafe_mutable_relative_access() {
    let mut branch = Library::mock();

    unsafe {
        let mut root_visitor = VisitorsMut::from(&mut branch);
        let mut geometry_visitor = root_visitor.branches_mut::<VisitorsMut>().next().unwrap();
        let mut shapes_visitor = geometry_visitor.branches_mut::<VisitorsMut>().next().unwrap();

        let mut geometry = shapes_visitor.parent_mut().unwrap();
        let geometry = geometry.as_module_mut().unwrap();
        geometry.value.name = geometry.value.name.to_uppercase();

        let mut math = shapes_visitor.root_mut();
        let math = math.as_library_mut().unwrap();
        math.value.name = math.value.name.to_uppercase();
    }

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.filter(|visitor| !visitor.is_string()).map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["exponential", "algebra", "shapes", "geometry", "MATH", "LIBRARY"]);

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
    assert_eq!(iterator.filter(|visitor| !visitor.is_string()).map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["exponential", "algebra", "SHAPES", "geometry", "math", "LIBRARY"]);

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
    assert_eq!(iterator.filter(|visitor| !visitor.is_string()).map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["exponential", "algebra", "shapes", "GEOMETRY", "math", "LIBRARY"]);
}

impl From<String> for Module {
    fn from(name: String) -> Self {
        Module { name, .. Default::default() }
    }
}

impl AddBranch<Module> for Module {
    fn add_branch(&mut self, module: Module) -> &mut Module {
        self.modules.push(module);
        self.modules.last_mut().unwrap()
    }
}

impl AddBranch<Module> for Library {
    fn add_branch(&mut self, module: Module) -> &mut Module {
        self.root_module = module;
        &mut self.root_module
    }
}

#[test]
fn branch_fn() {
    let mut root = Library { name: "Root".into(), ..Default::default() };
        root.branch("Branch")
            .branch("Leaf");
    assert_eq!(TreeIterator::<Visitors>::new(&root).filter(|visitor| !visitor.is_string()).map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["Leaf", "Branch", "Root"]);
}

#[test]
fn branch_visitor() { // Visitor<Parent, Value>
    let mut library: Library = Library::mock();
    let visitor: Visitors = Visitors::from(&library);
    let visitor: Visitor<Box<Visitors>, &Module> = visitor.relative(vec!["math", "geometry"]).unwrap().into_module().unwrap();
    assert_eq!(visitor.parent().unwrap().path_segment(), "math");
    assert_eq!(visitor.root().path_segment(), "library");
    // assert_eq!(visitor.get("shapes").unwrap().path_segment(), "shapes");

    unsafe {
        let mut visitor: VisitorsMut = VisitorsMut::from(&mut library);
        let mut visitor: Visitor<Box<Visitors>, &mut Module> = visitor.relative_mut(vec!["math", "geometry"]).unwrap().into_module().unwrap();
        assert_eq!(visitor.parent().unwrap().path_segment(), "math");
        assert_eq!(visitor.root().path_segment(), "library");
        // assert_eq!((visitor.get("shapes").unwrap()).path_segment(), "shapes");

        visitor.parent_mut().unwrap().as_module_mut().unwrap().value.name = visitor.parent_mut().unwrap().as_module_mut().unwrap().value.name.to_uppercase();
        visitor.root_mut().as_library_mut().unwrap().value.name = visitor.root().as_library_mut().unwrap().value.name.to_uppercase();
        // visitor.get_mut("shapes").unwrap().as_module_mut().unwrap().value.name = visitor.get_mut("shapes").unwrap().as_module_mut().unwrap().value.name.to_uppercase();

        assert_eq!(visitor.parent().unwrap().path_segment(), "MATH");
        assert_eq!(visitor.root().path_segment(), "LIBRARY");
        // assert_eq!((visitor.get("SHAPES").unwrap()).path_segment(), "SHAPES");

        // let visitor = visitor.relative(vec!["self"]).unwrap();

        // visitor.root_mut().as_library_mut().unwrap().value.name = visitor.root_mut().as_library_mut().unwrap().value.name.to_uppercase();
        // assert_eq!(visitor.root().path_segment(), "LIBRARY");
    }

}