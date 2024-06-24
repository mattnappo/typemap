use super::DepGraph;
use graphviz_rust::dot_structures::*;
use graphviz_rust::printer::PrinterContext;
use graphviz_rust::{cmd, exec};

pub fn generate_dot(graph: &DepGraph) {
    // Build nodes
    let mut nodes = graph
        .keys()
        .enumerate()
        .map(|(_i, n)| Stmt::Node(Node::new(NodeId(Id::Plain(n.to_string()), None), vec![])))
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

    nodes.extend(edges);

    // Collect into a vec of statements

    // Create the graph
    let dotgraph = Graph::DiGraph {
        id: Id::Plain("typemap".into()),
        strict: false,
        stmts: nodes,
    };

    // Output to pdf
    let mut ctx = PrinterContext::default();
    let args = vec![
        cmd::CommandArg::Output("test.pdf".into()),
        cmd::CommandArg::Format(cmd::Format::Pdf),
    ];
    exec(dotgraph.clone(), &mut ctx, args).unwrap();

    let graph_str = graphviz_rust::print(dotgraph, &mut ctx);
    println!("{}", graph_str)
    // println!("{nodes:?}");
}

#[cfg(test)]
mod test {
    use super::super::TypeMap;
    use super::*;

    #[test]
    fn test_gen() {
        let tm = TypeMap::build("examples/ex06.rs").unwrap();
        generate_dot(tm.graph());
    }
}
