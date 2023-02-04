use crate::{
    components::{
        batch_validator::recursive_batch_validator::RecursiveBatchValidator,
        component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO,
    },
    math::comparison::FloatComparison,
    ta::moving_average::{
        rma_component::RunningMovingAverageComponent, sma_component::SimpleMovingAverageComponent,
    },
};

pub struct WeightedMovingAverageComponent {
    pub length: usize,
    ctx: ComponentContext,
    value_lifo: RecursiveLIFO,
    batch_validator: RecursiveBatchValidator,
}

impl WeightedMovingAverageComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(
            length > 0,
            "WeightedMovingAverageComponent must have a length of at least 1"
        );
        return WeightedMovingAverageComponent {
            ctx: ctx.clone(),
            length,
            value_lifo: RecursiveLIFO::new(ctx.clone(), length),
            batch_validator: RecursiveBatchValidator::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();

        let (first_value, last_value, is_filled) = self.value_lifo.next(value);
        let is_valid = self.batch_validator.next(value);

        if !self.ctx.get().at_length(self.length) || !is_valid {
            return None;
        }

        let values = self.value_lifo.values_with_first();
        let mut weight: f64 = 0.0;
        let mut sum: f64 = 0.0;
        let mut norm: f64 = 0.0;

        for i in 0..values.len() {
            let value = values[values.len() - 1 - i].unwrap();
            weight = ((self.length - i) * self.length) as f64;
            norm += weight;
            sum += weight * value;
        }

        let wma = sum / norm;

        return Some(wma);
    }
}
