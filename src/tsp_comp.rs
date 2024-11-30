use adapton::macros::*;
use adapton::engine::*;
use adapton::reflect;

pub struct TspComp {
    input_nodes: Vec<Art<i32>>,
    res: Art<i32>,
    sealed: bool
}

impl TspComp {
    pub fn new(al: &'static Vec<Vec<i32>>, n: usize) -> TspComp {
        manage::init_dcg();

        if cfg!(feature = "traces") {
            println!("TspComp: traces enabled");
            reflect::dcg_reflect_begin();
        }

        let input_nodes = (0..n).map(|_| {
            cell!(0)
        }).collect();

        let res = TspComp::create_computation_graph(&input_nodes, al);
        
        TspComp {
            input_nodes,
            res,
            sealed: false
        }
    }

    pub fn update_input_nodes(&mut self, updates: Vec<(usize, i32)>) {
        // println!("tsp_comp update_input_nodes: {:?}", updates);
        self.ensure_unsealed();
        for (idx, val) in updates {
            set(&self.input_nodes[idx], val);
        }
    }

    pub fn get_result(&self) -> i32 {
        get!(self.res)
    }

    pub fn seal(&mut self) {
        self.ensure_unsealed();
        self.sealed = true;

        if cfg!(feature = "traces") {
            let traces = reflect::dcg_reflect_end();
            let counts = reflect::trace::trace_count(&traces, None);
            println!("TspComp: traces: {:?}", counts);
            // println!("Traces: {:?}", traces);
        }    
    }

    fn ensure_unsealed(&mut self) {
        assert!(!self.sealed, "TspComp is sealed");
    }

    fn create_computation_graph(input_nodes: &Vec<Art<i32>>, al: &'static Vec<Vec<i32>>) -> Art<i32> {
        // first layer contains the input nodes, which are the indices of the nodes in the adjacency list
        // second layer retrieves edges from adjacency list
        let mut outputs = input_nodes.windows(2).map(|chunk| {
            let a = chunk[0].clone();
            let b = chunk[1].clone();
            thunk!(al[get!(a) as usize][get!(b) as usize])
        }).collect::<Vec<Art<i32>>>();

        // last and first vertex
        let last = input_nodes[input_nodes.len() - 1].clone();
        let first: Art<i32> = input_nodes[0].clone();
        let closing_connection = thunk!(al[get!(last) as usize][get!(first) as usize]);
        outputs.push(closing_connection);

        fn devide_and_conquer(nodes: &Vec<Art<i32>>, left: usize, right: usize) -> Art<i32> {
            if left == right {
                return nodes[left].clone();
            }
    
            let mid = left + (right - left) / 2;
            let left_res = devide_and_conquer(nodes, left, mid);
            let right_res = devide_and_conquer(nodes, mid + 1, right);
    
            thunk!(get!(left_res) + get!(right_res))
        }
        
        // subsequent layers sum up the edges
        devide_and_conquer(&outputs, 0, outputs.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let size = 5;
        let al = Box::new(vec![
            vec![0, 1, 7, 6, 1],
            vec![1, 0, 1, 4, 9],
            vec![7, 1, 0, 1, 8],
            vec![6, 4, 1, 0, 1],
            vec![1, 9, 8, 1, 0]
        ]);
        let static_al: &'static mut Vec<Vec<i32>> = Box::leak(al);

        let mut tsp_comp = TspComp::new(static_al, size);
        assert_eq!(tsp_comp.get_result(), 0);

        let updates = vec![
            (0, 4),
            (1, 3),
            (2, 0),
            (3, 2),
            (4, 1),
        ];

        tsp_comp.update_input_nodes(updates);
        assert_eq!(tsp_comp.get_result(), 24);

        let updates = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
        ];
        tsp_comp.update_input_nodes(updates);
        assert_eq!(tsp_comp.get_result(), 5);
        tsp_comp.seal();
    }
}