
pub struct Graph<T> {
    nodes: Vec<NodeData<T>>,
    edges: Vec<EdgeData>,
}

pub type NodeIndex = usize;

#[derive(Debug)]
pub struct NodeData<T> {
    out: Option<EdgeIndex>,
    pub node: T
}


pub type EdgeIndex = usize;

pub struct EdgeData {
    target: NodeIndex,
    next_edge: Option<EdgeIndex>
}

impl <N : PartialEq> Graph<N> {
   
    pub fn node_at(&self, index: NodeIndex) -> &NodeData<N> {
        &self.nodes[index]
    }
    
    pub fn add_node(&mut self, node: N) -> NodeIndex {
        if let Some(existing_index) = self.find_node(&node) {
            existing_index
        } else {
            let index = self.nodes.len();
            self.nodes.push(NodeData { out: None, node });
            index
        }
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let mut source_node = &mut self.nodes[source];
        self.edges.push(EdgeData { target, next_edge: source_node.out });
        source_node.out = Some(edge_index);
    }

    pub fn successors(&self, source: NodeIndex) -> Successors<N> {
        let first_outgoing_edge = self.nodes[source].out;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }
    
    pub fn find_node(&self, node: &N) -> Option<NodeIndex> {
        self.nodes.iter().enumerate().filter_map(|(i,n)| if *node == n.node { Some(i as NodeIndex) } else { None }).nth(0)
    }
}

pub struct Successors<'graph, T> {
    graph: &'graph Graph<T>,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph, T> Iterator for Successors<'graph, T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_edge;
                Some(edge.target)
            }
        }
    }
}

impl <T> Default for Graph<T> {
    fn default() -> Self {
        Graph { nodes: vec![], edges: vec![] }
    }
}