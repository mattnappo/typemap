use std::any::TypeId;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use anyhow::Result;
use bimap::BiMap;
use syn::*;

/// A type in the analyzed codebase
struct Ty {
    /// Type name from std::any::type_name()
    token_name: String,

    /// Type ID from std::any::type_id()
    id: TypeId,

    /// Lexical type identifier
    ident: String,
}

/// A dependency graph of `Ty`s
#[derive(Debug)]
pub struct TypeMap {
    /// Map from a `Ty` to the `Ty`s it depends on
    graph: HashMap<String, Vec<String>>,
    //graph: HashMap<Ty, Vec<Ty>>,

    // Bijective map from type names to type IDs
    //resolver: BiMap<String, TypeId>,
}

impl TypeMap {
    /// Build a `TypeMap` from a single file.
    /// Eventually will support multi-file projects.
    pub fn build(src: &str) -> Result<Self> {
        //let mut resolver = BiMap::new();

        // Parse the file
        let mut fd = File::open(src)?;
        let mut file = String::new();
        fd.read_to_string(&mut file)?;
        let file = syn::parse_file(&file)?;
        //println!("AST:\n{:#?}", file);

        // Find all the user-defined structs and build the dependences
        let graph = Self::user_defined_types(&file)
            .into_iter()
            .map(|(type_name, s)| {
                let field_deps = Self::field_dependents(&s)
                    .into_iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<String>>();

                let trait_deps = Self::generic_dependents()

                (type_name, deps)
            })
            .collect::<HashMap<String, Vec<String>>>();

        Ok(Self { graph })
    }

    /// Return a list of pairs of user defined type identifier with their
    /// fields/generics.
    fn user_defined_types(file: &syn::File) -> Vec<(String, Vec<Fields>, Vec<Generics>)> {
        file.items
            .clone()
            .into_iter()
            .map(|item| match item {
                Item::Struct(s) => (s.ident.to_string(), vec![s.fields]),
                Item::Enum(e) => (
                    e.ident.to_string(),
                    e.variants
                        .into_iter()
                        .map(|v| v.fields)
                        .collect::<Vec<Fields>>(),
                ),
                Item::Union(u) => (u.ident.to_string(), vec![Fields::Named(u.fields)]),
                Item::Type(t) => (t.ident.to_string(), vec![]),
                Item::Trait(t) => (t.ident.to_string(), vec![]),
                _ => todo!(),
            })
            .collect::<Vec<(String, Vec<Fields>)>>()
    }

    /// Return all the type identifiers that these fields depend on
    fn field_dependents(fields: &Vec<Fields>) -> Vec<Ident> {
        fields
            .into_iter()
            .map(|f| match f {
                Fields::Unit => Vec::new(),
                Fields::Named(FieldsNamed { named: fields, .. }) => fields
                    .into_iter()
                    .map(|field| Self::base_types(&field.ty))
                    .flatten()
                    .collect::<Vec<Ident>>(),
                Fields::Unnamed(FieldsUnnamed {
                    unnamed: fields, ..
                }) => fields
                    .into_iter()
                    .map(|field| Self::base_types(&field.ty))
                    .flatten()
                    .collect::<Vec<Ident>>(),
            })
            .flatten()
            .collect::<Vec<Ident>>()
    }

    // This really should not return a vec
    // If you had a type A::B this would return [A, B], which is wrong
    fn base_types(ty: &Type) -> Vec<Ident> {
        match ty {
            Type::Path(TypePath { path, .. }) => path
                .segments
                .iter()
                .map(|seg| seg.ident.clone())
                .collect::<_>(),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! run_example {
        ($name:ident, $path:expr) => {
            #[test]
            fn $name() {
                dbg!(TypeMap::build($path).unwrap());
            }
        };
    }

    #[test]
    fn test_ex1() {
        let graph = TypeMap::build("examples/ex1.rs").unwrap().graph;
        assert_eq!(graph["A"], vec!["B", "C"]);
        assert_eq!(graph["B"], Vec::<String>::new());
        assert_eq!(graph["C"], Vec::<String>::new());
    }

    #[test]
    fn test_ex2() {
        let graph = TypeMap::build("examples/ex2.rs").unwrap().graph;
        assert_eq!(graph["A"], vec!["B", "C"]);
        assert_eq!(graph["B"], Vec::<String>::new());
        assert_eq!(graph["C"], Vec::<String>::new());
    }

    #[test]
    fn test_ex3() {
        let graph = TypeMap::build("examples/ex3.rs").unwrap().graph;
        assert_eq!(graph["A"], vec!["B"]);
        assert_eq!(graph["B"], vec!["C"]);
        assert_eq!(graph["C"], Vec::<String>::new());
    }

    #[test]
    fn test_ex4() {
        let graph = TypeMap::build("examples/ex4.rs").unwrap().graph;
        assert_eq!(graph["A"], vec!["B"]);
        assert_eq!(graph["B"], Vec::<String>::new());
    }

    #[test]
    fn test_ex5() {
        let graph = TypeMap::build("examples/ex5.rs").unwrap().graph;
        assert_eq!(graph["A"], vec!["B", "C"]);
        assert_eq!(graph["B"], Vec::<String>::new());
        assert_eq!(graph["C"], vec!["D"]);
        assert_eq!(graph["D"], vec!["i32", "usize"]);
    }

    run_example!(run_ex6, "examples/ex6.rs");

    #[test]
    fn test_ex7() {
        let graph = TypeMap::build("examples/ex7.rs").unwrap().graph;
        dbg!(&graph);
        assert_eq!(graph["A"], Vec::<String>::new());
        assert_eq!(graph["B"], vec!["A"]);
        assert_eq!(graph["C"], vec!["T"]); // Really this should be empty
    }
}
