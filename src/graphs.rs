extern crate priority_queue;
use priority_queue::PriorityQueue;
#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Graph {
    pub vertices: Vec<GraphVertex>,
}

pub struct GraphNode {
    pub neighbors: Vec<Box<(GraphVertex, u32)>>,
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

    pub fn connect(&mut self, value: usize, sender: (usize, bool), receiver: usize) {
        // false => from sender -> to receiver
        // true => from receiver -> to sender
        if !sender.1 {
            // if node we connect to has no edges, then need to create new edge or else just push new edge to
            // the vec of existing edges

            // receiver have no edges
            if self
                .vertices
                .get_mut(sender.0)
                .unwrap()
                .edges
                .as_mut()
                .is_none()
            {
                self.vertices.get_mut(sender.0).unwrap().edges = Some(vec![GraphEdge {
                    value,
                    edges: (receiver, sender.0),
                }])
            // receiver have edges
            } else {
                self.vertices
                    .get_mut(sender.0)
                    .unwrap()
                    .edges
                    .as_mut()
                    .unwrap()
                    .push(GraphEdge {
                        value,
                        edges: (receiver, sender.0),
                    });
            }
            // same thing for node from whom connection is going

            // sender have no edges
            if self
                .vertices
                .get_mut(receiver)
                .unwrap()
                .edges
                .as_mut()
                .is_none()
            {
                self.vertices.get_mut(receiver).unwrap().edges = Some(vec![GraphEdge {
                    value,
                    edges: (receiver, sender.0),
                }])
            // sender have edges
            } else {
                self.vertices
                    .get_mut(receiver)
                    .unwrap()
                    .edges
                    .as_mut()
                    .unwrap()
                    .push(GraphEdge {
                        value,
                        edges: (receiver, sender.0),
                    });
            }
        // true => connection going from receiver to sender
        } else {
            if self
                .vertices
                .get_mut(sender.0)
                .unwrap()
                .edges
                .as_mut()
                .is_none()
            {
                self.vertices.get_mut(sender.0).unwrap().edges = Some(vec![GraphEdge {
                    value,
                    edges: (sender.0, receiver),
                }])
            } else {
                self.vertices
                    .get_mut(sender.0)
                    .unwrap()
                    .edges
                    .as_mut()
                    .unwrap()
                    .push(GraphEdge {
                        value,
                        edges: (sender.0, receiver),
                    });
            }
            // node from connection is going
            if self
                .vertices
                .get_mut(receiver)
                .unwrap()
                .edges
                .as_mut()
                .is_none()
            {
                self.vertices.get_mut(receiver).unwrap().edges = Some(vec![GraphEdge {
                    value,
                    edges: (sender.0, receiver),
                }])
            } else {
                self.vertices
                    .get_mut(receiver)
                    .unwrap()
                    .edges
                    .as_mut()
                    .unwrap()
                    .push(GraphEdge {
                        value,
                        edges: (sender.0, receiver),
                    });
            }
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

    // not working if starting not from node 0 because of how directed graph implemented
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
}
