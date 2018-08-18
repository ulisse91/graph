use std::collections::HashSet;

use super::Graph;
use std::collections::VecDeque;

////////////////////////////////////////////////////////////////////////////
///////////////////////////////// DFS //////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
fn dfs_visit_counting(g: &mut Graph, v: i32, seen: &mut HashSet<i32>) -> i32 {

	let mut visited: i32 = 1;
	seen.insert(v);

	for neigh_t in g.node(v).out_neigh() {
	    if !seen.contains(&neigh_t.0) {
	    	// print!("{:?} ", neigh_t);
	        visited += dfs_visit_counting(g, neigh_t.0, seen);
	    }
	}
	// print!("| ");
	return visited;
}

fn dfs_visit_cycle(g: &mut Graph, v: i32, seen: &mut HashSet<i32>, parent: &mut Vec<i32>, start: &mut Vec<i32>, end: &mut Vec<i32>, clock: &mut i32) -> bool {
	start[v as usize] = *clock; 
	*clock = *clock + 1;
	seen.insert(v);

	let mut ret: bool = false;


	for neigh_t in g.node(v).out_neigh() {
	    if !seen.contains(&neigh_t.0) {
	    	// print!("{:?} ", neigh_t);
	        ret = dfs_visit_cycle(g, neigh_t.0, seen, parent, start, end, clock);
	        parent[neigh_t.0 as usize] = v;

	    } else {
	    	if start[v as usize] != -1 {
	        	if start[neigh_t.0 as usize] < start[v as usize] {
		        	//if end[neigh_t.0 as usize]==-1 || end[neigh_t.0 as usize] > end[v as usize] {
		        		return true;
		        	// }
	        	}
	        }
	    }

	}
	end[v as usize] = *clock; 
	*clock = *clock + 1;

	return ret;
}

pub fn is_cyclic(g: &mut Graph) -> bool {

    let mut seen: HashSet<i32> = HashSet::new();
    let mut start: Vec<i32> = vec![-1; g.node_count() as usize];
    let mut end: Vec<i32> = vec![-1; g.node_count() as usize];
    let mut parent: Vec<i32> = vec![-1; g.node_count() as usize];
    let mut ret: bool = false;
    let mut clock: i32 = 0;

    for node_t in g.nodes().values() {
        if !seen.contains(&node_t.id) {
        	// print!("{:?}: ", node_t.id);
            ret = dfs_visit_cycle(g, node_t.id, &mut seen, &mut parent, &mut start, &mut end, &mut clock);
            // println!();
            if ret {
           		return true;
            }
        }
    }

    for _x in 0..g.node_count() {
    	print!("{:?} ", start[_x as usize]);
    }println!(); 

    for _x in 0..g.node_count() {
    	print!("{:?} ", end[_x as usize]);
    }println!();

    for _x in 0..g.node_count() {
    	print!("{:?} ", parent[_x as usize]);
    }println!();


    return false;
}

pub fn dfs_from_node(g: &mut Graph, v: i32) -> i32 {

	if g.contains_node(v) {
		let mut seen: HashSet<i32> = HashSet::new();
		// print!("{:?}: ", v);
		let visited: i32 = dfs_visit_counting(g, v, &mut seen);
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
