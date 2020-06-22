pub mod graph {
    use std::collections::HashMap;
    use graph_items::node::Node;
    use graph_items::edge::Edge;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    } 

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges.append(&mut edges.to_vec());
            self
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes.append(&mut nodes.to_vec());
            self
        }

        pub fn with_attrs(mut self, tup_vec: &[(&str, &str)]) -> Self {
            for (key, val) in tup_vec.into_iter() {
                self.attrs.insert(key.to_string(), val.to_string());
            }
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
    }


    pub mod graph_items{
        use super::{HashMap};

        pub mod node {
            use super::{HashMap};

            #[derive(PartialEq, Debug, Clone)]
            pub struct Node{
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, tup_vec: &[(&str, &str)]) -> Self {
                    for (key, val) in tup_vec.into_iter() {
                        self.attrs.insert(key.to_string(), val.to_string());
                    }
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).and_then(|val| Some(&val[..]))
                }
            }
        }

        pub mod edge {
            use super::{HashMap};

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                label1: String,
                label2: String,
                pub attrs: HashMap<String, String>,
            }
    
            impl Edge {
                pub fn new(val1:&str, val2:&str) -> Self {
                    Edge {
                        label1: val1.to_string(),
                        label2: val2.to_string(),
                        attrs: HashMap::new(),
                    }
                }
    
                pub fn with_attrs(mut self, tup_vec: &[(&str, &str)]) -> Self {
                    for (key, val) in tup_vec.into_iter() {
                        self.attrs.insert(key.to_string(), val.to_string());
                    }
                    self
                }
            }
        }
    }
}