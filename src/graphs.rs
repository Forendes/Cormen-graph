use priority_queue::PriorityQueue;
#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Graph {
    pub vertices: Vec<GraphVertex>,
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct GraphVertex {
    pub index: usize,
    pub edges: Option<Vec<GraphEdge>>,
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct GraphEdge {
    pub value: usize,
    pub edges: (usize, usize),
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: vec![GraphVertex {
                index: 0,
                edges: None,
            }],
        }
    }

    pub fn add(&mut self) {
        self.vertices.push(GraphVertex {
            index: self.vertices.len(),
            edges: None,
        })
    }

    pub fn connect(&mut self, value: usize, sender: usize, receiver: usize, invert: bool) {
        // invert false => from sender -> to receiver
        // inver true => from receiver -> to sender

        if !invert {
            self.pusher(value, sender, receiver, sender);
            self.pusher(value, sender, receiver, receiver);
        } else {
            self.pusher(value, receiver, sender, sender);
            self.pusher(value, receiver, sender, receiver);
        }
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn breadth_first_search(&self, start_node: usize) -> Vec<usize> {
        let mut queue = vec![&self.vertices[start_node]];
        // make every node white
        // 0 -> white, 1 -> gray, 2 -> black
        // (Node, color) store color of each node
        // edges.1 = pacan => check in colors color of pacana
        let mut colors: Vec<usize> = vec![0; self.vertices.len()];
        colors[start_node] = 1;
        // index of vertex + distance from starting node
        let mut distance: Vec<usize> = vec![0; self.vertices.len()];
        while let Some(current) = queue.pop() {
            for i in current.edges.iter().flat_map(|edges| edges.iter()) {
                // if white
                if colors[i.edges.1] == 0 {
                    colors[i.edges.1] = 1;
                    distance[i.edges.1] = distance[i.edges.0] + 1;
                    queue.push(&self.vertices[i.edges.1]);
                }
            }
            colors[current.index] = 2;
        }
        distance
    }

    pub fn dijkstra(&self, start_node: usize) -> Vec<usize> {
        // distance for best distances each index have currently
        // queue for tracking each possible distance
        let mut distance: Vec<usize> = vec![0; self.vertices.len()];
        let mut queue = PriorityQueue::new();
        queue.push(&self.vertices[start_node], 0);
        while let Some((current, min_value)) = queue.pop() {
            for i in current.edges.iter().flat_map(|edges| edges.iter()) {
                // check if we can get a better result with new distance
                let new_distance = i.value + distance[i.edges.0];
                // check if its outgoing connection
                if i.edges.1 == current.index {
                    continue;
                }
                // if no distance was calculated insert any
                if distance[i.edges.1] == 0 {
                    distance[i.edges.1] = new_distance;
                    queue.push(&self.vertices[i.edges.1], min_value);
                    queue.change_priority(&self.vertices[i.edges.0], new_distance);
                }
                // compare to existing best distance and insert if we got better
                if new_distance < distance[i.edges.1] {
                    distance[i.edges.1] = new_distance;
                    queue.push(&self.vertices[i.edges.1], min_value);
                    queue.change_priority(&self.vertices[i.edges.0], new_distance);
                }
            }
        }
        distance
    }

    fn pusher(&mut self, value: usize, sender: usize, receiver: usize, index: usize) {
        // if node we connect to has no edges, then need to create new edge or else just push new edge to
        // the vec of existing edges

        // receiver have edges
        if let Some(edge) = self.vertices[index].edges.as_mut() {
            edge.push(GraphEdge {
                value,
                edges: (sender, receiver),
            });
        // no edges
        } else {
            self.vertices.get_mut(index).unwrap().edges = Some(vec![GraphEdge {
                value,
                edges: (sender, receiver),
            }])
        }
    }
}
