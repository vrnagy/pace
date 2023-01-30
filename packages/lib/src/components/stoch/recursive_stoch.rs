use crate::components::{component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO};

use super::stoch::compute_stoch;

pub struct RecursiveStoch {
    length: usize,
    ctx: ComponentContext,
    prev_stoch: Option<f64>,
    high_lifo: RecursiveLIFO,
    low_lifo: RecursiveLIFO,
}

impl RecursiveStoch {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "RecursiveStoch length must be >= 1");
        return RecursiveStoch {
            ctx: ctx.clone(),
            length,
            prev_stoch: None,
            high_lifo: RecursiveLIFO::new(ctx.clone(), length + 1),
            low_lifo: RecursiveLIFO::new(ctx.clone(), length + 1),
        };
    }

    pub fn next(&mut self, value: Option<f64>, high: Option<f64>, low: Option<f64>) -> Option<f64> {
        self.ctx.assert();
        let ctx = self.ctx.get();

        self.high_lifo.next(high);
        self.low_lifo.next(low);

        if !ctx.at_length(self.length) {
            return None;
        }

        let stoch = compute_stoch(
            value,
            self.high_lifo.values(),
            self.low_lifo.values(),
            self.prev_stoch,
        );
        self.prev_stoch = stoch;

        return stoch;
    }
}
