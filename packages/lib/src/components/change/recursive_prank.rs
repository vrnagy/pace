use crate::components::{
    batch_validator::recursive_batch_validator::RecursiveBatchValidator,
    component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO,
};

pub struct RecursivePercentRank {
    length: usize,
    ctx: ComponentContext,
    count: f64,
    value_lifo: RecursiveLIFO,
}

impl RecursivePercentRank {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(length >= 1, "RecursivePercentRank length must be >= 1");
        return RecursivePercentRank {
            ctx: ctx.clone(),
            length,
            count: 0.0,
            value_lifo: RecursiveLIFO::new(ctx.clone(), length + 1),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();

        if value.is_none() || !self.ctx.get().at_length(self.length + 1) {
            self.value_lifo.next(value);
            return None;
        }

        let last_value = value.unwrap();
        let mut count: f64 = 0.0;

        let values = self.value_lifo.values();

        let count = values
            .iter()
            .filter(|v| {
                if let Some(v) = v {
                    return v <= &last_value;
                }
                return false;
            })
            .count() as f64;

        let percent = count / self.length as f64 * 100.0;

        self.value_lifo.next(value);

        return Some(percent);
    }
}
