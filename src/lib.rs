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
#[derive(Default)]
pub struct TypeMap {
    /// Map from a `Ty` to the `Ty`s it depends on
    graph: HashMap<Ty, Vec<Ty>>,

    /// Bijective map from type names to type IDs
    resolver: BiMap<String, TypeId>,
}

impl TypeMap {
    /// Build a `TypeMap` from a single file.
    /// Eventually will support multi-file projects.
    pub fn build(src: &str) -> Result<Self> {
        let mut graph = HashMap::new();
        let mut resolver = BiMap::new();

        // Parse the file
        let mut fd = File::open(src)?;
        let mut file = String::new();
        fd.read_to_string(&mut file)?;
        let file = syn::parse_file(&file)?;

        // Find all the user-defined structs and populate `resolver`

        let structs = Self::structs(&file);
        dbg!(&structs);

        for (type_name, s) in structs {
            let d = Self::dependents(&s);
            println!("{type_name} depends on:");
            dbg!(d);
        }

        // For each struct s

        // Find the structs it depends on
        // Find the enums it depends on

        // Update the dependents as edges in the graph where the vertex is s

        Ok(Self { graph, resolver })
    }

    fn structs(file: &syn::File) -> Vec<(String, Fields)> {
        println!("AST:\n{:#?}", file);

        // All the structs in the file
        file.items
            .clone()
            .into_iter()
            .map(|item| match item {
                Item::Struct(s) => (s.ident.to_string(), s.fields),
                _ => todo!(),
            })
            .collect::<Vec<(String, Fields)>>()
    }

    fn dependents(s: &Fields) -> Vec<Ident> {
        match s {
            Fields::Unit => vec![],
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
        }
    }

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

    macro_rules! test_example {
        ($name:ident, $path:expr) => {
            #[test]
            fn $name() {
                TypeMap::build($path).unwrap();
            }
        };
    }

    test_example!(test_ex1, "examples/ex1.rs");
    test_example!(test_ex2, "examples/ex2.rs");
    test_example!(test_ex3, "examples/ex3.rs");
}
