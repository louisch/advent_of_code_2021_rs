use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use petgraph::graph::{UnGraph, IndexType, NodeIndex};


pub fn part_1(lines: &Vec<String>) {
    if let Some(count) = traverse_graph_1(lines) {
        println!("Total distinct paths: {}", count);
    } else {
        println!("Failed to find paths!");
    }
}

fn traverse_graph_1(lines: &Vec<String>) -> Option<u64> {
    let lines_as_nodes: Vec<Vec<&str>> = lines.into_iter()
        .filter_map(|line| {
            let line_trimmed = line.trim();
            if line_trimmed.is_empty() {
                return None;
            }
            Some(line_trimmed.split('-').collect())
        }).collect();
    let unique_nodes = HashSet::<&str>::from_iter(lines_as_nodes.iter().map(|s| s.clone()).flatten());
    let nodes = unique_nodes.into_iter().collect::<Vec<&str>>();
    let nodes_by_name: HashMap<&str, NodeIndex> = nodes.iter().enumerate().map(|(i, name)| (*name, NodeIndex::new(i))).collect();
    let start_node_index = nodes_by_name.get("start")?;
    let end_node_index = nodes_by_name.get("end")?;
    let graph = UnGraph::<&str, ()>::from_edges(lines_as_nodes.into_iter().filter_map(|pair| {
        let i1 = nodes_by_name.get(pair[0])?;
        let i2 = nodes_by_name.get(pair[1])?;
        Some((*i1, *i2))
    }).collect::<Vec<(NodeIndex, NodeIndex)>>());

    let mut paths_in_progress = vec![(*start_node_index, vec![*start_node_index])];
    let mut paths_completed = vec![];
    while let Some((current_index, mut path)) = paths_in_progress.pop() {
        path.push(current_index);

        if current_index == *end_node_index {
            paths_completed.push(path);
            continue;
        }

        let neighbors = graph.neighbors(current_index);
        let visitable_neighbors = neighbors.into_iter().filter(|neighbor| {
            let neighbor_node = nodes[neighbor.index()];
            neighbor_node.chars().all(|c| c.is_ascii_uppercase()) || !path.contains(neighbor)
        });
        for neighbor in visitable_neighbors {
            paths_in_progress.push((neighbor, path.clone()));
        }
    }

    Some(paths_completed.len() as u64)
}




pub fn part_2(lines: &Vec<String>) {
    if let Some(count) = traverse_graph_2(lines) {
        println!("Total distinct paths: {}", count);
    } else {
        println!("Failed to find paths!");
    }
}

fn vec_is_unique<T>(vec: &Vec<T>) -> bool
   where T: Eq + std::hash::Hash + Copy {
    let mut found: HashSet<T> = HashSet::new();
    for el in vec {
        if found.contains(el) {
            return false;
        }
        found.insert(*el);
    }
    true
}

fn traverse_graph_2(lines: &Vec<String>) -> Option<u64> {
    let lines_as_nodes: Vec<Vec<&str>> = lines.into_iter()
        .filter_map(|line| {
            let line_trimmed = line.trim();
            if line_trimmed.is_empty() {
                return None;
            }
            Some(line_trimmed.split('-').collect())
        }).collect();
    let unique_nodes = HashSet::<&str>::from_iter(lines_as_nodes.iter().map(|s| s.clone()).flatten());
    let nodes = unique_nodes.into_iter().collect::<Vec<&str>>();
    let nodes_by_name: HashMap<&str, NodeIndex> = nodes.iter().enumerate().map(|(i, name)| (*name, NodeIndex::new(i))).collect();
    let start_node_index = nodes_by_name.get("start")?;
    let end_node_index = nodes_by_name.get("end")?;
    let graph = UnGraph::<&str, ()>::from_edges(lines_as_nodes.into_iter().filter_map(|pair| {
        let i1 = nodes_by_name.get(pair[0])?;
        let i2 = nodes_by_name.get(pair[1])?;
        Some((*i1, *i2))
    }).collect::<Vec<(NodeIndex, NodeIndex)>>());

    let mut paths_in_progress = vec![(*start_node_index, vec![])];
    let mut paths_completed = vec![];
    while let Some((current_index, mut path)) = paths_in_progress.pop() {
        path.push(current_index);

        if current_index == *end_node_index {
            paths_completed.push(path);
            continue;
        }

        let neighbors = graph.neighbors(current_index);
        let visitable_neighbors = neighbors.into_iter().filter(|neighbor| {
            let neighbor_name = nodes[neighbor.index()];
            let is_start_or_end = neighbor == start_node_index || neighbor == end_node_index;
            let path_smalls: Vec<&str> = path.iter().filter(|i| *i != start_node_index && *i != end_node_index && !nodes[i.index()].chars().all(|c| c.is_ascii_uppercase())).map(|i| nodes[i.index()]).collect();
            let has_two_smalls = !vec_is_unique(&path_smalls);
            neighbor_name.chars().all(|c| {
                if c.is_ascii_uppercase() {
                    return true;
                }
                if is_start_or_end || has_two_smalls {
                    return !path.contains(neighbor);
                }
                !has_two_smalls && path.iter().filter(|visited| *visited == neighbor).count() < 2
            })
        });
        for neighbor in visitable_neighbors {
            paths_in_progress.push((neighbor, path.clone()));
        }
    }

    Some(paths_completed.len() as u64)
}


#[cfg(test)]
mod tests {
    use crate::day_12::*;

    const TEST_INPUT_1: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;

    const TEST_INPUT_2: &str = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#;

    const TEST_INPUT_3: &str = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"#;

    fn get_test_input(s: &str) -> Vec<String> {
        s.split_whitespace().map(str::to_string).collect()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(traverse_graph_1(&get_test_input(TEST_INPUT_1)), Some(10));
        assert_eq!(traverse_graph_1(&get_test_input(TEST_INPUT_2)), Some(19));
        assert_eq!(traverse_graph_1(&get_test_input(TEST_INPUT_3)), Some(226));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(traverse_graph_2(&get_test_input(TEST_INPUT_1)), Some(36));
        assert_eq!(traverse_graph_2(&get_test_input(TEST_INPUT_2)), Some(103));
        assert_eq!(traverse_graph_2(&get_test_input(TEST_INPUT_3)), Some(3509));
    }
}
