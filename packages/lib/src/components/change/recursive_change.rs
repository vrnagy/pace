use crate::components::{component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO};

pub struct RecursiveChange {
    length: usize,
    ctx: ComponentContext,
    lifo: RecursiveLIFO,
}

impl RecursiveChange {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "RecursiveChange length must be >= 1");
        return RecursiveChange {
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
        return Some(last_value.unwrap() - first_value.unwrap());
    }
}
