use std::collections::HashSet;

use super::Graph;

////////////////////////////////////////////////////////////////////////////
///////////////////////////////// DFS //////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
fn dfs_visit(g: &mut Graph, v: i32, seen: &mut HashSet<i32>) -> i32 {

	let mut visited: i32 = 1;
	seen.insert(v);

	for neigh_t in g.node(v).out_neigh() {
	    if !seen.contains(&neigh_t) {
	    	// print!("{:?} ", neigh_t);
	        visited += dfs_visit(g, neigh_t, seen);
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
////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////