

mod test {
    use adapton::macros::*;
    use adapton::engine::*;
    use adapton::reflect;
    use crate::diagnostics::Diagnostics;

    /**
     * This test is a naive implementation of the firewall graph
     * cell(a) -> thunk(t) -> cell(b) -> thunk(h)
     */
    #[test]
    fn naive_firewall() {
        manage::init_dcg();
        reflect::dcg_reflect_begin();

        let a = cell!([a] 2);
        let a_clone = a.clone();
        let t = thunk!([t]{
            let x = get!(a_clone);
            cell!([b] i32::pow(x, 2))
        });

        let h = thunk!([h] {
            let b = force(&t);
            let b_clone = b.clone();
            let x = get!(b_clone);
            println!("UPDATING");
            format!("{:?}", x)
        });

        let h_clone = h.clone();
        let res = get!(h_clone);
        println!("Result: {:?}", res);
        set(&a, -2);
        let res = get!(h_clone);
        println!("Result: {:?}", res);
        
        let traces = reflect::dcg_reflect_end();
        let diagnostics = Diagnostics::new(traces).analyse();

        println!("Traces: {:?}", diagnostics.trace_count);
    }

    #[test]
    fn firewall_testing() {
        manage::init_dcg();
        let a = cell!([a] 2);
        let b = cell!([b] 3);
        let c = cell!([c] 4);
        let d = cell!([d] 5);

        let a_clone = a.clone();
        let b_clone = b.clone();
        let c_clone = c.clone();
        let d_clone = d.clone();

        let t1 = thunk!([t1] {
            let x = get!(a_clone);
            cell!([a1] i32::pow(x, 2))
        });

        let t2 = thunk!([t2] {
            let x = get!(b_clone);
            cell!([b1] i32::pow(x, 2))
        });

        let t3 = thunk!([t3] {
            let x = get!(c_clone);
            cell!([c1] i32::pow(x, 2))
        });

        let t4 = thunk!([t4] {
            let x = get!(d_clone);
            cell!([d1] i32::pow(x, 2))
        });

        let h = thunk!([h] {
            let a1 = force(&t1);
            let b1 = force(&t2);
            let c1 = force(&t3);
            let d1 = force(&t4);

            let a1_val = get!(a1);
            let b1_val = get!(b1);
            let c1_val = get!(c1);
            let d1_val = get!(d1);

            println!("UPDATING");

            format!("{:?}", a1_val + b1_val + c1_val + d1_val)
        });

        let res = get!(h);
        println!("Result: {:?}", res);

        set(&a, -2);
        set(&b, -3);
        set(&c, -4);
        set(&d, -5);

        let res = get!(h);
        println!("Result: {:?}", res);

        set(&a, 4);
        set(&b, 6);
        set(&c, 8);
        set(&d, 10);

        let res = get!(h);
        println!("Result: {:?}", res);

        
    }
}