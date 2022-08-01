use crate::graph::Graph;

pub trait Path {
  fn _find_paths(
    &self,
    start: usize,
    end: usize,
    time: usize,
    remaining_vertices: &mut Vec<usize>,
    path: &mut Vec<usize>,
		paths: &mut Vec<Vec<usize>>
  ) -> Vec<Vec<usize>>;

  fn find_paths(
    &self,
    start: usize,
    end: usize,
    time: usize,
  ) -> Vec<Vec<usize>>;
}

impl Path for Graph {
  fn _find_paths(
    &self,
    start: usize,
    end: usize,
    time: usize,
    remaining_vertices: &mut Vec<usize>,
    path: &mut Vec<usize>,
		paths: &mut Vec<Vec<usize>>
  ) -> Vec<Vec<usize>> {
    let last = remaining_vertices.len() - 1;

    for i in 0..=last {
      let vertex = remaining_vertices[i];

      if self.get(start, vertex) {
        path.push(vertex);

        if vertex == end {
          paths.push(path.clone());
        } else if time > 0 {
          // Swap the current element with the last and
          // decrement the length.
          remaining_vertices.swap_remove(i);

          self._find_paths(
            vertex,
            end,
            time - 1,
            remaining_vertices,
            path,
						paths
          );

          // Revert our changes.
          remaining_vertices.push(vertex);
          remaining_vertices.swap(i, last);
        }

        path.pop();
      }
    }

    paths.to_vec()
  }

  fn find_paths(
    &self,
    start: usize,
    end: usize,
    time: usize,
  ) -> Vec<Vec<usize>> {
    let mut remaining_vertices = vec![];
    for i in 0..self.size {
      if i != start {
        remaining_vertices.push(i);
      }
    }

    let mut path = vec![start];

		let mut paths = vec![];

    self._find_paths(
      start,
      end,
      time,
      &mut remaining_vertices,
      &mut path,
			&mut paths
    )
  }
}
