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

/// Number of tips (leaves) under this node, inclusive.
fn count_tips(node: &Node) -> usize {
    // TODO:
    //   Leaf      -> 1
    //   Internal  -> sum of count_tips over each child
    let _ = node;
    0
}

/// Length of the longest path from this node to any descendant tip.
fn max_depth(node: &Node) -> f64 {
    // TODO:
    //   Leaf      -> *branch_length
    //   Internal  -> branch_length + max(max_depth(child) for child in children)
    let _ = node;
    0.0
}

/// Sum of every branch length in this subtree, including this node's own branch.
fn total_branch_length(node: &Node) -> f64 {
    // TODO:
    //   Leaf      -> *branch_length
    //   Internal  -> branch_length + sum of total_branch_length over each child
    let _ = node;
    0.0
}

// Helper constructors used by both main and the tests.
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
    // ((A:0.1, B:0.2):0.3, C:0.5);
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
        // ((A:0.1, B:0.2):0.3, C:0.5);
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
        // Both deepest paths give 0.5: (root)0.0 + 0.3 + 0.2 = 0.5, and
        // (root)0.0 + 0.5 = 0.5.
        assert!((max_depth(&small_tree()) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn small_tree_total_branch_length() {
        // 0.0 (root) + 0.3 + 0.1 + 0.2 + 0.5 = 1.1
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
        // Stack of single-child internals ending in one leaf.
        // (Pathological but legal; max_depth should be additive.)
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
