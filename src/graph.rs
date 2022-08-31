/// Compile time graphs.
/// ```
/// use const_graphs::Graph;
///
/// const SIZE: usize = 1_000;
/// const graph: Graph<SIZE> = Graph::<SIZE>::new();
/// ```
pub struct Graph<const SIZE: usize> {
  data: [[bool; SIZE]; SIZE],
}

impl<const SIZE: usize> Graph<SIZE> {
  /// Add an edge to the graph between `i` and `j`.
  /// ```
  /// use const_graphs::Graph;
  ///
  /// let mut graph = Graph::<10>::new();
  /// graph.add_edge(0, 1);
  /// assert!(graph.has_edge(0, 1));
  /// ```
  /// See also [Graph::add_edge_undirected].
  pub const fn add_edge(&mut self, i: usize, j: usize) {
    self.data[i][j] = true;
  }

  /// Add an undirected edge to the graph between `i` and
  /// `j`.
  /// ```
  /// use const_graphs::Graph;
  ///
  /// let mut graph = Graph::<10>::new();
  /// graph.add_edge_undirected(0, 1);
  /// assert!(graph.has_edge(0, 1));
  /// assert!(graph.has_edge(1, 0));
  /// ```
  pub const fn add_edge_undirected(
    &mut self,
    i: usize,
    j: usize,
  ) {
    self.data[i][j] = true;
    self.data[j][i] = true;
  }

  /// Remove an edge from the graph between `i` and `j`.
  /// ```
  /// use const_graphs::Graph;
  ///
  /// let mut graph = Graph::<10>::new();
  /// graph.add_edge(0, 1);
  /// graph.remove_edge(0, 1);
  /// assert!(!graph.has_edge(0, 1));
  /// ```
  pub const fn remove_edge(&mut self, i: usize, j: usize) {
    self.data[i][j] = false;
  }

  /// Remove an undirected edge from the graph between `i`
  /// and `j`.
  /// ```
  /// use const_graphs::Graph;
  ///
  /// let mut graph = Graph::<10>::new();
  /// graph.add_edge_undirected(0, 1);
  /// graph.remove_edge_undirected(0, 1);
  /// assert!(!graph.has_edge(0, 1));
  /// assert!(!graph.has_edge(1, 0));
  /// ```
  pub const fn remove_edge_undirected(
    &mut self,
    i: usize,
    j: usize,
  ) {
    self.data[i][j] = false;
    self.data[j][i] = false;
  }

  /// Checks whether there is an edge between `i` and `j`.
  /// ```
  /// use const_graphs::Graph;
  ///
  /// let mut graph = Graph::<10>::new();
  /// // The graph is initialized empty.
  /// assert!(!graph.has_edge(0, 1));
  /// ```
  pub const fn has_edge(&self, i: usize, j: usize) -> bool {
    self.data[i][j]
  }

  /// Returns an array where the ith element is a boolean
  /// representing whether there is an edge between `vertex`
  /// and `i`.
  pub const fn get_edges(
    &self,
    vertex: usize,
  ) -> &[bool; SIZE] {
    &self.data[vertex]
  }

  /// Returns the maximum number of edges in a
  /// [directed graph](https://en.wikipedia.org/wiki/Directed_graph),
  /// if your graph is
  /// [undirected graphs](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)#Graph)
  /// you should use [Graph::max_number_of_edges_undirected]
  /// instead.
  ///
  /// Note that the maximum number of edges in a
  /// [directed graph](https://en.wikipedia.org/wiki/Directed_graph)
  /// is always the double of the number of edes in the
  /// equivalent
  /// [undirected graphs](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)#Graph).
  /// ```
  /// use const_graphs::Graph;
  ///
  /// let graph = Graph::<3>::new();
  /// // The possible edges are:
  /// // 0 -> 1
  /// // 0 -> 2
  /// // 1 -> 0
  /// // 1 -> 2
  /// // 2 -> 0
  /// // 2 -> 1
  /// assert_eq!(graph.max_number_of_edges(), 6);
  /// ```
  pub const fn max_number_of_edges(&self) -> usize {
    SIZE * (SIZE - 1)
  }

  /// Returns the density of a graph, that is, the ratio
  /// between the number of edges and the maximum number of
  /// possible edges.
  /// ```
  /// use const_graphs::Graph;
  ///
  /// let mut graph = Graph::<3>::new();
  /// graph.add_edge_undirected(0, 1);
  /// graph.add_edge_undirected(0, 2);
  /// graph.add_edge_undirected(1, 2);
  /// assert_eq!(graph.density(), 1.0);
  /// ```
  pub const fn density(&self) -> f32 {
    let mut edges = 0;

    let mut i = 0;
    while i < SIZE {
      let mut j = 0;
      while j < SIZE {
        if self.data[i][j] {
          edges += 1;
        }

        j += 1;
      }
      i += 1;
    }

    edges as f32 / self.max_number_of_edges() as f32
  }

  /// Remove all edges from the graph.
  /// ```
  /// use const_graphs::Graph;
  ///
  /// let mut graph = Graph::<3>::new();
  /// graph.add_edge_undirected(0, 1);
  /// graph.add_edge_undirected(0, 2);
  /// graph.add_edge_undirected(1, 2);
  /// graph.clear();
  ///
  /// assert_eq!(graph.density(), 0.0);
  /// ```
  pub const fn clear(&mut self) {
    let mut i = 0;
    while i < SIZE {
      let mut j = 0;
      while j < SIZE {
        self.data[i][j] = false;

        j += 1;
      }
      i += 1;
    }
  }

  /// Creates a new graph.
  /// ```
  /// use const_graphs::Graph;
  ///
  /// const SIZE: usize = 10;
  /// let graph = Graph::<SIZE>::new();
  /// ```
  pub const fn new() -> Graph<SIZE> {
    Graph {
      data: [[false; SIZE]; SIZE],
    }
  }
}
