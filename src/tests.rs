use crate::graphs::*;
fn create_graph() -> Graph {
    let mut graph = Graph::new();
    graph.add();
    graph.add();
    graph
}

fn create_large_graph() -> Graph {
    let mut graph = Graph::new();
    let mut i = 0;
    while i < 7 {
        graph.add();
        i += 1;
    }

    graph.connect(100, 0, 1, false);
    graph.connect(100, 0, 2, false);
    graph.connect(100, 1, 3, false);
    graph.connect(100, 2, 4, false);
    graph.connect(1000, 2, 5, false);
    graph.connect(2000, 4, 6, false);
    graph.connect(100, 5, 6, false);
    graph.connect(100, 5, 7, false);
    graph.connect(100, 6, 7, false);
    graph
    // 1 ← 0   4 → 6        1 ← 0 = (100)          4 = (2000) → 6
    // ↓   ↓ ↗   ↗ ↓        ↓          ↓         ↗  = (100)  (↗ ↓) = (100)
    // 3   2 → 5 → 7        3 = (100)  2 = (100) → 5 = (1000) → 7
}

fn create_large_graph_2() -> Graph {
    let mut graph = Graph::new();
    let mut i = 0;
    while i < 7 {
        graph.add();
        i += 1;
    }
    graph.connect(100, 0, 1, false);
    graph.connect(100, 0, 2, false);
    graph.connect(100, 1, 3, false);
    graph.connect(1, 2, 4, false);
    graph.connect(100, 2, 5, false);
    graph.connect(1, 4, 6, false);
    graph.connect(100, 5, 6, false);
    graph.connect(10, 5, 7, false);
    graph.connect(1, 6, 7, false);
    graph
    // 1 ← 0   4 → 6        1 ← 0 = (100)          4 = (1) → 6          6
    // ↓   ↓ ↗   ↗ ↓        ↓          ↓         ↗  = (1)  ↗ = (100)    ↓ = (1)
    // 3   2 → 5 → 7        3 = (100)  2 = (100) → 5 = (100) → 7 = (10) 7
}

#[test]
fn edge_connection_works() {
    let mut graph = create_graph();
    graph.connect(1, 0, 1, false);
    graph.connect(10, 0, 1, true);
    graph.connect(15, 1, 2, false);
    graph.connect(20, 0, 2, false);

    if let Some(edge) = graph.vertices[0].edges.as_ref() {
        assert_eq!(
            edge.get(0),
            Some(&GraphEdge {
                value: 1,
                edges: (0, 1)
            })
        );
    }
    if let Some(edge) = graph.vertices[2].edges.as_ref() {
        assert_eq!(
            edge.get(0),
            Some(&GraphEdge {
                value: 15,
                edges: (1, 2)
            })
        );
    }
}

#[test]
fn len_and_is_empty_works() {
    let graph = create_graph();
    assert_eq!(graph.is_empty(), false);
    assert_eq!(graph.len(), 3);
}

#[test]
fn breadth_first_search_works() {
    // 1 ← 0   4 → 6
    // ↓   ↓ ↗   ↗ ↓
    // 3   2 → 5 → 7
    let graph = create_large_graph();
    let distance = graph.breadth_first_search(0);
    let distance_2 = graph.breadth_first_search(2);
    // from node with index 0
    assert_eq!(vec![0, 1, 1, 2, 2, 2, 3, 3], distance);
    // from node with index 2
    assert_eq!(vec![0, 0, 0, 0, 1, 1, 2, 2], distance_2);
}

#[test]
fn dijkstra_works() {
    let graph = create_large_graph();
    let graph_2 = create_large_graph_2();
    assert_eq!(graph.dijkstra(0), [0, 100, 100, 200, 200, 1100, 1200, 1200]);
    assert_eq!(graph_2.dijkstra(0), [0, 100, 100, 200, 101, 200, 102, 103]);
    assert_eq!(graph_2.dijkstra(2), [0, 0, 0, 0, 1, 100, 2, 3]);
    assert_eq!(graph.dijkstra(1), [0, 0, 0, 100, 0, 0, 0, 0]);
}
