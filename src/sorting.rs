use crate::repo::Repos;
use std::path::Component;
use std::collections::HashMap;
use std::ffi::OsStr;

pub trait Sort {
    fn sort(&mut self, repos: &mut Repos);
}

pub struct AlphabeticalSorter;

impl Sort for AlphabeticalSorter {
    fn sort(&mut self, repos: &mut Repos){
        repos.repos.sort_by(|a, b| a.name.cmp(&b.name));
    }
}

#[derive(Debug)]
pub struct TreeNode<'a> {
    children: HashMap<&'a OsStr, Box<TreeNode<'a>>>
}

impl<'a> TreeNode<'a> {
    fn new() -> Self {
        Self {
            children: HashMap::<&'a OsStr, Box<TreeNode<'a>>>::new()
        }
    }
}

#[derive(Debug)]
pub struct TreeSorter<'a> {
    // TODO
    tree: Box<TreeNode<'a>>
}

impl<'a> TreeSorter<'a> {
    pub fn new() -> Self{
        Self {
            tree: Box::new(TreeNode::new())
        }
    }

    fn add_path(&mut self, components: Vec::<Component>) {
        let mut curr_node = *self.tree;
        for component in components {
            if let Component::Normal(k) = component {
                if let Some(child) = curr_node.children.get_mut(k){
                    curr_node = **child;        
                } else {
                    curr_node.children.insert(k, Box::new(TreeNode::new()));
                }
            }
        }
    }
}


impl<'a> Sort for TreeSorter<'a> {
    fn sort(&mut self, repos: &mut Repos){
        for repo in repos.repos {
            // Build tree
            let path_components: Vec::<Component> = repo.path.components().collect();
        }
    }
}
