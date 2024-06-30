use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

use anyhow::Result;
use syn::*;

pub mod dot;

pub type Set<T> = HashSet<T>;

pub type DepGraph = HashMap<String, Set<Dependence>>;

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
    graph: DepGraph,
    //graph: HashMap<Ty, Vec<Ty>>,

    // Bijective map from type names to type IDs
    //resolver: BiMap<String, TypeId>,
}

// TODO: would be nice to have extra annotations within "Field/Type" (like struct/enum/fn)
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum Dependence {
    Field(String), // TODO Really should be called "Type"
    Trait(String),
}

impl ToString for Dependence {
    fn to_string(&self) -> String {
        match self {
            Self::Field(s) => s.into(),
            Self::Trait(s) => s.into(),
        }
    }
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
                let field_deps = Set::from_iter(
                    Self::field_dependents(&s), // .into_iter()
                                                // .map(|d| Dependence::Field(d))
                                                // .collect::<Vec<Dependence>>(),
                );

                let generic_deps = Set::from_iter(
                    Self::generic_dependents(&g), // .into_iter()
                                                  // .map(|d| Dependence::Trait(d))
                                                  // .collect::<Vec<Dependence>>(),
                );

                let generic_names = Set::from_iter(
                    Self::generic_names(&g)
                        .into_iter()
                        .map(|n| Dependence::Field(n))
                        .collect::<Vec<Dependence>>(),
                );

                // deps = (field_deps \ generic_names) U generic_deps
                let deps = field_deps
                    .difference(&generic_names)
                    .cloned()
                    .collect::<Set<Dependence>>()
                    .union(&generic_deps)
                    .cloned()
                    .collect();

