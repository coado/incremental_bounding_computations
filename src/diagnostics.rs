use adapton::reflect::trace::*;

#[derive(Default, Debug)]
pub(crate) struct Diagnostics {
    traces: Vec<Trace>,
    pub thunks_count: usize,
    pub cells_count: usize,
}

impl Diagnostics {
    pub fn new(traces: Vec<Trace>) -> Self {
        Diagnostics {
            traces,
            thunks_count: 0,
            cells_count: 0,
        }
    }

    pub fn analyse(mut self) -> Self {
        self.trace_nodes_count();
        self
    }
        
    fn trace_nodes_count(&mut self) {
        for tr in &self.traces {    
            match tr.effect {
                Effect::Alloc(AllocCase::LocFresh, AllocKind::RefCell) => {
                    self.cells_count += 1;
                },
                Effect::Alloc(AllocCase::LocFresh, AllocKind::Thunk) => {
                    self.thunks_count += 1;
                },
                _ => {}
            }
        }
    }
}