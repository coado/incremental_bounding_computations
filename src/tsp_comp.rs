use std::{
    collections::{HashSet, VecDeque},
    hash::{Hash, Hasher},
    rc::Rc,
};

use depends::{
    derives::{Dependencies, Operation, Value},
    error::EarlyExit,
    Dependency, DerivedNode, InputNode, Resolve, SingleRef, TargetMut, UpdateDerived, UpdateInput,
};


// #[derive(Value, Default, Hash)]
// struct Vertex {
//     id: i32,
// }

// impl Vertex {
//     fn new(id: i32) -> Vertex {
//         Vertex { id }
//     }
// }

// impl UpdateInput for Vertex {
//     type Update = i32;

//     fn update_mut(&mut self, update: Self::Update) {
//         self.id = update;
//     }
// }

#[derive(Value, Default, Hash)]
pub struct PathLength {
    pub value: i32
}

impl PathLength {
    fn new(value: i32) -> PathLength {
        PathLength { value }
    }
}

impl UpdateInput for PathLength {
    type Update = i32;

    fn update_mut(&mut self, update: Self::Update) {
        self.value = update;
    }
}

#[derive(Dependencies)]
struct TwoPaths {
    lhs: PathLength,
    rhs: PathLength,
}

#[derive(Operation)]
struct Add;

impl UpdateDerived for Add {
    type Input<'a> = TwoPathsRef<'a> where Self: 'a;
    type Target<'a> = TargetMut<'a, PathLength> where Self: 'a;

    fn update_derived(input: Self::Input<'_>, mut target: Self::Target<'_>) -> Result<(), EarlyExit> {
        let TwoPathsRef { lhs, rhs } = input;
        target.value = lhs.value + rhs.value;
        Ok(())
    }
}

enum NodeWrapper {
    First(Rc<DerivedNode<TwoPathsDep<InputNode<PathLength>, InputNode<PathLength>>, Add, PathLength>>),
    Second(Rc<DerivedNode<TwoPathsDep<
        DerivedNode<TwoPathsDep<InputNode<PathLength>, InputNode<PathLength>>, Add, PathLength>, 
        DerivedNode<TwoPathsDep<InputNode<PathLength>, InputNode<PathLength>>, Add, PathLength>
    >, Add, PathLength>>),
    Third(Rc<DerivedNode<TwoPathsDep<
        DerivedNode<TwoPathsDep<InputNode<PathLength>, InputNode<PathLength>>, Add, PathLength>, 
        DerivedNode<TwoPathsDep<InputNode<PathLength>, InputNode<PathLength>>, Add, PathLength>
    >, Add, PathLength>>),
}

struct TSPPathLengthIncrementalComputation {
    input_nodes: Vec<Rc<InputNode<PathLength>>>,
}

impl TSPPathLengthIncrementalComputation {
    fn new(edges: Vec<i32>) -> Self {
        let mut input_nodes = Vec::new();
        
        edges.iter().for_each(|edge| {
            let node = InputNode::new(PathLength::new(*edge));
            input_nodes.push(node);
        });

        if input_nodes.len() % 2 == 1 {
            let dummy = InputNode::new(PathLength::new(0));
            input_nodes.push(dummy);
        }

        let mut nodes = input_nodes
            .chunks(2)
            .map(|chunk| {
                let lhs = Rc::clone(&chunk[0]);
                let rhs = Rc::clone(&chunk[1]);
                let derived = DerivedNode::new(
                    TwoPaths::init(lhs, rhs),
                    Add,
                    PathLength::default()
                );
                NodeWrapper::Simple(derived)
            }).collect::<VecDeque<NodeWrapper>>();
        
        // while nodes.len() > 1 {
        //     let lhs = nodes.pop_front().unwrap();
        //     let rhs = nodes.pop_front().unwrap();
        //     let derived = DerivedNode::new(
        //         TwoPaths::init(Rc::clone(&lhs), Rc::clone(&rhs)),
        //         Add,
        //         PathLength::default()
        //     );
        //     // nodes.push_back(derived);
        // }

        TSPPathLengthIncrementalComputation { input_nodes }        
    }
}