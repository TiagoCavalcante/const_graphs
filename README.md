# const_graphs

Blazingly-fast compile-time no-std graph [crate](https://crates.io/crates/const_graphs)

## Quickstart

Add this line bellow `[dependencies]` in your `Cargo.toml` file:
```
const_graphs = "*"
```

And to create a graph:
```rs
use const_graphs::Graph;
use const_graphs::WeightedGraph;

const SIZE: usize = 500;
let mut graph = Graph::<SIZE>::new();

const SIZE: usize = 500;
let mut graph = WeightedGraph::<SIZE>::new();
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

  let mut distance = [usize::MAX; SIZE];
  let mut predecessor = [usize::MAX; SIZE];

  distance[start] = 0;

  queue.push_back(start);

  while let Some(current) = queue.pop_front() {
    for (neighbor, &has_edge) in graph
                                     .get_edges(current)
                                     .iter()
                                     .enumerate() {
      if has_edge && distance[neighbor] == usize::MAX {
        distance[neighbor] = distance[current] + 1;
        predecessor[neighbor] = current;
        queue.push_back(neighbor);

        if neighbor == end {
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

use const_graphs::WeightedGraph;

fn bfs_weighted<const SIZE: usize>(
  graph: &WeightedGraph<SIZE>,
  start: usize,
  end: usize,
) -> Option<Vec<usize>> {
  let mut queue = std::collections::VecDeque::new();

  let mut distance = [usize::MAX; SIZE];
  let mut predecessor = [usize::MAX; SIZE];

  distance[start] = 0;

  queue.push_back(start);

  while let Some(current) = queue.pop_front() {
    for (neighbor, &edge) in graph
                                     .get_edges(current)
                                     .iter()
                                     .enumerate() {
      if edge.is_some() &&
         distance[neighbor] == usize::MAX {
        distance[neighbor] = distance[current] + 1;
        predecessor[neighbor] = current;
        queue.push_back(neighbor);

        if neighbor == end {
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
