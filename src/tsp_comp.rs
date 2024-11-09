use adapton::macros::*;
use adapton::engine::*;
use adapton::reflect;

pub struct TspComp {
    // al: &'static Vec<Vec<i32>>,
    input_nodes: Vec<Art<i32>>,
    res: Art<i32>,
    // n: usize,
    sealed: bool
}

impl TspComp {
    pub fn new(al: &'static Vec<Vec<i32>>, n: usize) -> TspComp {
        manage::init_dcg();
        reflect::dcg_reflect_begin();

        let input_nodes = (0..n).map(|_| {
            cell!(0)
        }).collect();

        println!("tsp_comp al rows: {}, al cols: {}", al.len(), al[0].len());

        let res = TspComp::calc_result(&input_nodes, al);
        
        TspComp {
            // al,
            input_nodes,
            res,
            // n,
            sealed: false
        }
    }

    pub fn update_input_nodes(&mut self, updates: Vec<(usize, i32)>) {
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
        let traces = reflect::dcg_reflect_end();
        let counts = reflect::trace::trace_count(&traces, None);

        // TODO: implement better diagnostics 
        println!("TspComp: traces: {:?}", counts);
    }

    fn ensure_unsealed(&mut self) {
        assert!(!self.sealed, "TspComp is sealed");
    }

    fn calc_result(input_nodes: &Vec<Art<i32>>, al: &'static Vec<Vec<i32>>) -> Art<i32> {
        // first layer contains the input nodes, which are the indices of the nodes in the adjacency list
        // second layer retrieves edges from adjacency list
        let mut outputs = input_nodes.windows(2).map(|chunk| {
            let a = chunk[0].clone();
            let b = chunk[1].clone();
            thunk!(al[get!(a) as usize][get!(b) as usize])
        }).collect::<Vec<Art<i32>>>();

        // last and first vertex
        let a = input_nodes[input_nodes.len() - 1].clone();
        let b = input_nodes[0].clone();
        let c = thunk!(al[get!(a) as usize][get!(b) as usize]);
        outputs.push(c);
        
        // subsequent layers sum up the edges
        // TODO: make it better
        while outputs.len() > 1 {
            let mut new_inputs = vec![];
        
            if outputs.len() % 2 == 1 {
                outputs.push(cell!(0));
            }
    
            let mut i = 1;
            while i < outputs.len() {
                let a = outputs[i - 1].clone();
                let b = outputs[i].clone();
                
                let c = thunk!(get!(a) + get!(b));
                new_inputs.push(c.clone());
    
                i += 2;
            }
    
            outputs = new_inputs.clone();
        }

        outputs[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let size = 6;
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
            (5, 4)
        ];

        tsp_comp.update_input_nodes(updates);
        assert_eq!(tsp_comp.get_result(), 24);

        let updates = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 0)
        ];
        tsp_comp.update_input_nodes(updates);
        assert_eq!(tsp_comp.get_result(), 5);
        tsp_comp.seal();
    }
}