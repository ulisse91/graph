use std::collections::HashMap;

mod algo;

#[derive(Clone)]
pub struct Graph {
	nodes: HashMap<i32, Node>,
	node_count: i32,
	edge_count: i32
}

#[derive(Clone)]
pub struct Node {
	id: i32,
	out_neigh: Vec<(i32, f64)>,
	in_neigh: Vec<(i32, f64)>,
	// out_neigh: Vec<i32>,
	// in_neigh: Vec<i32>,
}

impl Node {
	pub fn insert_out(&mut self, _edge: (i32, f64)) {
		self.out_neigh.push(_edge);
	}

	pub fn insert_in(&mut self, _edge: (i32, f64)) {
		self.in_neigh.push(_edge);
	}

	pub fn remove_out(&mut self, _id: i32) {
		let mut i=0;
		while self.out_neigh[i].0 != _id {
			i+=1;
		}
		self.out_neigh.remove(i);
	}

	pub fn remove_in(&mut self, _id: i32) {
		let mut i=0;
		while self.in_neigh[i].0 != _id {
			i+=1;
		}
		self.in_neigh.remove(i);
	}

	pub fn out_neigh(&mut self) -> Vec<(i32, f64)> {
		return self.out_neigh.clone();
	}

	pub fn in_neigh(&mut self) -> Vec<(i32, f64)> {
		return self.in_neigh.clone();
	}

	pub fn out_neigh_degree(&mut self) -> usize {
		return self.out_neigh.len();
	}

	pub fn in_neigh_degree(&mut self) -> usize {
		return self.in_neigh.len();
	}

	pub fn id(&mut self) -> i32 {
		return self.id;
	}

}

impl Graph {
	pub fn new() -> Graph {
		Graph {
			nodes: HashMap::new(),
			node_count: 0,
			edge_count: 0
		}
	}

	pub fn new_node(&mut self, _id: i32) -> bool {
		if self.contains_node(_id) {
			return false;
		}
		let _node = Node { 
        	id: _id,
			out_neigh: Vec::new(),
			in_neigh: Vec::new()
         };
         self.nodes.insert(_id, _node);
         self.node_count+=1;
         return true;
	}

	pub fn node_count(&mut self) -> i32 {
		return self.node_count;
	}

	pub fn edge_count(&mut self) -> i32 {
		return self.edge_count;
	}

	pub fn contains_node(&mut self, _id: i32) -> bool {
		return self.nodes.contains_key(&_id);		
	}

	pub fn add_edge(&mut self, _id1: i32, _id2: i32, _weight: f64) -> bool {
		if !self.contains_node(_id1) || !self.contains_node(_id2) {
			return false;
		}

		if self.contains_edge(_id1, _id2) {
			return false;
		}

		if let Some(x) = self.nodes.get_mut(&_id1) {
		    x.insert_out((_id2, _weight));
		}

		if let Some(x) = self.nodes.get_mut(&_id2) {
		    x.insert_in((_id1, _weight));
		}

		//finishing...
		self.edge_count+=1;

		return true;
	}

	pub fn remove_edge(&mut self, _id1: i32, _id2: i32) -> bool {
		if !self.contains_node(_id1) || !self.contains_node(_id2) {
			return false;
		}

		if !self.contains_edge(_id1, _id2) {
			return false;
		}

		if let Some(x) = self.nodes.get_mut(&_id1) {
		    x.remove_out(_id2);
		}

		if let Some(x) = self.nodes.get_mut(&_id2) {
		    x.remove_in(_id1);
		}


		//finishing...
		self.edge_count-=1;

		return true;

	}

	pub fn nodes(&mut self) -> HashMap<i32, Node> {
		return self.nodes.clone();
	}

	pub fn node(&mut self, v: i32) -> Node {
		return self.nodes[&v].clone();
	}

	pub fn contains_edge(&mut self, _id1: i32, _id2: i32) -> bool {
		if !self.contains_node(_id1) || !self.contains_node(_id2) {
			return false;
		}
		for _neig in self.node(_id1).out_neigh() {
			if _neig.0 == _id2 {
				return true;
			}
		}
		return false;
	}

	pub fn print_list_adj(&mut self) -> () {
		for tt in self.nodes().values() {
			print!("{:?}: ", tt.id);
			for aa in tt.clone().out_neigh() {
				print!("{:?} ", self.node(aa.0).id);
			} println!("");
		}
	}

	pub fn inverse_graph(&mut self) -> Graph {
		let mut inverse_g = Graph::new();
		let list_nodes = self.nodes();
		for x in list_nodes {
			let mut current_node: Node = x.1.clone();
			inverse_g.new_node(x.0);
			for neigh in current_node.out_neigh() {
				inverse_g.new_node(self.node(neigh.0).id);
				inverse_g.add_edge(neigh.0, current_node.id, neigh.1);
			}
		}
		return inverse_g;
	}

