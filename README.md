# const_graphs

Blazingly-fast compile-time const-fns copy-free unweighted graph library

## Quickstart

Add this line bellow `[dependencies]` in your `Cargo.toml` file:
```
const_graphs = { git = "https://github.com/TiagoCavalcante/const_graphs" }
```

And to create a graph:
```rs
use const_graphs::Graph;

const SIZE: usize = 10_100;
let mut graph = Graph::<SIZE>::new();
```

Here is an implementation of the
[BFS](https://en.wikipedia.org/wiki/Breadth-first_search)
algorithm:
```rs
use const_graphs::Graph;

fn bfs<const SIZE: usize>(
  graph: &Graph<SIZE>,
  start: usize,
  end: usize,
) -> Option<Vec<usize>> {
  let mut queue = std::collections::VecDeque::new();

  let mut distance = vec![usize::MAX; SIZE];
  let mut predecessor = vec![usize::MAX; SIZE];

  distance[start] = 0;

  queue.push_back(start);

  while let Some(current) = queue.pop_front() {
    for &vertex in graph.get_neighbors(current) {
      if distance[vertex] == usize::MAX {
        distance[vertex] = distance[current] + 1;
        predecessor[vertex] = current;
        queue.push_back(vertex);

        if vertex == end {
          let mut path = vec![end];
          let mut current = end;
          while predecessor[current] != usize::MAX {
            current = predecessor[current];
            path.push(current);
          }

          path.reverse();

          return Some(path);
        }
      }
    }
  }

  return None;
}
```
