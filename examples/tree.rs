use std::path::{PathBuf, Component};
use std::ffi::OsString;
use petgraph::graph::Graph;



pub fn main(){
    let mut graph = Graph::<OsString, ()>::new();
    let paths = vec![
        PathBuf::from("/home/naterichman/Downloads"),
        PathBuf::from("/home/naterichman/Documents")
    ];
    for path in paths {
        let mut components = path.components().peekable();
        loop {
            if let (Some(curr_r), Some(next_r)) = (components.next(), components.peek()) {
                if let (Component::Normal(curr), Component::Normal(next)) = (curr_r, next_r) {
                    let c_idx = graph.add_node(curr.to_os_string());
                    let n_idx = graph.add_node(next.to_os_string());
                    graph.add_edge(c_idx, n_idx, ());
                }
            } else {
                break
            }
        }
    }
    
    println!("{:?}", graph)
}
