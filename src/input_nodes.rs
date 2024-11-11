use adapton::macros::*;
use adapton::engine::*;

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

pub fn create_input_nodes(n: usize) -> Vec<Art<i32>> {
    let mut input_nodes = Vec::new();
    for _ in 0..n {
        input_nodes.push(cell!(0));
    }

    input_nodes
}

pub fn devide_and_conquer(nodes: &Vec<Art<i32>>, left: usize, right: usize) -> Art<i32> {
    if left == right {
        return nodes[left].clone();
    }

    let mid = left + (right - left) / 2;
    let left_res = devide_and_conquer(nodes, left, mid);
    let right_res = devide_and_conquer(nodes, mid + 1, right);

    thunk!(get!(left_res) + get!(right_res) + (fibonacci(0) as i32))
}

pub fn update_input_nodes(nodes: &Vec<Art<i32>>, n: usize) {
    for i in 0..n {
        set(&nodes[i], i as i32);
        get!(nodes[i]);
    }
}