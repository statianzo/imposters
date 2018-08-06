use std::collections::HashMap;

struct Edge<'a>(&'a str, &'a str, u32);

pub struct Graph<'a> {
    edges: Vec<Edge<'a>>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        Graph { edges: vec![] }
    }

    pub fn add_edge(&mut self, from: &'a str, to: &'a str, weight: u32) {
        self.edges.push(Edge(from, to, weight))
    }

    pub fn paths_from(&self, from: &'a str) -> HashMap<&'a str, u32> {
        let mut paths: HashMap<&'a str, u32> = HashMap::new();
        for Edge(u, v, _) in &self.edges {
            paths.insert(u, u32::max_value());
            paths.insert(v, u32::max_value());
        }
        paths.insert(from, 0);

        for _ in 0..paths.keys().len() {
            for Edge(u, v, weight) in &self.edges {
                if paths[u] != u32::max_value() && paths[u] + weight < paths[v] {
                    let larger = paths[u] + weight;
                    paths.insert(v, larger);
                }
            }
        }

        paths
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_works() {
        let mut _g = Graph::new();
    }

    #[test]
    fn test_add_edge() {
        let mut g = Graph::new();

        g.add_edge("A", "B", 3);
        assert_eq!(g.edges.len(), 1);
    }

    #[test]
    fn test_weight_from() {
        let mut g = Graph::new();

        g.add_edge("A", "B", 3);
        g.add_edge("B", "C", 4);
        g.add_edge("B", "D", 7);
        g.add_edge("C", "D", 1);

        let paths = g.paths_from("A");
        assert_eq!(paths.get("A"), Some(&0));
        assert_eq!(paths.get("B"), Some(&3));
        assert_eq!(paths.get("D"), Some(&8));
    }

    #[test]
    fn test_weight_from_other() {
        let mut g = Graph::new();

        g.add_edge("A", "B", 3);
        g.add_edge("B", "C", 4);
        g.add_edge("B", "D", 7);
        g.add_edge("C", "D", 1);

        let paths = g.paths_from("B");
        assert_eq!(paths.get("B"), Some(&0));
        assert_eq!(paths.get("D"), Some(&5));
        assert_eq!(paths.get("A"), Some(&u32::max_value()));
    }
}
