use super::DepGraph;
use graphviz_rust::dot_structures::*;
use graphviz_rust::printer::PrinterContext;
use graphviz_rust::{cmd, exec};

pub fn generate_dot(graph: &DepGraph, outfile: Option<&str>) -> String {
    // Build nodes
    let mut nodes = graph
        .keys()
        .enumerate()
        .map(|(_i, n)| {
            Stmt::Node(Node::new(
                NodeId(Id::Plain(n.to_string()), None),
                vec![Attribute(
                    Id::Plain("shape".into()),
                    Id::Plain("square".into()),
                )],
            ))
        })
        .collect::<Vec<Stmt>>();

    // Build edges
    let edges = graph
        .into_iter()
        .map(|(src, dests)| {
            dests
                .into_iter()
                .map(|dest| {
                    let d = Vertex::N(NodeId(Id::Plain(dest.to_string()), None));
                    Stmt::Edge(Edge {
                        ty: EdgeTy::Pair(Vertex::N(NodeId(Id::Plain(src.to_string()), None)), d),
                        attributes: vec![],
                    })
                })
                .collect::<Vec<Stmt>>()
        })
        .flatten()
        .collect::<Vec<Stmt>>();

    let stmts = {
        nodes.extend(edges);
        nodes
    };

    // Create the graph
    let dotgraph = Graph::DiGraph {
        id: Id::Plain("typemap".into()),
        strict: false,
        stmts,
    };

    // Output to pdf
    let mut ctx = PrinterContext::default();
    if let Some(outf) = outfile {
        let args = vec![
            cmd::CommandArg::Output(outf.into()),
            cmd::CommandArg::Format(cmd::Format::Pdf),
        ];
        exec(dotgraph.clone(), &mut ctx, args).unwrap();
    }

    // Get dot string
    let graph_str = graphviz_rust::print(dotgraph, &mut ctx);
    graph_str
}

#[cfg(test)]
mod test {
    use super::super::TypeMap;
    use super::*;

    #[test]
    fn test_gen() {
        let tm = TypeMap::build("examples/ex06.rs").unwrap();
        generate_dot(tm.graph(), Some("tmp/test.pdf"));
    }

    //#[test]
    #[allow(dead_code)]
    fn tmp_test() {
        let s = "digraph {
    // rankdir=RL
    subgraph template {
        node [shape=square]
        edge [color=black]
        subgraph top {
            node [group=1]
        A
        B
        C
        D
        E
    }
    subgraph bottom {
        node  [group=2]
        F
        G
        H
        }
    }

    C -> c
    F -> f
    subgraph s1 {
        edge [color=red]
        A -> a
        B -> b1
        D -> d1
        E -> e1
        G -> g1
        H -> h1
        }
    subgraph s2 {
        edge [color=blue]
        A -> b1
        B -> a
        D -> d2
        E -> e2
        G -> g2
        H -> h2
    }

    subgraph s3 {
        edge [color=green]
        A -> a
        B -> b1
        D -> d2
        E -> e3
        G -> g3
        H -> h1
    }

    subgraph s4  {
        edge [color=purple]
        A -> b1
        B -> a
        D -> e1
        E -> e2
        G -> g4
        H -> h1
    }
    subgraph s5 {
        edge [ color=orange]
        A -> b1
        B -> a
        D -> d5
        E -> e1
        G -> g5
        H -> h1
    }

    subgraph s6 {
        edge [ color=brown]
        A -> a
        B -> b1
        D -> d1
        E -> e6
        G -> g6
        H -> h1
    }
    subgraph s6 {
        edge [ color=tan]
        A -> a
        B -> b2
        D -> d2
        E -> e6
        G -> g6
        H -> h1
    }
}";

        let g = graphviz_rust::parse(s).unwrap();
        println!("{g:#?}");
    }
}
