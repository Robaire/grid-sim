use crate::agents::Agent;
use petgraph::Graph;
use std::cmp::Ordering;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct TransmissionLine {
    efficiency: f32,
    capacity: f32,
}

impl TransmissionLine {
    pub fn new() -> Self {
        Self {
            capacity: 100.0,
            efficiency: 1.0,
        }
    }
}

impl PartialOrd for TransmissionLine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // If efficiency is the same, compare capacity
        // Return in descending order, preferring higher efficiency and higher capacity
        if self.efficiency == other.efficiency {
            return Some(other.capacity.partial_cmp(&self.capacity).unwrap());
        } else {
            return Some(other.efficiency.partial_cmp(&self.efficiency).unwrap());
        }
    }
}
pub struct Grid {
    graph: Graph<Agent, TransmissionLine>,
    time: u32,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            time: 0,
        }
    }

    pub fn add_agent(&mut self, agent: Agent) {
        self.graph.add_node(agent);
    }

    pub fn delete_agent(&mut self, id: u128) {
        let index = self
            .graph
            .node_indices()
            .find(|i| self.graph[*i].id().as_u128() == id);

        if let Some(index) = index {
            self.graph.remove_node(index);
        }
    }

    pub fn set_agent(&mut self, id: u128, name: String) -> bool {
        let index = self
            .graph
            .node_indices()
            .find(|i| self.graph[*i].id().as_u128() == id);

        if let Some(index) = index {
            self.graph[index].set_name(name);
            return true;
        }

        return false;
    }

    pub fn add_connection(&mut self, from: Uuid, to: Uuid, line: TransmissionLine) -> bool {
        // Find the nodes with matching ids
        let from_index = self
            .graph
            .node_indices()
            .find(|i| self.graph[*i].id() == from);
        let to_index = self
            .graph
            .node_indices()
            .find(|i| self.graph[*i].id() == to);

        // If the nodes exist, add the edge
        if let Some(from_index) = from_index {
            if let Some(to_index) = to_index {
                self.graph.add_edge(from_index, to_index, line);
                return true;
            }
        }

        return false;
    }

    pub fn step(&mut self) {
        self.time += 1;
    }

    pub fn get_agents(&self) -> Vec<&Agent> {
        self.graph.node_weights().collect()
    }
}
