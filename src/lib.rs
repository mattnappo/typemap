use std::any::TypeId;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use anyhow::Result;
use bimap::BiMap;
use syn;

/// A type in the analyzed codebase
struct Ty {
    /// Type name from std::any::type_name()
    token_name: String,

    /// Type ID from std::any::type_id()
    id: TypeId,

    /// Type name from from the syn token.
    name: String,
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
        dbg!(structs);

        // For each struct s

        // Find the structs it depends on
        // Find the enums it depends on

        // Update the dependents as edges in the graph where the vertex is s

        Ok(Self { graph, resolver })
    }

    fn structs(file: &syn::File) -> Vec<syn::Ident> {
        println!("AST:\n{:#?}", file);

        // All the structs in the file
        file.items
            .clone()
            .into_iter()
            .map(|item| match item {
                syn::Item::Struct(s) => s.ident,
                _ => todo!(),
            })
            .collect::<Vec<syn::Ident>>()
    }
}
