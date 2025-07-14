use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::{SearchResult, VectorIndex};

#[derive(Debug, Clone)]
struct Connection {
    id: String,
    distance: f32,
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Connection {}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other
            .distance
            .partial_cmp(&self.distance)
            .unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug)]
struct Node {
    id: String,
    vector: Vec<f32>,
    connections: Vec<String>,
}

pub struct HnswIndex {
    nodes: HashMap<String, Node>,
    entry_point: Option<String>,
    max_connections: usize,
    ef_construction: usize,
}

impl HnswIndex {
    pub fn new(_dimensions: usize) -> Result<Self> {
        Ok(Self {
            nodes: HashMap::new(),
            entry_point: None,
            max_connections: 16,
            ef_construction: 200,
        })
    }

    fn search_layer(
        &self,
        query: &[f32],
        entry_points: Vec<String>,
        num_closest: usize,
    ) -> Vec<Connection> {
        let mut visited = std::collections::HashSet::new();
        let mut candidates = BinaryHeap::new();
        let mut w = BinaryHeap::new();

        // Initialize with entry points
        for ep in entry_points {
            if let Some(node) = self.nodes.get(&ep) {
                let distance = cosine_similarity(query, &node.vector);
                let conn = Connection {
                    id: ep.clone(),
                    distance,
                };
                candidates.push(conn.clone());
                w.push(conn);
                visited.insert(ep);
            }
        }

        while let Some(c) = candidates.pop() {
            if let Some(f) = w.peek() {
                if c.distance > f.distance {
                    break;
                }
            }

            if let Some(node) = self.nodes.get(&c.id) {
                for neighbor_id in &node.connections {
                    if !visited.contains(neighbor_id) {
                        visited.insert(neighbor_id.clone());

                        if let Some(neighbor) = self.nodes.get(neighbor_id) {
                            let distance = cosine_similarity(query, &neighbor.vector);
                            let conn = Connection {
                                id: neighbor_id.clone(),
                                distance,
                            };

                            if w.len() < num_closest {
                                candidates.push(conn.clone());
                                w.push(conn);
                            } else if let Some(f) = w.peek() {
                                if distance < f.distance {
                                    candidates.push(conn.clone());
                                    w.push(conn);
                                    if w.len() > num_closest {
                                        w.pop();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        w.into_sorted_vec()
    }
}

impl VectorIndex for HnswIndex {
    fn add_vector(&mut self, id: &str, vector: &[f32]) -> Result<()> {
        let node = Node {
            id: id.to_string(),
            vector: vector.to_vec(),
            connections: Vec::new(),
        };

        // If this is the first node, make it the entry point
        if self.entry_point.is_none() {
            self.entry_point = Some(id.to_string());
            self.nodes.insert(id.to_string(), node);
            return Ok(());
        }

        // Search for closest nodes
        let entry_point = self.entry_point.as_ref().unwrap().clone();
        let candidates = self.search_layer(vector, vec![entry_point], self.ef_construction);

        // Select M neighbors
        let mut selected = Vec::new();
        for candidate in candidates.into_iter().take(self.max_connections) {
            selected.push(candidate.id);
        }

        // Add bidirectional connections
        let mut new_node = node;
        new_node.connections = selected.clone();

        for neighbor_id in &selected {
            if let Some(neighbor) = self.nodes.get_mut(neighbor_id) {
                neighbor.connections.push(id.to_string());
                // Prune connections if needed
                if neighbor.connections.len() > self.max_connections {
                    neighbor.connections.truncate(self.max_connections);
                }
            }
        }

        self.nodes.insert(id.to_string(), new_node);
        Ok(())
    }

    fn remove_vector(&mut self, id: &str) -> Result<bool> {
        if let Some(node) = self.nodes.remove(id) {
            // Remove connections from neighbors
            for neighbor_id in &node.connections {
                if let Some(neighbor) = self.nodes.get_mut(neighbor_id) {
                    neighbor.connections.retain(|conn_id| conn_id != id);
                }
            }

            // Update entry point if needed
            if self.entry_point.as_ref() == Some(&id.to_string()) {
                self.entry_point = self.nodes.keys().next().cloned();
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>> {
        if let Some(entry_point) = &self.entry_point {
            let connections = self.search_layer(query, vec![entry_point.clone()], k.max(50));

            let results = connections
                .into_iter()
                .take(k)
                .map(|conn| SearchResult {
                    id: conn.id,
                    score: conn.distance, // Using cosine similarity
                })
                .collect();

            Ok(results)
        } else {
            Ok(Vec::new())
        }
    }

    fn size(&self) -> usize {
        self.nodes.len()
    }

    fn clear(&mut self) {
        self.nodes.clear();
        self.entry_point = None;
    }
}

// Helper functions for distance calculations
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}

fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}

fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
