# graph
Simple graph implementation in rust

## Examples

1. Create a new graph
```
let mut g = Graph::new();

g.new_node(0);
g.new_node(1);
g.new_node(2);

g.add_edge(0, 1);
g.add_edge(1, 2);
g.add_edge(2, 1);
```
