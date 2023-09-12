//chech maximum_path_sum_ii for a less naive and better solution, this was mostly used to practice a tree implementation in rust
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    data: i32,
    left_child: Option<Rc<Node>>,
    right_child: Option<Rc<Node>>,
}

fn pyramid_to_tree(pyramid: &str) -> Rc<Node> {
    let mut child_nodes: Vec<Rc<Node>> = Vec::new();
    let line = pyramid.lines().rev().nth(0).unwrap();
    for num in line.trim().split(" ") {
        let child = Rc::new(Node {
            data: num.parse().unwrap(),
            left_child: None,
            right_child: None,
        });
        child_nodes.push(child);
    }
    for line in pyramid.lines().rev().skip(1) {
        let mut parent_nodes: Vec<Rc<Node>> = Vec::new();
        for (index, num) in line.trim().split(" ").enumerate() {
            let parent = Rc::new(Node {
                data: num.parse().unwrap(),
                left_child: Some(child_nodes[index].clone()),
                right_child: Some(child_nodes[index+1].clone()),
            });
            
            parent_nodes.push(parent);
        }
        child_nodes = parent_nodes;
    }
    child_nodes[0].to_owned()
}

fn max_path_sum(node: Rc<Node>, sum: i32, max_sum: &mut i32, total_routes: &mut i32) {
    if let Some(left_child) = node.left_child.clone() {
        max_path_sum(left_child, sum+node.data, max_sum, total_routes);
    }
    if let Some(right_child) = node.right_child.clone() {
        max_path_sum(right_child, sum+node.data, max_sum, total_routes);
    }
    if let None = node.left_child {
        if sum + node.data > *max_sum {
            *max_sum = sum + node.data;
        }
    }
}

fn main() {
    let pyramid = "75
    95 64
    17 47 82
    18 35 87 10
    20 04 82 47 65
    19 01 23 75 03 34
    88 02 77 73 07 63 67
    99 65 04 28 06 16 70 92
    41 41 26 56 83 40 80 70 33
    41 48 72 33 47 32 37 16 94 29
    53 71 44 65 25 43 91 52 97 51 14
    70 11 33 28 77 73 17 78 39 68 17 57
    91 71 52 38 17 14 91 43 58 50 27 29 48
    63 66 04 68 89 53 67 30 73 16 69 87 40 31
    04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";
    let tree_root = pyramid_to_tree(pyramid);
    let mut max_sum = 0;
    let mut total_routes = 0;
    max_path_sum(tree_root, 0, &mut max_sum, &mut total_routes);
    println!("{} {}", max_sum, total_routes);
}
