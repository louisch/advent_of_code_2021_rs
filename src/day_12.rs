use petgraph::graph::{Graph, UnGraph};
use petgraph::Undirected;


pub fn part_1(lines: &Vec<String>) {
}

struct Node<'a> {
    name: String,
    connected: UnsafeCell<HashMap<String, &'a Node<'a>>>,
}

impl<'a> Node<'a> {
    fn is_start(&self) -> bool {
        self.name == "start"
    }

    fn is_end(&self) -> bool {
        self.name == "end"
    }

    fn is_big(&self) -> bool {
        self.name.matches(|c: char| c.is_ascii_uppercase()).count() > 0
    }

    fn is_small(&self) -> bool {
        !self.is_big()
    }

    fn from_name(name: String, arena: &'a Arena<Node<'a>>) -> &'a Node<'a> {
        Node::new(name, UnsafeCell::new(HashMap::new()), arena)
    }

    fn new(name: String, connected: UnsafeCell<HashMap<String, &'a Node<'a>>>, arena: &'a Arena<Node<'a>>) -> &'a Node<'a> {
        arena.alloc(Node {
            connected: connected,
            name: name,
        })
    }

    unsafe fn connect(&self, other_node: &'a Node<'a>) {
        (*self.connected.get()).insert(other_node.name.clone(), other_node);
    }

    unsafe fn is_connected_to(&self, other_node: &Node) -> bool {
        (*self.connected.get()).contains_key(&other_node.name)
    }
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Node<'a>) -> bool {
        self.name == other.name
    }
}

unsafe fn node_from_line<'a>(line: &String, existing_nodes: &mut HashMap<String, &'a Node<'a>>, arena: &'a Arena<Node<'a>>) -> Option<(Node<'a>, Node<'a>)> {
    let separator_index = line.trim().find('-')?;
    let (first_name, second_name) = line.split_at(separator_index);
    let first_node_option = existing_nodes.get(first_name);
    let second_node_option = existing_nodes.get(second_name);
    if first_node_option.is_some() && second_node_option.is_some() {
        return None; // Both nodes already exist, so just return without creating anything (this doesn't happen for our puzzle inputs)
    }
    let mut first_node = first_node_option.unwrap_or(&&Node::from_name(first_name.to_owned(), arena));
    let mut second_node = second_node_option.unwrap_or(&&Node::from_name(second_name.to_owned(), arena));
    if !first_node.is_connected_to(second_node) {
        first_node.connect(&Box::new(*second_node));
    }
    if !second_node.is_connected_to(second_node) {
        second_node.connect(&Box::new(*first_node));
    }
    existing_nodes.insert(first_name.to_owned(), &first_node);
    existing_nodes.insert(second_name.to_owned(), &second_node);
    Some((**first_node, **second_node))
}

unsafe fn traverse_graph(lines: &Vec<String>) -> Option<u64> {
    let arena = Arena::new();
    let mut nodes: HashMap<String, &Node> = HashMap::new();
    for line in lines {
        node_from_line(&line, &mut nodes, &arena);
    }
    let start_node = nodes.get("start")?;
    let mut visitable_nodes: Vec<&Node> = (*start_node.connected.get()).values().map(|n| *n).collect();
    let mut paths = vec![];
    let mut current_path: Vec<String> = vec![start_node.name.clone()];

    while let Some(current_node) = visitable_nodes.pop() {
        if current_node.is_start() {
            continue;
        } if current_node.is_end() {
            current_path.push("end".to_owned());
            paths.push(current_path.clone());
            if current_path.last().as_deref() != visitable_nodes.last().map(|n| n.name.to_string()).as_deref() {
            }
        } else if current_node.is_big() || !current_path.contains(&current_node.name) {
            current_path.push(current_node.name.clone());
            visitable_nodes.append(&mut (*current_node.connected.get()).values().map(|n| *n).collect::<Vec<&Node>>());
        }
    }

    Some(paths.len() as u64)
}




pub fn part_2(lines: &Vec<String>) {
}


#[cfg(test)]
mod tests {
    use crate::day_12::*;

    const TEST_INPUT: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;

    fn get_test_input() -> Vec<String> {
        TEST_INPUT.split_whitespace().map(str::to_string).collect()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(traverse_graph(&get_test_input()), Some(10));
    }
}