                (type_name, deps)
            })
            .collect::<DepGraph>();

        Ok(Self { graph })
    }

    pub fn graph(&self) -> &DepGraph {
        &self.graph
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
                // TODO: Also need to add supertrait support
                Item::Type(t) => (t.ident.to_string(), vec![], vec![t.generics]),
                Item::Trait(t) => (t.ident.to_string(), vec![], vec![t.generics]),
                // Item::Mod(m) => Self::user_defined_types(...)
                _ => todo!(),
            })
            .collect::<Vec<(String, Vec<Fields>, Vec<Generics>)>>()
    }

    /// Return all the type identifiers that these fields depend on
    // TODO: move the `Dependence` wrapper type in here
    fn field_dependents(fields: &Vec<Fields>) -> Vec<Dependence> {
        fields
            .into_iter()
            .map(|f| match f {
                Fields::Unit => Vec::new(),
                Fields::Named(FieldsNamed { named: fields, .. }) => fields
                    .into_iter()
                    .map(|field| Self::base_types(&field.ty))
                    .flatten()
                    .map(|f| Dependence::Field(f))
                    .collect::<Vec<Dependence>>(),
                Fields::Unnamed(FieldsUnnamed {
                    unnamed: fields, ..
                }) => fields
                    .into_iter()
                    .map(|field| Self::base_types(&field.ty))
                    .flatten()
                    .map(|f| Dependence::Field(f))
                    .collect::<Vec<Dependence>>(),
            })
            .flatten()
            .collect::<Vec<Dependence>>()
    }

    /// Get the trait bounds on any generic parameters, which form a (trait) dependence.
    fn generic_dependents(generics: &Vec<Generics>) -> Vec<Dependence> {
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
                                .map(|b| Dependence::Trait(b))
                                .collect::<Vec<Dependence>>(),
                            GenericParam::Lifetime(_) => {
                                vec![]
                            }
                            GenericParam::Const(_) => todo!("const generics not yet supported"),
                        })
                        .flatten()
                        .collect::<Vec<Dependence>>()
                }
            })
            .flatten()
            .collect::<Vec<Dependence>>()
    }

    /// The generic parameter names (without type bounds)
    fn generic_names(generics: &Vec<Generics>) -> Vec<String> {
        generics
            .into_iter()
            .map(|g| {
                {
                    g.params
                        .clone()
                        .into_iter()
                        .map(|param| match param {
                            GenericParam::Type(t) => t.ident.to_string(),
                            GenericParam::Lifetime(_) => "".into(),
                            GenericParam::Const(_) => todo!("const generics not yet supported"),
                        })
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
            .collect::<Vec<String>>()
            .join("::")
    }

    // This really should not return a vec
    // If you had a type A::B this would return [A, B], which is wrong
    // TODO: change to HashSet
    fn base_types(ty: &Type) -> Vec<String> {
        match ty {
            Type::Path(TypePath { path, .. }) => vec![Self::type_from_path(path)],
            Type::Array(TypeArray { elem, .. }) => Self::base_types(elem),
            Type::BareFn(TypeBareFn { inputs, output, .. }) => {
                let mut tys = vec![];
                let input_tys = inputs
                    .into_iter()
                    .map(|i| Self::base_types(&i.ty))
                    .flatten()
                    .collect::<Vec<String>>();
                if let ReturnType::Type(_, ty) = output {
                    tys.extend(Self::base_types(ty))
                }
                tys.extend(input_tys);
                tys
            }
            Type::Tuple(TypeTuple { elems, .. }) => elems
                .into_iter()
                .map(|i| Self::base_types(i))
                .flatten()
                .collect::<Vec<String>>(),
            Type::Slice(TypeSlice { elem, .. }) => Self::base_types(elem),
            Type::ImplTrait(TypeImplTrait { bounds, .. }) => {
                // TODO: these need to be marked not as fields, but as Dependence::Traits
                bounds
                    .into_iter()
                    .map(|b| match b {
                        TypeParamBound::Trait(t) => Self::type_from_path(&t.path),
                        _ => "".into(),
                    })
                    .collect::<Vec<String>>()
            }
            Type::Reference(TypeReference { elem, .. }) => Self::base_types(elem),
            _ => vec![],
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
                Set::from([$(Dependence::Field(stringify!($b).into())),*])
            );
        };
    }

    macro_rules! tr {
        ($a:ident) => {
            Dependence::Trait(stringify!($a).into())
        };
    }

    macro_rules! fi {
        ($a:ident) => {
            Dependence::Field(stringify!($a).into())
        };
    }

    // A "raw" edge
    macro_rules! redge {
        ($g:ident, $a:ident -> $($b:expr),*) => {
            assert_eq!(
                $g[stringify!($a)],
                Set::from([$($b),*])
            );
        };
    }

    #[test]
    fn test_ex01() {
        let graph = TypeMap::build("examples/ex01.rs").unwrap().graph;
        edge! {graph, A -> B, C};
        edge! {graph, B -> };
        edge! {graph, C -> };
    }

    #[test]
    fn test_ex02() {
        let graph = TypeMap::build("examples/ex02.rs").unwrap().graph;
        edge! {graph, A -> B, C};
        edge! {graph, B -> };
        edge! {graph, C -> };
    }

    #[test]
    fn test_ex03() {
        let graph = TypeMap::build("examples/ex03.rs").unwrap().graph;
        edge! {graph, A -> B };
        edge! {graph, B -> C };
        edge! {graph, C ->   };
    }

    #[test]
    fn test_ex04() {
        let graph = TypeMap::build("examples/ex04.rs").unwrap().graph;
        edge! {graph, A -> B };
        edge! {graph, B ->   };
    }

    #[test]
    fn test_ex05() {
        let graph = TypeMap::build("examples/ex05.rs").unwrap().graph;
        edge! {graph, A -> B, C };
        edge! {graph, B ->      };
        edge! {graph, C -> D    };
        edge! {graph, D -> i32, usize };
    }

    // This test is not perfect yet, as `type` aliases don't have deps
    #[test]
    fn test_ex06() {
        let graph = TypeMap::build("examples/ex06.rs").unwrap().graph;
        dbg!(&graph);
        edge! {graph, A ->   };
        edge! {graph, B -> A };
        redge! {graph, C -> tr!(D) };
        redge! {graph, D -> tr!(E), tr!(F), tr!(G) };
    }

    #[test]
    fn test_ex07() {
        let graph = TypeMap::build("examples/ex07.rs").unwrap().graph;
        edge! {graph, A ->   };
        redge! {graph, B -> tr!(A) };
        edge! {graph, C -> };
    }

    #[test]
    fn test_ex08() {
        let graph = TypeMap::build("examples/ex08.rs").unwrap().graph;
        redge! {graph,  A -> tr!(C) };
        redge! {graph, B -> tr!(C) };
        edge! {graph,  C -> };
    }

    #[test]
    fn test_ex09() {
        let graph = TypeMap::build("examples/ex09.rs").unwrap().graph;
        dbg!(&graph);
    }

    #[test]
    fn test_ex10() {
        let graph = TypeMap::build("examples/ex10.rs").unwrap().graph;
        dbg!(&graph);
        edge! {graph, A -> A}
    }

    #[test]
    fn test_ex11() {
        let graph = TypeMap::build("examples/ex11.rs").unwrap().graph;
        dbg!(&graph);
        redge! {graph, A -> fi!(B), fi!(C), fi!(D),
        fi!(D), fi!(E), fi!(usize),
        fi!(isize), fi!(bool), fi!(f64),
        fi!(F), fi!(G), Dependence::Field("std::collections::HashMap".into()), fi!(H),
        fi!(X), fi!(Y) }; // TODO: X and Y should be tr!
    }
}
