use std::collections::HashSet;

use super::Graph;
use std::collections::VecDeque;

////////////////////////////////////////////////////////////////////////////
///////////////////////////////// DFS //////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
fn dfs_visit(g: &mut Graph, v: i32, seen: &mut HashSet<i32>) -> i32 {

	let mut visited: i32 = 1;
	seen.insert(v);

	for neigh_t in g.node(v).out_neigh() {
	    if !seen.contains(&neigh_t.0) {
	    	// print!("{:?} ", neigh_t);
	        visited += dfs_visit(g, neigh_t.0, seen);
	    }
	}
	// print!("| ");
	return visited;
}


pub fn dfs(g: &mut Graph) {

    let mut seen: HashSet<i32> = HashSet::new();

    for node_t in g.nodes().values() {
        if !seen.contains(&node_t.id) {
        	// print!("{:?}: ", node_t.id);
            dfs_visit(g, node_t.id, &mut seen);
            // println!();
        }
    }
}

pub fn dfs_from_node(g: &mut Graph, v: i32) -> i32 {

	if g.contains_node(v) {
		let mut seen: HashSet<i32> = HashSet::new();
		// print!("{:?}: ", v);
		let visited: i32 = dfs_visit(g, v, &mut seen);
		// print!(" - {:?}", visited);
		// println!();
		return visited;
	}
	return 0;	
}

////////////////////////////////////////////////////////////////////////////
///////////////////////////////// BFS //////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

pub fn has_path_connecting(g: &mut Graph, from: i32, to: i32) -> bool {

	let mut queue: VecDeque<i32> = VecDeque::new();
	let mut seen: HashSet<i32> = HashSet::new();

	queue.push_front(from);
	seen.insert(from);

	while !queue.is_empty() {
        let p: i32 = queue.pop_front().unwrap();

        // stop expanding if reached target point
        if p == to {
            return true;
        }

        for neigh_t in g.node(p).out_neigh() {
        	if !seen.contains(&neigh_t.0) {
        		print!("{:?} ", neigh_t.0);
        		queue.push_back(neigh_t.0);
        		seen.insert(neigh_t.0);
        	}
        } println!();
    }
    return false;
}

////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
