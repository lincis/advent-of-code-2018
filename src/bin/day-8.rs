use aoc::utils::*;

#[derive(Debug)]
struct TreeNode {
    n_children: usize,
    n_metadata: usize,
    metadata: Vec<usize>,
    idx_children: Vec<usize>,
}

#[derive(Debug)]
struct InProcess {
    idx: usize,
    n_to_process: usize,
}

fn end_of_node (
    nodes_in_process: &mut Vec<InProcess>,
    all_nodes: &mut Vec<TreeNode>,
    metadata_idx: &mut usize,
    n_metadata: &mut usize,
    current_idx: &mut usize
) -> &'static str {
    match nodes_in_process.last() {
        Some(current_node) => {
            if current_node.n_to_process == 0 {
                let return_to_node = (*nodes_in_process).pop().unwrap();
                if all_nodes[return_to_node.idx].n_metadata > 0 {
                    *metadata_idx = return_to_node.idx;
                    *n_metadata = all_nodes[return_to_node.idx].n_metadata;
                    let current_state = "metadata";
                    // println!("Need {} metadata for node w all children at {}, move to {}", n_metadata, metadata_idx, current_state);
                    return current_state;
                } else {
                    *current_idx += 1;
                    let current_state = "header1";
                    // println!("Node in process finished, move to {}", current_state);
                    return current_state;
                }
            } else {
                *current_idx += 1;
                let current_state = "header1";
                // println!("Node in process needs more children, move to {}", current_state);
                return current_state;
            }
        }
        None => {
            *current_idx += 1;
            let current_state = "header1";
            // println!("No nodes in processing at {}, move to {}", current_idx, current_state);
            return current_state;
        },
    }
}

fn parse_nodes (input_data: &Vec<usize>) -> Vec<TreeNode> {
    let mut all_nodes: Vec<TreeNode> = Vec::new();
    let mut nodes_in_process: Vec<InProcess> = Vec::new();
    let mut current_state = "header1";
    let mut current_idx: usize = 0;
    let mut n_metadata: usize = 0;
    let mut metadata_idx: usize = 0;
    for number in input_data {
        // println!("Process number = {}, index = {}, current_state = {}", number, current_idx, current_state);
        match current_state {
            "header1" => {
                match nodes_in_process.last_mut() {
                    Some(current_node) => {
                        // println!("Found node in processing = {:?}", current_node);
                        (*current_node).n_to_process -= 1;
                        all_nodes[current_node.idx].idx_children.push(current_idx);
                    }
                    None => (),
                }
                if number > &0 {
                    nodes_in_process.push(InProcess {
                        idx: current_idx,
                        n_to_process: *number,
                    });
                }
                all_nodes.push(TreeNode {
                    n_children: *number,
                    n_metadata: 0,
                    metadata: Vec::new(),
                    idx_children: Vec::new(),
                });
                current_state = "header2";
                // println!("Add new node at {}, move to {}", current_idx, current_state);
            },
            "header2" => {
                let current_node = all_nodes.last_mut().unwrap();
                (*current_node).n_metadata = *number;
                if current_node.n_children > 0 || number == &0 {
                    current_state = end_of_node(&mut nodes_in_process, &mut all_nodes, &mut metadata_idx, &mut n_metadata, &mut current_idx);
                    // println!("Children present or no metadata at {}, move to {}", current_idx, current_state);
                } else if number > &0 {
                    current_state = "metadata";
                    n_metadata = *number;
                    metadata_idx = current_idx;
                    // println!("No children but metadata {} at {}, move to {}", number, current_idx, current_state);
                } else {
                    current_state = end_of_node(&mut nodes_in_process, &mut all_nodes, &mut metadata_idx, &mut n_metadata, &mut current_idx);
                    // println!("No children or metadata {}, move to {}", current_idx, current_state);
                }
            },
            "metadata" => {
                all_nodes[metadata_idx].metadata.push(*number);
                // println!("Add metadata {} to {}", number, metadata_idx);
                n_metadata -= 1;
                match n_metadata {
                    0 => {
                        current_state = end_of_node(&mut nodes_in_process, &mut all_nodes, &mut metadata_idx, &mut n_metadata, &mut current_idx);
                        // println!("All metadata for {}, move to {}", current_idx, current_state);
                    }
                    _ => (),
                }
            },
            _ => (),
        }
        // if current_idx > 8 {
        //     break;
        // }
    }
    all_nodes
}

fn node_sum (node_index: usize, all_nodes: &Vec<TreeNode>) -> usize {
    if node_index >= all_nodes.len() {
        return 0;
    }
    let node = &all_nodes[node_index];
    if node.n_children == 0 {
        return node.metadata.iter().sum();
    }
    node.metadata.iter().map(|&i| {
        if i <= node.n_children {
            node_sum(node.idx_children[i - 1], all_nodes)
        } else {
            0
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn test_parsing () {
    //     assert_eq!(parse_nodes(&vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]), REF_NODES);
    // }
    #[test]
    fn test_sum() {
        let ref_nodes: Vec<TreeNode> = vec![
            TreeNode{n_children: 2, n_metadata: 3, metadata: vec![1, 1, 2], idx_children: vec![1, 2]},
            TreeNode{n_children: 0, n_metadata: 3, metadata: vec![10, 11, 12], idx_children: vec![]},
            TreeNode{n_children: 1, n_metadata: 1, metadata: vec![2], idx_children: vec![3]},
            TreeNode{n_children: 0, n_metadata: 1, metadata: vec![99], idx_children: vec![]},
        ];

        assert_eq!(node_sum(0, &ref_nodes), 66);
    }
}

fn main() {
    let input_data: Vec<usize> = read_inputs("inputs/day-8.txt")[0]
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    // println!("inputs = {:?}", input_data);
    // println!("my nodes: {:?}", all_nodes);
    let all_nodes = parse_nodes(&input_data);
    let meta_sum: usize = all_nodes
        .iter()
        .map(|node| node.metadata.iter().sum())
        .collect::<Vec<usize>>()
        .iter()
        .sum();
    println!("Metadata sum = {}", meta_sum);
    println!("Node 0 sum = {}", node_sum(0, &all_nodes));
}
