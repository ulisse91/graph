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
	out_neigh: Vec<i32>,
	in_neigh: Vec<i32>,
	out_neigh_degree: i32,
	in_neigh_degree: i32
}

impl Node {
	pub fn insert_out(&mut self, _id: i32) {
		self.out_neigh.push(_id);
		self.out_neigh_degree+=1;
	}

	pub fn insert_in(&mut self, _id: i32) {
		self.in_neigh.push(_id);
		self.in_neigh_degree+=1;
	}

	pub fn out_neigh(&mut self) -> Vec<i32> {
		return self.out_neigh.clone();
	}

	pub fn in_neigh(&mut self) -> Vec<i32> {
		return self.in_neigh.clone();
	}

	pub fn out_neigh_degree(&mut self) -> i32 {
		return self.out_neigh_degree;
	}

	pub fn in_neigh_degree(&mut self) -> i32 {
		return self.in_neigh_degree;
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
			in_neigh: Vec::new(),
			out_neigh_degree: 0,
			in_neigh_degree: 0
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

	pub fn add_edge(&mut self, _id1: i32, _id2: i32) -> bool {
		if !self.contains_node(_id1) || !self.contains_node(_id2) {
			return false;
		}

		if self.contains_edge(_id1, _id2) {
			return false;
		}

		if let Some(x) = self.nodes.get_mut(&_id1) {
		    x.insert_out(_id2);
		}

		if let Some(x) = self.nodes.get_mut(&_id2) {
		    x.insert_in(_id1);
		}


		//finishing...
		self.edge_count+=1;

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
			if _neig == _id2 {
				return true;
			}
		}
		return false;
	}

	pub fn print_list_adj(&mut self) -> () {
		for tt in self.nodes().values() {
			print!("{:?}: ", tt.id);
			for aa in tt.clone().out_neigh() {
				print!("{:?} ", self.node(aa).id);
			} println!("");
		}
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
	fn test1() {
		let mut g = Graph::new();
		assert_eq!(g.node_count(), 0);
		assert_eq!(g.new_node(0), true);
		assert_eq!(g.node_count(), 1);
		assert_eq!(g.new_node(0), false);
		assert_eq!(g.node_count(), 1);
		assert_eq!(g.contains_node(0), true);
		assert_eq!(g.contains_node(1), false);
		assert_eq!(g.add_edge(0, 1), false);
		assert_eq!(g.new_node(1), true);
		assert_eq!(g.contains_node(1), true);
		assert_eq!(g.add_edge(0, 1), true);
	}

	#[test]
	fn test2() {
		let mut g = Graph::new();
		assert_eq!(g.new_node(0), true);
		assert_eq!(g.new_node(1), true);
		assert_eq!(g.new_node(2), true);
		assert_eq!(g.new_node(3), true);
		assert_eq!(g.add_edge(0, 1), true);
		assert_eq!(g.add_edge(0, 2), true);
		assert_eq!(g.add_edge(0, 3), true);
		assert_eq!(g.contains_edge(0,1), true);
		assert_eq!(g.contains_edge(1, 0), false);
	}
}