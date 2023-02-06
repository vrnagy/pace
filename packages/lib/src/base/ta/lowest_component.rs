use crate::base::{
    components::{
        common::fixed_value_cache_component::FixedValueCacheComponent,
        component_context::ComponentContext,
    },
    ta::{
        ema_component::ExponentialMovingAverageComponent,
        rma_component::RunningMovingAverageComponent, sma_component::SimpleMovingAverageComponent,
    },
};

use super::bars::compute_lowest;

pub struct LowestComponent {
    length: usize,
    ctx: ComponentContext,
    input_cache: FixedValueCacheComponent,
}

impl LowestComponent {
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        return LowestComponent {
            ctx: ctx.clone(),
            length,
            input_cache: FixedValueCacheComponent::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.on_next();
        let ctx = self.ctx.get();

        self.input_cache.next(value);

        if !self.ctx.at_length(self.length) {
            return None;
        }

        let values = self.input_cache.all();
        let highest = compute_lowest(values);

        return highest;
    }
}
