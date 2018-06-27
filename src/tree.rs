#![allow(unused)]
#[derive(Debug)]
struct Node<'a>(&'a str, Vec<Node<'a>>);

impl<'a> Node<'a> {
    fn from_path(path: &'a str) -> Self {
        let parts = path.split("/").collect::<Vec<&str>>();
        Node(&parts[0], vec![])
    }
}

#[test]
fn test_it_works() {
    let t = Node("home", vec![]);
    let Node(name, children) = t;
    assert_eq!(name, "home");
    assert_eq!(children.len(), 0);
}

#[test]
fn parse_single_node() {
    let path = "home";
    let result = Node::from_path(path);
    assert_eq!(result.0, "home");
    assert_eq!(result.1.len(), 0);
}

#[test]
fn parse_multi_node() {
    let path = "home/music/hip_hop";
    let result = Node::from_path(path);
    assert_eq!(result.0, "home");
    assert_eq!(result.1.len(), 0);
}
