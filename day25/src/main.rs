use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use rand::seq::IteratorRandom;

type NodeMap = HashMap<String, Rc<HashSet<String>>>;

fn parse(input: &[&str]) -> (NodeMap, HashSet<(String, String)>) {
    let mut nodes = HashMap::new();
    let mut edges = HashSet::new();

    for line in input {
        let (node_str, conns_str) = line.split_once(": ").unwrap();
        nodes.insert(
            node_str.to_owned(),
            Rc::new(HashSet::from_iter([node_str.to_owned()])),
        );
        for conn in conns_str.split_ascii_whitespace() {
            if !nodes.contains_key(conn) {
                nodes.insert(
                    conn.to_owned(),
                    Rc::new(HashSet::from_iter([conn.to_owned()])),
                );
            }
            edges.insert((node_str.to_owned(), conn.to_owned()));
        }
    }

    (nodes, edges)
}

fn contract_edge(
    edge: (String, String),
    nodes: &mut HashMap<String, Rc<HashSet<String>>>,
    edges: &mut HashSet<(String, String)>,
) {
    let (left, right) = (nodes.get(&edge.0).unwrap(), nodes.get(&edge.1).unwrap());
    assert!(!Rc::ptr_eq(left, right));

    let merged: Rc<HashSet<String>> = Rc::new(left.union(right).cloned().collect());
    *nodes.get_mut(&edge.0).unwrap() = merged.clone();
    *nodes.get_mut(&edge.1).unwrap() = merged.clone();

    for v in nodes.values_mut() {
        if merged.is_superset(v) {
            *v = merged.clone();
        }
    }

    // this edge removed because of ownership
    // nodes fixed up to both point at merge of nodes
    // need to remove any edges that have both sides now same node
    edges.retain(|e| !Rc::ptr_eq(nodes.get(&e.0).unwrap(), nodes.get(&e.1).unwrap()));
}

fn solve1(input: &[&str]) -> usize {
    let (nodes, edges) = parse(input);
    let mut rng = rand::thread_rng();

    loop {
        let mut nodes = nodes.clone();
        let mut edges = edges.clone();
        let mut num_nodes = nodes.len();
        while num_nodes > 2 {
            let remove_edge = edges.iter().choose(&mut rng).unwrap().clone();
            let remove_edge = edges.take(&remove_edge).unwrap();
            contract_edge(remove_edge, &mut nodes, &mut edges);
            num_nodes -= 1;
        }

        if edges.len() == 3 {
            let first_supernode = nodes.values().next().unwrap();
            for supernode in nodes.values() {
                if !Rc::ptr_eq(first_supernode, supernode) {
                    return first_supernode.len() * supernode.len();
                }
            }
        }
    }
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "jqt: rhn xhk nvd",
        "rsh: frs pzl lsr",
        "xhk: hfx",
        "cmg: qnr nvd lhk bvb",
        "rhn: xhk bvb hfx",
        "bvb: xhk hfx",
        "pzl: lsr hfx nvd",
        "qnr: nvd",
        "ntq: jqt hfx bvb xhk",
        "nvd: lhk",
        "lsr: lhk",
        "rzs: qnr cmg lsr rsh",
        "frs: qnr lhk lsr",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 54)
    }
}
