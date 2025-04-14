use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::dot::{Dot, Config};
use std::fs::File;
use std::io::Write;

pub struct MyGraph {
    graph: Graph<&'static str, ()>,
    node_indices: Vec<NodeIndex>, // Para guardar os índices dos nós
}

impl MyGraph {
    pub fn new() -> Self {
        let mut file = File::create("src/graph.dot").unwrap();
        let mut graph = Graph::<&'static str, ()>::new();

        // Adicionando nós
        let n1 = graph.add_node("1");
        let n2 = graph.add_node("2");
        let n3 = graph.add_node("3");
        let n4 = graph.add_node("4");
        let n5 = graph.add_node("5");
        let n6 = graph.add_node("6");

        // Conectando nós com arestas (exemplo, você pode mudar)
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        graph.add_edge(n3, n4, ());
        graph.add_edge(n1, n5, ());
        graph.add_edge(n5, n6, ());

        // Gerar o arquivo .dot
        let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
        let mut file = File::create("graph.dot").unwrap();
        file.write_all(format!("{:?}", dot).as_bytes()).unwrap();

        Self {
            graph,
            node_indices: vec![n1, n2, n3, n4, n5, n6],
        }
    }

    pub fn dfs_from_node1(&self) -> Vec<&'static str> {
        let mut dfs = Dfs::new(&self.graph, self.node_indices[0]); // Começa no nó "1"
        let mut visited = vec![];

        while let Some(nx) = dfs.next(&self.graph) {
            visited.push(self.graph[nx]); // Pega o label do nó
        }

        visited
    }
}
