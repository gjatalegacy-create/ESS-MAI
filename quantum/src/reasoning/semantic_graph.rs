// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  REASONING/SEMANTIC_GRAPH.RS — Grafi Semantik me Peshë (Teoria 30)    ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  nodes / edges / 4 relacione / cross-module / confidence links.       ║
// ║  Relacionet: supports | contradicts | extends | requires             ║
// ║  semantic_distance via BFS (normalizuar ndaj max_depth=10).          ║
// ║                                                                          ║
// ║  Pseudo përdor IF/WHILE; KONVERTUAR në match/loop pa if (zero if/else).║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::collections::{HashMap, HashSet, VecDeque};

// ─────────────────────────────────────────────────────────────────────────────
// RELACIONI — 4 tipet e skajeve
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Relation {
    Supports,    // dykahor
    Contradicts,
    Extends,
    Requires,
}

impl Relation {
    pub fn label(self) -> &'static str {
        match self {
            Relation::Supports    => "supports",
            Relation::Contradicts => "contradicts",
            Relation::Extends     => "extends",
            Relation::Requires    => "requires",
        }
    }

    /// supports është dykahor (bidirectional).
    pub fn is_bidirectional(self) -> bool {
        matches!(self, Relation::Supports)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// NODE + EDGE
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SemanticNode {
    pub node_id: usize,
    pub label:   String,
    pub domain:  String,
    pub module:  String,
    pub weight:  f32,
    pub stems:   HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct SemanticEdge {
    pub from:       usize,
    pub to:         usize,
    pub relation:   Relation,
    pub confidence: f32,
    pub cross_module: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// SEMANTIC GRAPH
// ─────────────────────────────────────────────────────────────────────────────

pub struct SemanticGraph {
    nodes:     Vec<SemanticNode>,
    edges:     Vec<SemanticEdge>,
    adjacency: HashMap<usize, Vec<usize>>,
    next_id:   usize,
}

impl SemanticGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            adjacency: HashMap::new(),
            next_id: 0,
        }
    }

    /// add_node — kthen node_id.
    pub fn add_node(&mut self, label: &str, domain: &str, module: &str, weight: f32) -> usize {
        use crate::tokenizer::semantic_stems;
        let node_id = self.next_id;
        self.next_id += 1;
        self.nodes.push(SemanticNode {
            node_id, label: label.to_string(), domain: domain.to_string(),
            module: module.to_string(), weight,
            stems: semantic_stems(label),
        });
        node_id
    }

    /// add_edge — shton skaj + adjacency. supports → dykahor.
    pub fn add_edge(&mut self, from: usize, to: usize, relation: Relation, confidence: f32) {
        let cross_module = self.is_cross_module(from, to);
        self.edges.push(SemanticEdge { from, to, relation, confidence, cross_module });

        // Adjacency: gjithmonë from→to.
        self.adjacency.entry(from).or_insert_with(Vec::new).push(to);

        // supports → shto edhe to→from (dykahor). Zero if — match.
        match relation.is_bidirectional() {
            true  => { self.adjacency.entry(to).or_insert_with(Vec::new).push(from); }
            false => {}
        }
    }

    fn is_cross_module(&self, from: usize, to: usize) -> bool {
        let m_from = self.nodes.get(from).map(|n| n.module.clone());
        let m_to = self.nodes.get(to).map(|n| n.module.clone());
        match (m_from, m_to) {
            (Some(a), Some(b)) => a != b,
            _ => false,
        }
    }

    // ── BFS DISTANCE ───────────────────────────────────────────────────────

    /// bfs_distance — distanca minimale midis dy node-ve.
    /// Kthen depth ose 11 (= INF/jashtë kufirit max_depth=10).
    pub fn bfs_distance(&self, from_id: usize, to_id: usize) -> u32 {
        // from == to → 0 (match, zero if).
        match from_id == to_id {
            true => 0,
            false => self.bfs_search(from_id, to_id),
        }
    }

    fn bfs_search(&self, from_id: usize, to_id: usize) -> u32 {
        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(from_id);
        let mut queue: VecDeque<(usize, u32)> = VecDeque::new();
        queue.push_back((from_id, 0));

        // Loop pa if — përdor while let + match për kontrollet.
        while let Some((current, depth)) = queue.pop_front() {
            // Kufiri max_depth=10 → kthe 11 (INF).
            match depth > 10 {
                true => return 11,
                false => {}
            }

            let neighbors = self.adjacency.get(&current).cloned().unwrap_or_default();
            for neighbor in neighbors {
                // neighbor == to → gjetëm (depth+1).
                match neighbor == to_id {
                    true => return depth + 1,
                    false => {
                        // I pavizituar → shto.
                        match visited.insert(neighbor) {
                            true  => queue.push_back((neighbor, depth + 1)),
                            false => {}
                        }
                    }
                }
            }
        }
        11 // INF — pa rrugë
    }

    /// semantic_distance — BFS minimal midis node-ve të query e proposal.
    /// Normalizuar ndaj max_depth=10. 0=perfekte, 1=pa lidhje.
    pub fn semantic_distance(&self, from_id: usize, to_id: usize) -> f32 {
        let dist = self.bfs_distance(from_id, to_id);
        (dist as f32 / 10.0).clamp(0.0, 1.0)
    }

    // ── BUILD FROM CANDIDATES ──────────────────────────────────────────────

    /// build_from_candidates — ndërton grafin nga kandidatët + query.
    /// candidates: (label, domain, module, score).
    /// relation = supports nëse score>0.5, ndryshe extends (zero if — match).
    pub fn build_from_candidates(
        &mut self,
        query: &str,
        domain: &str,
        candidates: &[(String, String, String, f32)],
    ) -> usize {
        let q_node = self.add_node(query, domain, "ENTRY", 1.0);

        for (label, dom, module, score) in candidates {
            let c_node = self.add_node(label, dom, module, *score);
            // relation nga score (zero if — match mbi krahasim).
            let relation = match *score > 0.5 {
                true  => Relation::Supports,
                false => Relation::Extends,
            };
            self.add_edge(q_node, c_node, relation, *score);
        }
        q_node
    }

    // ── TOPOLOGY ───────────────────────────────────────────────────────────

    pub fn cross_module_links(&self) -> Vec<&SemanticEdge> {
        self.edges.iter().filter(|e| e.cross_module).collect()
    }

    pub fn confidence_links(&self, min_confidence: f32) -> Vec<&SemanticEdge> {
        self.edges.iter().filter(|e| e.confidence >= min_confidence).collect()
    }

    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }
}

