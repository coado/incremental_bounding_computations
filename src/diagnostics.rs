use adapton::reflect::trace::*;

#[derive(Debug, Clone)]
struct ActualTraceCount(TraceCount);

impl Default for ActualTraceCount {
    fn default() -> Self {
        ActualTraceCount(trace_count_zero())
    }
}

#[derive(Debug, Default)]
pub(crate) struct Diagnostics {
    pub traces: Vec<Trace>,
    pub thunks_count: usize,
    pub cells_count: usize,
    pub trace_count: ActualTraceCount
}

impl Diagnostics {
    pub fn new(traces: Vec<Trace>) -> Self {
        let counts = trace_count(&traces, None);
        Diagnostics {
            traces,
            thunks_count: 0,
            cells_count: 0,
            trace_count: ActualTraceCount(counts)
        }
    }

    pub fn analyse(mut self) -> Self {
        self.trace_nodes_count();
        self
    }

    pub fn get_trace_count(&self) -> &TraceCount {
        &self.trace_count.0
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

    fn trace_edges(&mut self) {
        for tr in &self.traces {
            let edge = &tr.edge;
            match edge {
                EffectEdge::Fwd(next_edge) => {},
                EffectEdge::Bwd(_) => {},
                EffectEdge::None => {}
            }
        }
    }
}