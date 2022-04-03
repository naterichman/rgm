use crate::repo::Repos;
use std::path::Component;
use indexmap::{IndexMap, IndexSet};

pub trait Sort {
    fn sort(&mut self, repos: Repos) -> Repos;
}

pub struct AlphabeticalSorter;

impl Sort for AlphabeticalSorter {
    fn sort(&mut self, repos: Repos) -> Repos {
        let mut repos = repos;
        repos.repos.sort_by(|a, b| a.name.cmp(&b.name));
        repos
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Incoming,
    Outgoing
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct NodeEdge {
    name: String,
    direction: Direction
}

#[derive(Debug)]
pub struct PathTree {
    nodes: IndexMap<String, IndexSet<NodeEdge>>,
    edges: IndexSet<(String,String)>
}

impl PathTree {
    pub fn with_capacity(nodes: usize) -> Self {
        PathTree {
            nodes: IndexMap::with_capacity(nodes),
            edges: IndexSet::new()
        }
    }

    pub fn add_node(&mut self, node: String) {
        self.nodes.entry(node).or_insert(IndexSet::new());
    }

    pub fn add_edge(&mut self, node_a: String, node_b: String) {
        self.edges.insert((node_a.clone(), node_b.clone()));
        self.nodes
            .entry(node_a.clone())
            .or_insert(IndexSet::with_capacity(1))
            .insert(NodeEdge {
                name: node_b.clone(),
                direction: Direction::Outgoing
            });
        self.nodes
            .entry(node_b.clone())
            .or_insert(IndexSet::with_capacity(1))
            .insert(NodeEdge {
                name: node_a.clone(),
                direction: Direction::Incoming
            });
    }

    pub fn nodes(&self) -> &IndexMap<String, IndexSet<NodeEdge>> {
        &self.nodes
    }

    pub fn edges(&self) -> &IndexSet<(String, String)> {
        &self.edges
    }
}

struct TreeSorter {
    tree: PathTree
}


impl Sort for TreeSorter {
    fn sort(&mut self, repos: Repos) -> Repos{
        for repo in repos.repos.iter() {
            // Build tree
            let components = repo.path
                .components()
                .filter_map(|c| match c {
                    Component::Normal(v) => v.to_str(),
                    _ => None
                })
                .collect::<Vec<&'_ str>>();
            println!("{:?}", components);
            for i in 1..components.len()+1 {
                let parent = components[0..i-1].join("/");
                let node = components[0..i].join("/");
                self.tree.add_edge(parent, node);
            }
        }
        // Use sort_by and the path order from the TreePath
        repos
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    fn tree_with_paths(paths: Vec<PathBuf>) -> PathTree {
        let mut tree = PathTree::with_capacity(3);
        for path in paths {
            let components = path
                .components()
                .filter_map(|c| match c {
                    Component::Normal(v) => v.to_str(),
                    _ => None
                })
                .collect::<Vec<&'_ str>>();
            for i in 1..components.len()+1 {
                let parent = components[0..i-1].join("/");
                let node = components[0..i].join("/");
                tree.add_edge(parent, node);
            }
        }
        tree
    }

    #[test]
    fn test_tree_default(){
        let paths = vec![
            PathBuf::from("/tmp/test/test1"),
            PathBuf::from("/tmp/test1/test1"),
            PathBuf::from("/tmp/test/test2"),
        ];
        let tree = tree_with_paths(paths);
        assert!(tree.nodes().len() == 7);
        assert!(tree.edges().len() == 6);
    }
}
