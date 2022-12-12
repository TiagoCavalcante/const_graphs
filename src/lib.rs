//! Blazingly-fast compile-time copy-free graph library.
//! ```
//! use const_graphs::Graph;
//!
//! fn bfs<const SIZE: usize>(
//!   graph: &Graph<SIZE>,
//!   start: usize,
//!   end: usize,
//! ) -> Option<Vec<usize>> {
//!   let mut queue = std::collections::VecDeque::new();
//!
//!   let mut distance = [usize::MAX; SIZE];
//!   let mut predecessor = [usize::MAX; SIZE];
//!
//!   distance[start] = 0;
//!
//!   queue.push_back(start);
//!
//!   while let Some(current) = queue.pop_front() {
//!     for (neighbor, &has_edge) in graph
//!                                      .get_edges(current)
//!                                      .iter()
//!                                      .enumerate() {
//!       if has_edge && distance[neighbor] == usize::MAX {
//!         distance[neighbor] = distance[current] + 1;
//!         predecessor[neighbor] = current;
//!         queue.push_back(neighbor);
//!
//!         if neighbor == end {
//!           let mut path = vec![end];
//!           let mut current = end;
//!           while predecessor[current] != usize::MAX {
//!             current = predecessor[current];
//!             path.push(current);
//!           }
//!
//!           path.reverse();
//!
//!           return Some(path);
//!         }
//!       }
//!     }
//!   }
//!
//!   return None;
//! }
//! 
//! use const_graphs::WeightedGraph;
//!
//! fn bfs_weighted<const SIZE: usize>(
//!   graph: &WeightedGraph<SIZE>,
//!   start: usize,
//!   end: usize,
//! ) -> Option<Vec<usize>> {
//!   let mut queue = std::collections::VecDeque::new();
//!
//!   let mut distance = [usize::MAX; SIZE];
//!   let mut predecessor = [usize::MAX; SIZE];
//!
//!   distance[start] = 0;
//!
//!   queue.push_back(start);
//!
//!   while let Some(current) = queue.pop_front() {
//!     for (neighbor, &edge) in graph
//!                                      .get_edges(current)
//!                                      .iter()
//!                                      .enumerate() {
//!       if edge.is_some() &&
//!          distance[neighbor] == usize::MAX {
//!         distance[neighbor] = distance[current] + 1;
//!         predecessor[neighbor] = current;
//!         queue.push_back(neighbor);
//!
//!         if neighbor == end {
//!           let mut path = vec![end];
//!           let mut current = end;
//!           while predecessor[current] != usize::MAX {
//!             current = predecessor[current];
//!             path.push(current);
//!           }
//!
//!           path.reverse();
//!
//!           return Some(path);
//!         }
//!       }
//!     }
//!   }
//!
//!   return None;
//! }
//! ```

#![feature(const_mut_refs)]
#![feature(const_fn_floating_point_arithmetic)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::bare_urls)]

mod graph;
mod weighted_graph;

pub use self::graph::Graph;
pub use self::weighted_graph::WeightedGraph;
