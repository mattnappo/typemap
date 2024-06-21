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
    graph: HashMap<String, Vec<Dependence>>,
    //graph: HashMap<Ty, Vec<Ty>>,

    // Bijective map from type names to type IDs
    //resolver: BiMap<String, TypeId>,
}

#[derive(Debug, PartialEq)]
enum Dependence {
    Field(String),
    Trait(String),
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
            .map(|(type_name, s, g)| {
                let mut deps = Self::field_dependents(&s)
                    .into_iter()
                    .map(|d| Dependence::Field(d))
                    .collect::<Vec<Dependence>>();
                let generic_deps = Self::generic_dependents(&g)
                    .into_iter()
                    .map(|d| Dependence::Trait(d))
                    .collect::<Vec<Dependence>>();
                deps.extend(generic_deps);
                (type_name, deps)
            })
            .collect::<HashMap<String, Vec<Dependence>>>();

        Ok(Self { graph })
    }

    /// Return a list of pairs of user defined type identifier with their
    /// fields/generics.
    fn user_defined_types(file: &syn::File) -> Vec<(String, Vec<Fields>, Vec<Generics>)> {
        file.items
            .clone()
            .into_iter()
            .map(|item| match item {
                Item::Struct(s) => (s.ident.to_string(), vec![s.fields], vec![s.generics]),
                Item::Enum(e) => (
                    e.ident.to_string(),
                    e.variants
                        .into_iter()
                        .map(|v| v.fields)
                        .collect::<Vec<Fields>>(),
                    vec![e.generics],
                ),
                Item::Union(u) => (
                    u.ident.to_string(),
                    vec![Fields::Named(u.fields)],
                    vec![u.generics],
                ),
                Item::Type(t) => (t.ident.to_string(), vec![], vec![]),
                Item::Trait(t) => (t.ident.to_string(), vec![], vec![]),
                _ => todo!(),
            })
            .collect::<Vec<(String, Vec<Fields>, Vec<Generics>)>>()
    }

    /// Return all the type identifiers that these fields depend on
    fn field_dependents(fields: &Vec<Fields>) -> Vec<String> {
        fields
            .into_iter()
            .map(|f| match f {
                Fields::Unit => Vec::new(),
                Fields::Named(FieldsNamed { named: fields, .. }) => fields
                    .into_iter()
                    .map(|field| Self::base_types(&field.ty))
                    .flatten()
                    .collect::<Vec<String>>(),
                Fields::Unnamed(FieldsUnnamed {
                    unnamed: fields, ..
                }) => fields
                    .into_iter()
                    .map(|field| Self::base_types(&field.ty))
                    .flatten()
                    .collect::<Vec<String>>(),
            })
            .flatten()
            .collect::<Vec<String>>()
    }

    /// Get the trait bounds on any generic parameters, which form a (trait) dependence.
    fn generic_dependents(generics: &Vec<Generics>) -> Vec<String> {
        generics
            .into_iter()
            .map(|g| {
                {
                    g.params
                        .clone()
                        .into_iter()
                        .map(|param| match param {
                            GenericParam::Type(t) => t
                                .bounds
                                .into_iter()
                                .map(|bound| match bound {
                                    TypeParamBound::Trait(TraitBound { path, .. }) => {
                                        Self::type_from_path(&path)
                                    }
                                    _ => todo!(),
                                })
                                .collect::<Vec<String>>(),
                            GenericParam::Lifetime(_) => {
                                todo!("lifetime parameters not yet supported")
                            }
                            GenericParam::Const(_) => todo!("const generics not yet supported"),
                        })
                        .flatten()
                        .collect::<Vec<String>>()
                }
            })
            .flatten()
            .collect::<Vec<String>>()
    }

    fn type_from_path(path: &Path) -> String {
        path.segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .collect::<_>()
    }

    // This really should not return a vec
    // If you had a type A::B this would return [A, B], which is wrong
    fn base_types(ty: &Type) -> Vec<String> {
        match ty {
            Type::Path(TypePath { path, .. }) => vec![Self::type_from_path(path)],
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

    macro_rules! edge {
        ($g:ident, $a:ident -> $($b:ident),*) => {
            assert_eq!(
                $g[stringify!($a)],
                vec![$(Dependence::Field(stringify!($b).into())),*]
            );
        };
    }

    /*
    macro_rules! empty {
        ($g:ident, $a:ident) => {
            assert_eq!(
                $g[stringify!($a)],
                vec![$(Dependence::Field(stringify!($b).into())),+]
            );
        };
    }
    */

    #[test]
    fn test_ex1() {
        let graph = TypeMap::build("examples/ex1.rs").unwrap().graph;
        edge! {graph, A -> B, C};
        edge! {graph, B -> };
        edge! {graph, C -> };
    }

    #[test]
    fn test_ex2() {
        let graph = TypeMap::build("examples/ex2.rs").unwrap().graph;
        edge! {graph, A -> B, C};
        edge! {graph, B -> };
        edge! {graph, C -> };
    }

    #[test]
    fn test_ex3() {
        let graph = TypeMap::build("examples/ex3.rs").unwrap().graph;
        edge! {graph, A -> B };
        edge! {graph, B -> C };
        edge! {graph, C ->   };
    }

    #[test]
    fn test_ex4() {
        let graph = TypeMap::build("examples/ex4.rs").unwrap().graph;
        edge! {graph, A -> B };
        edge! {graph, B ->   };
    }

    #[test]
    fn test_ex5() {
        let graph = TypeMap::build("examples/ex5.rs").unwrap().graph;
        edge! {graph, A -> B, C };
        edge! {graph, B ->      };
        edge! {graph, C -> D    };
        edge! {graph, D -> i32, usize };
    }

    run_example!(run_ex6, "examples/ex6.rs");

    #[test]
    fn test_ex7() {
        let graph = TypeMap::build("examples/ex7.rs").unwrap().graph;
        dbg!(&graph);
        edge! {graph, A ->      };
        edge! {graph, B -> A    };
        edge! {graph, C -> T    };
    }
}
