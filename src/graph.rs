pub trait Graph<'a> {
    fn adjacent(&'a self, vertex: usize) -> &'a [usize];
    fn degree(&self, vertex: usize) -> usize;
    fn edge_count(&self) -> usize;
    fn vertex_count(&self) -> usize;
}


pub struct AdjacencyGraph {
    edge_count: usize,
    vertex_count: usize,
    adjacents: Vec<Vec<usize>>
}


impl<'a> Graph<'a> for AdjacencyGraph {
    fn adjacent(&'a self, vertex: usize) -> &'a [usize] {
        &self.adjacents[vertex]
    }

    fn degree(&self, vertex: usize) -> usize {
        self.adjacents[vertex].len()
    }

    fn edge_count(&self) -> usize {
        self.edge_count
    }

    fn vertex_count(&self) -> usize {
        self.vertex_count
    }
}


impl AdjacencyGraph {
    pub fn with_vertex_count(vertex_count: usize) -> AdjacencyGraph {
        let adjacents = vec![vec![]; vertex_count];
        AdjacencyGraph {
            vertex_count: vertex_count,
            edge_count: 0,
            adjacents: adjacents
        }
    }

    pub fn add_edge(&mut self, from_vertex: usize, to_vertex: usize) {
        self.edge_count += 1;
        self.adjacents[from_vertex].push(to_vertex);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_count() {
        let mut target = AdjacencyGraph::with_vertex_count(2);
        target.add_edge(0, 1);
        target.add_edge(1, 1);
        assert_eq!(target.edge_count(), 2);
    }

    #[test]
    fn test_degree() {
        let mut target = AdjacencyGraph::with_vertex_count(4);
        target.add_edge(0, 1);
        target.add_edge(0, 2);
        target.add_edge(0, 3);
        assert_eq!(target.degree(1), 0);
        assert_eq!(target.degree(0), 3);
    }

    #[test]
    fn test_adjacent() {
        let mut target = AdjacencyGraph::with_vertex_count(4);
        target.add_edge(0, 1);
        target.add_edge(0, 2);
        target.add_edge(0, 3);
        target.add_edge(3, 1);
        // test accessing target as a Graph
        fn f<'a, G: Graph<'a>>(graph: &'a G) {
            assert_eq!(graph.adjacent(0), &[1, 2, 3]);
            assert_eq!(graph.adjacent(1), &[]);
            assert_eq!(graph.adjacent(2), &[]);
            assert_eq!(graph.adjacent(3), &[1]);
        };
        f(&target);
    }

    #[test]
    fn test_vertex_count() {
        let target = AdjacencyGraph::with_vertex_count(4);
        assert_eq!(target.vertex_count(), 4);
    }
}
