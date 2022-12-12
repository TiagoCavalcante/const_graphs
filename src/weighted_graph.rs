/// Compile time graphs.
/// ```
/// use const_graphs::WeightedGraph;
///
/// const SIZE: usize = 500;
/// // You can use const.
/// const graph1: WeightedGraph<SIZE>
///   = WeightedGraph::new();
///
/// // And, static.
/// static mut graph2: WeightedGraph<SIZE>
///   = WeightedGraph::new();
/// 
/// unsafe {
///   graph2.add_edge(0, 1, 0.1);
///   assert!(graph2.has_edge(0, 1));
/// }
///
/// // And, of course, let too:
/// let graph3 = WeightedGraph::<SIZE>::new();
/// ```
pub struct WeightedGraph<const SIZE: usize> {
  data: [[Option<f32>; SIZE]; SIZE],
}

impl<const SIZE: usize> WeightedGraph<SIZE> {
  /// Add an edge to the graph between `i` and `j`.
  /// ```
  /// use const_graphs::WeightedGraph;
  ///
  /// let mut graph = WeightedGraph::<10>::new();
  /// graph.add_edge(0, 1, 0.7);
  /// assert!(graph.has_edge(0, 1));
  /// ```
  /// See also [WeightedGraph::add_edge_undirected].
  pub const fn add_edge(
    &mut self,
    i: usize,
    j: usize,
    weight: f32,
  ) {
    self.data[i][j] = Some(weight);
  }

  /// Add an undirected edge to the graph between `i` and
  /// `j`.
  /// ```
  /// use const_graphs::WeightedGraph;
  ///
  /// let mut graph = WeightedGraph::<10>::new();
  /// graph.add_edge_undirected(0, 1, 3.0);
  /// assert!(graph.has_edge(0, 1));
  /// assert!(graph.has_edge(1, 0));
  /// ```
  /// See also [WeightedGraph::add_edge].
  pub const fn add_edge_undirected(
    &mut self,
    i: usize,
    j: usize,
    weight: f32,
  ) {
    self.data[i][j] = Some(weight);
    self.data[j][i] = Some(weight);
  }

  /// Remove an edge from the graph between `i` and `j`.
  /// ```
  /// use const_graphs::WeightedGraph;
  ///
  /// let mut graph = WeightedGraph::<10>::new();
  /// graph.add_edge(0, 1, 0.3);
  /// graph.remove_edge(0, 1);
  /// assert!(!graph.has_edge(0, 1));
  /// ```
  /// See also [WeightedGraph::remove_edge_undirected].
  pub const fn remove_edge(&mut self, i: usize, j: usize) {
    self.data[i][j] = None;
  }

  /// Remove an undirected edge from the graph between `i`
  /// and `j`.
  /// ```
  /// use const_graphs::WeightedGraph;
  ///
  /// let mut graph = WeightedGraph::<10>::new();
  /// graph.add_edge_undirected(0, 1, 0.4);
  /// graph.remove_edge_undirected(0, 1);
  /// assert!(!graph.has_edge(0, 1));
  /// assert!(!graph.has_edge(1, 0));
  /// ```
  /// See also [WeightedGraph::remove_edge].
  pub const fn remove_edge_undirected(
    &mut self,
    i: usize,
    j: usize,
  ) {
    self.data[i][j] = None;
    self.data[j][i] = None;
  }

  /// Gets the optional edge between `i` and `j`.
  /// ```
  /// use const_graphs::WeightedGraph;
  ///
  /// let mut graph = WeightedGraph::<10>::new();
  /// graph.add_edge(0, 1, 16.0);
  /// assert_eq!(graph.get_edge(0, 1), Some(16.0));
  /// ```
  pub const fn get_edge(
    &self,
    i: usize,
    j: usize,
  ) -> Option<f32> {
    self.data[i][j]
  }

  /// Checks whether there is an edge between `i` and `j`.
  /// ```
  /// use const_graphs::WeightedGraph;
  ///
  /// let mut graph = WeightedGraph::<10>::new();
  /// // The graph is initialized empty.
  /// assert!(!graph.has_edge(0, 1));
  /// ```
  pub const fn has_edge(&self, i: usize, j: usize) -> bool {
    self.data[i][j].is_some()
  }

  /// Returns an array where the ith element is the optional
  /// edge between `vertex` and `i`.
  /// ```
  /// use const_graphs::WeightedGraph;
  ///
  /// let mut graph = WeightedGraph::<3>::new();
  /// graph.add_edge(0, 2, 2.3);
  /// assert_eq!(
	///   graph.get_edges(0),
	///   &[None, None, Some(2.3)]
	/// );
  /// ```
  /// See also [WeightedGraph::get_inverse_edges].
  pub const fn get_edges(
    &self,
    vertex: usize,
  ) -> &[Option<f32>; SIZE] {
    &self.data[vertex]
  }

  /// Returns an array where the ith element is the optional
  /// edge between `i` and `vertex`.
  /// This is useful in a few graph algorithms where you
  /// need to know which vertices "point" to the current,
  /// and not the contrary.
  /// ```
  /// use const_graphs::WeightedGraph;
  ///
  /// let mut graph = WeightedGraph::<3>::new();
  /// graph.add_edge(0, 2, 0.8);
  /// assert_eq!(
  ///   graph.get_inverse_edges(2),
  ///   [Some(0.8), None, None]
  /// );
  /// ```
  /// See also [WeightedGraph::get_edges].
  pub const fn get_inverse_edges(
    &self,
    vertex: usize,
  ) -> [Option<f32>; SIZE] {
    let mut edges = [None; SIZE];

    let mut neighbor = 0;
    while neighbor < SIZE {
      edges[neighbor] = self.data[neighbor][vertex];

      neighbor += 1;
    }

    edges
  }

  /// Returns the maximum number of edges of a graph.
  /// ```
  /// use const_graphs::WeightedGraph;
  ///
  /// let graph = WeightedGraph::<3>::new();
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
  /// use const_graphs::WeightedGraph;
  ///
  /// let mut graph = WeightedGraph::<3>::new();
  /// graph.add_edge_undirected(0, 1, 0.1);
  /// graph.add_edge_undirected(0, 2, 1.1);
  /// graph.add_edge_undirected(1, 2, 0.5);
  /// assert_eq!(graph.density(), 1.0);
  /// ```
  pub const fn density(&self) -> f32 {
    let mut edges = 0;

    let mut i = 0;
    while i < SIZE {
      let mut j = 0;
      while j < SIZE {
        if self.data[i][j].is_some() {
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
  /// use const_graphs::WeightedGraph;
  ///
  /// let mut graph = WeightedGraph::<3>::new();
  /// graph.add_edge_undirected(0, 1, 0.2);
  /// graph.add_edge_undirected(0, 2, 0.6);
  /// graph.add_edge_undirected(1, 2, 5.5);
  /// graph.clear();
  ///
  /// assert_eq!(graph.density(), 0.0);
  /// ```
  pub const fn clear(&mut self) {
    let mut i = 0;
    while i < SIZE {
      let mut j = 0;
      while j < SIZE {
        self.data[i][j] = None;

        j += 1;
      }
      i += 1;
    }
  }

  /// Creates a new weighted graph.
  /// ```
  /// use const_graphs::WeightedGraph;
  ///
  /// const SIZE: usize = 10;
  /// let graph = WeightedGraph::<SIZE>::new();
  /// ```
  pub const fn new() -> WeightedGraph<SIZE> {
    WeightedGraph {
      data: [[None; SIZE]; SIZE],
    }
  }
}
