use crate::agents::Agent;
use petgraph::graph::DiGraph;
use petgraph::{Direction, Graph};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct TransmissionLine {
    // Inherent properties
    efficiency: f32, // 0-1
    capacity: f32,   // kW

    // Dynamic properties
    supply: f32, // kW
    demand: f32, // kW
    rate: f32,   // $/kW
}

impl TransmissionLine {
    pub fn new() -> Self {
        Self {
            capacity: 100.0,
            efficiency: 1.0,
            supply: 0.0,
            demand: 0.0,
            rate: 0.0,
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
    graph: DiGraph<Box<dyn Agent + Send + Sync>, TransmissionLine>,
    time: u32,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            time: 0,
        }
    }

    /// Add an agent to the grid
    pub fn add_agent(&mut self, agent: Box<dyn Agent + Send + Sync>) {
        self.graph.add_node(agent);
    }

    /// Delete an agent from the grid
    pub fn delete_agent(&mut self, id: u128) {
        let index = self
            .graph
            .node_indices()
            .find(|i| self.graph[*i].id().as_u128() == id);

        if let Some(index) = index {
            self.graph.remove_node(index);
        }
    }

    /// Set the name of an agent
    pub fn set_agent_name(&mut self, id: u128, name: String) -> bool {
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

    pub fn add_connection(&mut self, from: u128, to: u128, line: TransmissionLine) -> bool {
        // Don't allow self-loops
        if from == to {
            return false;
        }

        // Find the nodes with matching ids
        let from_index = self
            .graph
            .node_indices()
            .find(|i| self.graph[*i].id().as_u128() == from);
        let to_index = self
            .graph
            .node_indices()
            .find(|i| self.graph[*i].id().as_u128() == to);

        // If the nodes exist, add the edge
        if let Some(from_index) = from_index {
            if let Some(to_index) = to_index {
                self.graph.add_edge(from_index, to_index, line);
                return true;
            }
        }

        return false;
    }

    pub fn get_connections(&self, id: u128) -> Vec<u128> {
        let index = self
            .graph
            .node_indices()
            .find(|i| self.graph[*i].id().as_u128() == id);

        if let Some(index) = index {
            return self
                .graph
                .neighbors_undirected(index)
                .map(|i| self.graph[i].id().as_u128())
                .collect();
        }

        return vec![];
    }

    /// Initialize the edges of the grid
    pub fn setup(&mut self) {

        // For every producer, set the rate of outgoing edges to the rate of the producer
    }

    /// Step the grid forward one time step
    pub fn step(&mut self) {
        // Create a copy of the previous time step
        /*
        let previous_graph = self.graph.clone();

        println!("Step: {}", self.time);

        // Node indices are the same in both graphs
        for index in self.graph.node_indices() {
            // let mut new_agent = &self.graph[index];
            // let previous_agent = &previous_graph[index];

            // Get the edges connected to this node
            let p_incoming = previous_graph
                .edges_directed(index, Direction::Incoming)
                .collect();
            let mut n_incoming = self
                .graph
                .edges_directed(index, Direction::Incoming)
                .collect();

            let p_outgoing = previous_graph
                .edges_directed(index, Direction::Outgoing)
                .collect();
            let mut n_outgoing: Vec<petgraph::graph::EdgeReference<'_, TransmissionLine>> = self
                .graph
                .edges_directed(index, Direction::Outgoing)
                .collect();

            // The agent will needs to know the previous edges to update the new edges
            //let edges = zip_eq(self.graph.edges(index), previous_graph.edges(index));
            let mut agent = self.graph[index];
            // agent.step(p_incoming, n_incoming, p_outgoing, n_outgoing);

            println!("Agent: {}", agent.name());
        }
        */

        self.time += 1;
    }

    pub fn get_agents(&self) -> Vec<&Box<dyn Agent + Send + Sync>> {
        self.graph.node_weights().collect()
    }
}
