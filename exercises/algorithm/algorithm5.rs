use std::collections::VecDeque;

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    // 从起点开始
    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        result.push(node);

        // 遍历邻居
        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let graph = vec![
            vec![1, 4],
            vec![0, 2, 3],
            vec![1],
            vec![1],
            vec![0],
        ];
        assert_eq!(bfs(&graph, 0), vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let graph = vec![
            vec![1],
            vec![0, 2],
            vec![1],
        ];
        assert_eq!(bfs(&graph, 2), vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_single_node() {
        let graph = vec![vec![]];
        assert_eq!(bfs(&graph, 0), vec![0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let graph = vec![
            vec![1],
            vec![0, 2],
            vec![1],
        ];
        assert_eq!(bfs(&graph, 0), vec![0, 1, 2]);
    }
}