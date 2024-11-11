#[macro_use] extern crate adapton;
mod graph;
mod tsp;
mod tsp_draw;
mod tsp_comp;

use std::rc::Rc;

use adapton::macros::*;
use adapton::engine::*;
use adapton::reflect;

const AL: [[i32; 5]; 5] = [
    [0, 1, 7, 6, 1],
    [1, 0, 1, 4, 9],
    [7, 1, 0, 1, 8],
    [6, 4, 1, 0, 1],
    [1, 9, 8, 1, 0],
];


fn main() {
    // tsp_draw::draw();

    manage::init_dcg();
    reflect::dcg_reflect_begin();

    let a1 = cell!(0);
    let a2 = cell!(1);
    let a3 = cell!(2);
    let a4 = cell!(3);
    let a5 = cell!(4);

    let a1_clone = a1.clone();
    let a2_clone = a2.clone();
    let a3_clone = a3.clone();
    let a4_clone = a4.clone();
    let a5_clone = a5.clone();

    let tmp = a3.clone();

    let b1 = thunk!(AL[get!(a1) as usize][get!(a2_clone) as usize]);
    let b2 = thunk!(AL[get!(a2) as usize][get!(a3_clone) as usize]);
    let b3 = thunk!(AL[get!(a3) as usize][get!(a4_clone) as usize]);
    let b4 = thunk!(AL[get!(a4) as usize][get!(a5_clone) as usize]);
    let b5 = thunk!(AL[get!(a1_clone) as usize][get!(a5) as usize]);
    
    let c1 = thunk!(get!(b1) + get!(b2));
    let c2 = thunk!(get!(b3) + get!(b4));

    let d1 = thunk!(get!(c1) + get!(c2));

    let e1 = thunk!(get!(d1) + get!(b5));

    println!("Result: {}", get!(e1));

    set(&tmp, 1);

    println!("Result2: {}", get!(e1));

    let traces = reflect::dcg_reflect_end();
    let counts = reflect::trace::trace_count(&traces, None);
    
    println!("Counts: {:?}", counts);

    // println!("Traces: {:?}", traces);

    
}
