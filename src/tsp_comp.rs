use adapton::macros::*;
use adapton::engine::*;

pub struct TspComp<'a> {
    al: &'a Vec<Vec<i32>>,
    input_nodes: Vec<Art<i32>>,
    res: Art<i32>,
    n: usize
}

impl<'a> TspComp<'a> {
    fn new(al: &'static Vec<Vec<i32>>, n: usize) -> TspComp {
        manage::init_dcg();
        let input_nodes = (0..n).map(|_| {
            cell!(0)
        }).collect();

        let res = TspComp::calc_result(&input_nodes, al);
        
        TspComp {
            al,
            input_nodes,
            res,
            n
        }
    }

    pub fn update_input_nodes(&mut self, updates: Vec<(usize, i32)>) {
        for (idx, val) in updates {
            set(&self.input_nodes[idx], val);
        }
    }

    pub fn get_result(&self) -> i32 {
        get!(self.res)
    }

    fn calc_result(input_nodes: &Vec<Art<i32>>, al: &'static Vec<Vec<i32>>) -> Art<i32> {
        // first layer contains the input nodes, which are the indices of the nodes in the adjacency list
        // second layer retrieves edges from adjacency list
        let mut outputs = input_nodes.windows(2).map(|chunk| {
            let a = chunk[0].clone();
            let b = chunk[1].clone();
            let c = thunk!(al[get!(a) as usize][get!(b) as usize]);
            c
        }).collect::<Vec<Art<i32>>>();
        
        // subsequent layers sum up the edges
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
        println!("result: {}", tsp_comp.get_result());
        // assert_eq!(tsp_comp.get_result(), 10);
    }
}