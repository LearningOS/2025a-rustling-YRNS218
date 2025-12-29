use std::collections::HashSet;

fn dfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut visit_order = Vec::new();
    dfs_helper(graph, start, &mut visited, &mut visit_order);
    visit_order
}

fn dfs_helper(
    graph: &Vec<Vec<usize>>,
    node: usize,
    visited: &mut HashSet<usize>,
    visit_order: &mut Vec<usize>,
) {
    visited.insert(node);
    visit_order.push(node);

    for &neighbor in &graph[node] {
        if !visited.contains(&neighbor) {
            dfs_helper(graph, neighbor, visited, visit_order);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_all_nodes_visited() {
        let graph = vec![
            vec![1, 4],
            vec![0, 2, 3],
            vec![1],
            vec![1],
            vec![0],
        ];
        assert_eq!(dfs(&graph, 0), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_dfs_different_start() {
        let graph = vec![
            vec![1],
            vec![0, 2],
            vec![1],
        ];
        assert_eq!(dfs(&graph, 2), vec![2, 1, 0]);
    }

    #[test]
    fn test_dfs_single_node() {
        let graph = vec![vec![]];
        assert_eq!(dfs(&graph, 0), vec![0]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let graph = vec![
            vec![1],
            vec![0, 2],
            vec![1],
        ];
        assert_eq!(dfs(&graph, 0), vec![0, 1, 2]);
    }
}