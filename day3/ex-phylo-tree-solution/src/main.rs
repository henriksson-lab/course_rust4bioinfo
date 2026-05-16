#[derive(Debug, Clone)]
enum Node {
    Leaf {
        name: String,
        branch_length: f64,
    },
    Internal {
        branch_length: f64,
        children: Vec<Node>,
    },
}

fn count_tips(node: &Node) -> usize {
    match node {
        Node::Leaf { .. } => 1,
        Node::Internal { children, .. } => {
            let mut total = 0;
            for child in children {
                total += count_tips(child);
            }
            total
        }
    }
}

fn max_depth(node: &Node) -> f64 {
    match node {
        Node::Leaf { branch_length, .. } => *branch_length,
        Node::Internal {
            branch_length,
            children,
        } => {
            let mut deepest: f64 = 0.0;
            for child in children {
                let d = max_depth(child);
                if d > deepest {
                    deepest = d;
                }
            }
            branch_length + deepest
        }
    }
}

fn total_branch_length(node: &Node) -> f64 {
    match node {
        Node::Leaf { branch_length, .. } => *branch_length,
        Node::Internal {
            branch_length,
            children,
        } => {
            let mut total = *branch_length;
            for child in children {
                total += total_branch_length(child);
            }
            total
        }
    }
}

fn leaf(name: &str, bl: f64) -> Node {
    Node::Leaf {
        name: name.to_string(),
        branch_length: bl,
    }
}

fn internal(bl: f64, children: Vec<Node>) -> Node {
    Node::Internal {
        branch_length: bl,
        children,
    }
}

fn main() {
    let tree = internal(
        0.0,
        vec![
            internal(0.3, vec![leaf("A", 0.1), leaf("B", 0.2)]),
            leaf("C", 0.5),
        ],
    );
    println!("tips:                {}", count_tips(&tree));
    println!("max depth:           {}", max_depth(&tree));
    println!("total branch length: {}", total_branch_length(&tree));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_tree() -> Node {
        internal(
            0.0,
            vec![
                internal(0.3, vec![leaf("A", 0.1), leaf("B", 0.2)]),
                leaf("C", 0.5),
            ],
        )
    }

    #[test]
    fn single_leaf_count() {
        assert_eq!(count_tips(&leaf("X", 0.1)), 1);
    }

    #[test]
    fn single_leaf_depth() {
        assert_eq!(max_depth(&leaf("X", 0.7)), 0.7);
    }

    #[test]
    fn single_leaf_total() {
        assert_eq!(total_branch_length(&leaf("X", 0.7)), 0.7);
    }

    #[test]
    fn small_tree_tips() {
        assert_eq!(count_tips(&small_tree()), 3);
    }

    #[test]
    fn small_tree_max_depth() {
        assert!((max_depth(&small_tree()) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn small_tree_total_branch_length() {
        assert!((total_branch_length(&small_tree()) - 1.1).abs() < 1e-9);
    }

    #[test]
    fn three_way_polytomy_tips() {
        let tree = internal(
            0.0,
            vec![leaf("A", 0.1), leaf("B", 0.1), leaf("C", 0.1)],
        );
        assert_eq!(count_tips(&tree), 3);
    }

    #[test]
    fn deep_skinny_tree() {
        let tree = internal(
            1.0,
            vec![internal(
                2.0,
                vec![internal(3.0, vec![leaf("tip", 4.0)])],
            )],
        );
        assert!((max_depth(&tree) - 10.0).abs() < 1e-9);
        assert_eq!(count_tips(&tree), 1);
    }
}
