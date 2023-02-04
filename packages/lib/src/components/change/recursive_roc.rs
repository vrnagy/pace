use crate::components::{component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO};

pub struct RecursiveRateOfChange {
    length: usize,
    ctx: ComponentContext,
    lifo: RecursiveLIFO,
}

impl RecursiveRateOfChange {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "RecursiveRateOfChange length must be >= 1");
        return RecursiveRateOfChange {
            ctx: ctx.clone(),
            length,
            lifo: RecursiveLIFO::new(ctx.clone(), length + 1),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let (first_value, last_value, is_filled) = self.lifo.next(value);

        if !is_filled || first_value.is_none() || last_value.is_none() {
            return None;
        }
        let first_value = first_value.unwrap();
        if first_value == 0.0 {
            return None;
        }
        let last_value = last_value.unwrap();
        return Some(100.0 * (last_value - first_value) / first_value);
    }
}
