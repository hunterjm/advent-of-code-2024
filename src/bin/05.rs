use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;
use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let rules = sections.next().unwrap();
    let updates = sections.next().unwrap().lines();

    let mut middle_nodes = Vec::new();
    for update in updates {
        let sequence: Vec<u32> = update.split(',').map(|s| s.parse().unwrap()).collect();

        // Only build graph with rules relevant to this sequence
        let filtered_rules = filter_rules(rules, &sequence);
        let (graph, node_map) = build_graph(&filtered_rules);

        if validate_sequence(&graph, &sequence, &node_map) {
            let middle = sequence.len() / 2;
            middle_nodes.push(sequence[middle]);
        }
    }

    Some(middle_nodes.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let rules = sections.next().unwrap();
    let updates = sections.next().unwrap().lines();

    let mut middle_nodes = Vec::new();
    for update in updates {
        let sequence: Vec<u32> = update.split(',').map(|s| s.parse().unwrap()).collect();

        // Only build graph with rules relevant to this sequence
        let filtered_rules = filter_rules(rules, &sequence);
        let (graph, node_map) = build_graph(&filtered_rules);

        if !validate_sequence(&graph, &sequence, &node_map) {
            let order = match toposort(&graph, None) {
                Ok(order) => order.into_iter().map(|idx| graph[idx]).collect::<Vec<_>>(),
                Err(_) => sequence.clone(), // If cyclic, keep original order
            };

            let order_map: HashMap<u32, usize> = order.iter()
                .enumerate()
                .map(|(i, &val)| (val, i))
                .collect();

            let mut sorted_sequence = sequence.clone();
            sorted_sequence.sort_by(|&a, &b| {
                match (order_map.get(&a), order_map.get(&b)) {
                    (Some(ord_a), Some(ord_b)) => ord_a.cmp(ord_b),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => sequence.iter()
                        .position(|&x| x == a)
                        .cmp(&sequence.iter().position(|&x| x == b)),
                }
            });

            let middle = sorted_sequence.len() / 2;
            middle_nodes.push(sorted_sequence[middle]);
        }
    }

    Some(middle_nodes.iter().sum())
}

fn filter_rules(rules: &str, sequence: &[u32]) -> Vec<(u32, u32)> {
    rules.lines()
        .filter_map(|rule| {
            let parts: Vec<u32> = rule.split('|').map(|s| s.parse().unwrap()).collect();
            let (x, y) = (parts[0], parts[1]);
            if sequence.contains(&x) && sequence.contains(&y) {
                Some((x, y))
            } else {
                None
            }
        })
        .collect()
}

fn build_graph(rules: &[(u32, u32)]) -> (DiGraph<u32, ()>, HashMap<u32, NodeIndex>) {
    let mut graph = DiGraph::<u32, ()>::new();
    let mut node_map: HashMap<u32, NodeIndex> = HashMap::new();

    for &(x, y) in rules {
        let x_node = *node_map.entry(x).or_insert_with(|| graph.add_node(x));
        let y_node = *node_map.entry(y).or_insert_with(|| graph.add_node(y));
        graph.add_edge(x_node, y_node, ());
    }

    (graph, node_map)
}

fn validate_sequence(
    graph: &DiGraph<u32, ()>,
    sequence: &[u32],
    node_map: &HashMap<u32, NodeIndex>,
) -> bool {
    // Create a HashMap lookup for the sequence indices
    let seq_index: HashMap<u32, usize> =
        sequence.iter().enumerate().map(|(i, &x)| (x, i)).collect();

    // Iterate through each node in the sequence
    for i in 0..sequence.len() {
        let x = sequence[i];
        if let Some(&x_node) = node_map.get(&x) {
            // Compare against all following nodes in the sequence
            for j in i + 1..sequence.len() {
                let y = sequence[j];
                if let Some(&y_node) = node_map.get(&y) {
                    // Check if a rule (edge) exists: X -> Y or Y -> X
                    let x_to_y = graph.find_edge(x_node, y_node).is_some();
                    let y_to_x = graph.find_edge(y_node, x_node).is_some();

                    if (x_to_y && seq_index[&x] > seq_index[&y])
                        || (y_to_x && seq_index[&y] > seq_index[&x])
                    {
                        return false;
                    }
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
