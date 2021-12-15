use std::collections::HashMap;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::dijkstra::dijkstra;


pub fn part_1(lines: &Vec<String>) {
    if let Some(total_risk) = lowest_total_risk_part_1(lines) {
        println!("Total Risk: {}", total_risk);
    } else {
        println!("Could not find total risk!");
    }
}

pub fn part_2(lines: &Vec<String>) {
    if let Some(total_risk) = lowest_total_risk_part_2(lines) {
        println!("Total Risk: {}", total_risk);
    } else {
        println!("Could not find total risk!");
    }
}


fn lowest_total_risk_part_1(lines: &Vec<String>) -> Option<u64> {
    let (graph, start_node, end_node) = parse_graph(lines, false);
    lowest_total_risk(&graph, start_node, end_node)
}
fn lowest_total_risk_part_2(lines: &Vec<String>) -> Option<u64> {
    let (graph, start_node, end_node) = parse_graph(lines, true);
    lowest_total_risk(&graph, start_node, end_node)
}

fn parse_graph(lines: &Vec<String>, is_part_2: bool) -> (Graph<usize, u64>, NodeIndex, NodeIndex) {
    let mut graph = Graph::<usize, u64>::new();
    let mut node_weights = Vec::<u64>::new();
    let mut node_map = HashMap::<(usize, usize), usize>::new();
    let mut tile_height = 0;
    let mut tile_width = 0;

    for (i, line) in lines.iter().filter(|line| !line.trim().is_empty()).enumerate() {
        if tile_height < i {
            tile_height = i;
        }
        for (j, weight_as_char) in line.chars().enumerate() {
            if tile_width < j {
                tile_width = j;
            }

            let weight = weight_as_char.to_digit(10).unwrap() as u64;
            create_node(&mut graph, i, j, weight, &mut node_map, &mut node_weights);
        }
    }

    tile_height += 1;
    tile_width += 1;

    if is_part_2 && lines.len() > 0 && lines[0].len() > 0 {
        for tile_i in 0..5 {
            for tile_j in 0..5 {
                if tile_i == 0 && tile_j == 0 {
                    continue;
                }

                for i in 0..tile_height {
                    for j in 0..tile_width {
                        let map_i = i + tile_i * tile_height;
                        let map_j = j + tile_j * tile_width;
                        if let Some(original_node_index) = node_map.get(&(i, j)) {
                            let original_weight = node_weights[*original_node_index];
                            let weight = ((original_weight - 1 + tile_i as u64 + tile_j as u64) % 9) + 1;
                            create_node(&mut graph, map_i, map_j, weight, &mut node_map, &mut node_weights);
                        }
                    }
                }
            }
        }
    }

    (graph, NodeIndex::new(0), NodeIndex::new(node_weights.len() - 1))
}

fn create_node(graph: &mut Graph<usize, u64>, i: usize, j: usize, weight: u64, node_map: &mut HashMap<(usize, usize), usize>, node_weights: &mut Vec<u64>) {
    node_weights.push(weight);
    let node_index = node_weights.len() - 1;
    node_map.insert((i, j), node_index);
    graph.add_node(node_index);
    if i > 0 {
        if let Some(up_node_index) = node_map.get(&(i - 1, j)) {
            let up_weight = node_weights[*up_node_index];
            graph.add_edge(NodeIndex::new(*up_node_index), NodeIndex::new(node_index), weight);
            graph.add_edge(NodeIndex::new(node_index), NodeIndex::new(*up_node_index), up_weight);
        }
    }
    if j > 0 {
        if let Some(left_node_index) = node_map.get(&(i, j - 1)) {
            let left_weight = node_weights[*left_node_index];
            graph.add_edge(NodeIndex::new(*left_node_index), NodeIndex::new(node_index), weight);
            graph.add_edge(NodeIndex::new(node_index), NodeIndex::new(*left_node_index), left_weight);
        }
    }
}

fn lowest_total_risk(graph: &Graph<usize, u64>, start_node: NodeIndex, end_node: NodeIndex) -> Option<u64> {
    let res = dijkstra(graph, start_node, Some(end_node), |e| *e.weight());

    let risk = res.get(&end_node)?;
    Some(*risk as u64)
}


#[cfg(test)]
mod tests {
    use crate::day_15::*;

    const TEST_INPUT_1: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    fn get_test_input(s: &str) -> Vec<String> {
        s.split("\n").map(str::to_string).collect()
    }

    #[test]
    fn test_part_1() {
        let test_input = get_test_input(TEST_INPUT_1);
        assert_eq!(lowest_total_risk_part_1(&test_input), Some(40));
    }

    #[test]
    fn test_part_2() {
        let test_input = get_test_input(TEST_INPUT_1);
        assert_eq!(lowest_total_risk_part_2(&test_input), Some(315));
    }
}