impl Default for SemanticGraph {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_nodes_and_edges() {
        let mut g = SemanticGraph::new();
        let a = g.add_node("quantum reasoning", "sci", "PRO", 0.9);
        let b = g.add_node("deterministic elimination", "sci", "SRK", 0.8);
        g.add_edge(a, b, Relation::Supports, 0.85);
        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 1);
    }

    #[test]
    fn bfs_self_is_zero() {
        let mut g = SemanticGraph::new();
        let a = g.add_node("x", "d", "m", 1.0);
        assert_eq!(g.bfs_distance(a, a), 0);
    }

    #[test]
    fn bfs_finds_path() {
        let mut g = SemanticGraph::new();
        let a = g.add_node("a", "d", "m", 1.0);
        let b = g.add_node("b", "d", "m", 1.0);
        let c = g.add_node("c", "d", "m", 1.0);
        // a→b→c (extends, jednokahor)
        g.add_edge(a, b, Relation::Extends, 0.8);
        g.add_edge(b, c, Relation::Extends, 0.8);
        assert_eq!(g.bfs_distance(a, c), 2);
        assert_eq!(g.bfs_distance(a, b), 1);
    }

    #[test]
    fn bfs_no_path_is_inf() {
        let mut g = SemanticGraph::new();
        let a = g.add_node("a", "d", "m", 1.0);
        let b = g.add_node("b", "d", "m", 1.0);
        // Pa skaj → INF (11)
        assert_eq!(g.bfs_distance(a, b), 11);
    }

    #[test]
    fn supports_is_bidirectional() {
        let mut g = SemanticGraph::new();
        let a = g.add_node("a", "d", "m", 1.0);
        let b = g.add_node("b", "d", "m", 1.0);
        g.add_edge(a, b, Relation::Supports, 0.9);
        // dykahor → b→a gjithashtu
        assert_eq!(g.bfs_distance(b, a), 1);
    }

    #[test]
    fn build_from_candidates_creates_graph() {
        let mut g = SemanticGraph::new();
        let cands = vec![
            ("strong candidate".to_string(), "sci".to_string(), "PRO".to_string(), 0.8),
            ("weak candidate".to_string(), "sci".to_string(), "SRK".to_string(), 0.3),
        ];
        let q = g.build_from_candidates("the query", "sci", &cands);
        // 1 query + 2 kandidatë = 3 node, 2 skaje
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);
        // query→strong është supports (0.8>0.5); query→weak është extends.
        assert_eq!(g.bfs_distance(q, 1), 1); // te strong (supports, dykahor)
    }

    #[test]
    fn cross_module_detected() {
        let mut g = SemanticGraph::new();
        let a = g.add_node("a", "d", "PRO", 1.0);
        let b = g.add_node("b", "d", "SRK", 1.0);  // module ndryshe
        g.add_edge(a, b, Relation::Extends, 0.8);
        assert_eq!(g.cross_module_links().len(), 1);
    }

    #[test]
    fn confidence_filter() {
        let mut g = SemanticGraph::new();
        let a = g.add_node("a", "d", "m", 1.0);
        let b = g.add_node("b", "d", "m", 1.0);
        let c = g.add_node("c", "d", "m", 1.0);
        g.add_edge(a, b, Relation::Extends, 0.9);
        g.add_edge(a, c, Relation::Extends, 0.5);
        assert_eq!(g.confidence_links(0.8).len(), 1); // vetëm 0.9
    }
}