	pub fn remove_node(&mut self, id: i32) -> bool {
		if !self.contains_node(id) {
			return false;
		}

		for neigh in self.node(id).out_neigh() {
			self.remove_edge(id, neigh.0);
		}

		for neigh in self.node(id).in_neigh() {
			self.remove_edge(neigh.0, id);
		}

		self.nodes.remove(&id);
		self.node_count-=1;

		return true;
	}

}

////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

pub fn dfs(g: &mut Graph) {
	algo::dfs(g);
}

pub fn dfs_from_node(g: &mut Graph, v: i32) -> i32 {
	return algo::dfs_from_node(g, v);
}

////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

mod tests {
	use super::*;

    #[test]
	fn add_node_contains_node() {
		let mut g = Graph::new();
		assert_eq!(g.node_count(), 0);
		assert_eq!(g.new_node(0), true);
		assert_eq!(g.node_count(), 1);
		assert_eq!(g.new_node(0), false);
		assert_eq!(g.node_count(), 1);
		assert_eq!(g.contains_node(0), true);
		assert_eq!(g.contains_node(1), false);
		assert_eq!(g.add_edge(0, 1, 1.0), false);
		assert_eq!(g.new_node(1), true);
		assert_eq!(g.contains_node(1), true);
		assert_eq!(g.add_edge(0, 1, 1.0), true);
	}

	#[test]
	fn add_edges_contains_edges() {
		let mut g = Graph::new();
		assert_eq!(g.new_node(0), true);
		assert_eq!(g.new_node(1), true);
		assert_eq!(g.new_node(2), true);
		assert_eq!(g.new_node(3), true);
		assert_eq!(g.add_edge(0, 1, 1.0), true);
		assert_eq!(g.add_edge(0, 2, 1.0), true);
		assert_eq!(g.add_edge(0, 3, 1.0), true);
		assert_eq!(g.contains_edge(0,1), true);
		assert_eq!(g.contains_edge(1, 0), false);
	}

	#[test]
	fn remove_edges() {
		let mut g = Graph::new();
		assert_eq!(g.new_node(0), true);
		assert_eq!(g.new_node(1), true);
		assert_eq!(g.new_node(2), true);
		assert_eq!(g.add_edge(0, 1, 1.0), true);
		assert_eq!(g.add_edge(0, 2, 1.0), true);
		assert_eq!(g.contains_edge(0, 1), true);
		assert_eq!(g.contains_edge(0, 2), true);
		assert_eq!(g.remove_edge(0,3), false);
		assert_eq!(g.remove_edge(1,2), false);
		assert_eq!(g.remove_edge(0,1), true);
		assert_eq!(g.contains_edge(0, 1), false);
	}

	#[test]
	fn inverse_graph() {
		let mut g = Graph::new();
		assert_eq!(g.new_node(1), true);
		assert_eq!(g.new_node(2), true);
		assert_eq!(g.new_node(3), true);
		assert_eq!(g.new_node(4), true);
		assert_eq!(g.add_edge(1, 2, 1.0), true);
		assert_eq!(g.add_edge(1, 3, 1.0), true);
		assert_eq!(g.add_edge(2, 4, 1.0), true);
		assert_eq!(g.add_edge(3, 1, 1.0), true);
		assert_eq!(g.add_edge(3, 4, 1.0), true);
		assert_eq!(g.add_edge(4, 2, 1.0), true);
		let mut inverse_g: Graph = g.inverse_graph();
		assert_eq!(g.node_count(), inverse_g.node_count());
		assert_eq!(g.edge_count(), inverse_g.edge_count());
		assert_eq!(inverse_g.contains_edge(2, 1), true);
		assert_eq!(inverse_g.contains_edge(3, 1), true);
		assert_eq!(inverse_g.contains_edge(4, 2), true);
		assert_eq!(inverse_g.contains_edge(1, 3), true);
		assert_eq!(inverse_g.contains_edge(4, 3), true);
		assert_eq!(inverse_g.contains_edge(2, 4), true);
		assert_eq!(inverse_g.contains_edge(1, 2), false);
		assert_eq!(inverse_g.contains_edge(3, 4), false);
	}

	#[test]
	fn remove_node() {
		let mut g = Graph::new();
		assert_eq!(g.new_node(1), true);
		assert_eq!(g.new_node(2), true);
		assert_eq!(g.new_node(3), true);
		assert_eq!(g.new_node(4), true);
		assert_eq!(g.add_edge(1, 2, 1.0), true);
		assert_eq!(g.add_edge(1, 3, 1.0), true);
		assert_eq!(g.add_edge(2, 4, 1.0), true);
		assert_eq!(g.add_edge(3, 1, 1.0), true);
		assert_eq!(g.add_edge(3, 4, 1.0), true);
		assert_eq!(g.add_edge(4, 2, 1.0), true);
		assert_eq!(g.remove_node(5), false);
		assert_eq!(g.remove_node(1), true);
		assert_eq!(g.contains_node(1), false);
		assert_eq!(g.node_count(), 3);
		assert_eq!(g.edge_count(), 3);
		assert_eq!(g.contains_edge(1, 3), false);
		assert_eq!(g.contains_edge(3, 1), false);
		assert_eq!(g.contains_edge(1, 2), false);
	}
}