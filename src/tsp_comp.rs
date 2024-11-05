use adapton::macros::*;
use adapton::engine::*;

fn generate() {
    let edges = [10, 20, 30, 40, 50, 60];

    let inputs = edges.map(|x| {
        let cell = cell!(x);
        cell
    });

    let mut outputs = Vec::from(inputs.clone());

    while outputs.len() > 1 {
        let mut new_inputs = vec![];

        print!("Outputs: {}", outputs.len());

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

    print!("Result: {}\n", get!(outputs[0]));
}

pub fn run() {
    manage::init_dcg();

    generate();

    // let num = cell!(42);
    // let den = cell!(2);

    // // In Rust, cloning is explicit:
    // let den2 = den.clone(); // clone _global reference_ to cell.
    // let den3 = den.clone(); // clone _global reference_ to cell, again.

    // // Two subcomputations: The division, and a check thunk with a conditional expression
    // let div   = thunk![ get!(num) / get!(den) ];
    // let check = thunk![ if get!(den2) == 0 { None } else { Some(get!(div)) } ];

    // print!("num: {}\n", get!(check).unwrap());

    // set(&den3, 4);

    // print!("num: {}\n", get!(check).unwrap());
}