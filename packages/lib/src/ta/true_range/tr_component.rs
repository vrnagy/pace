use crate::{
    components::component_context::ComponentContext,
    ta::moving_average::rma_component::RunningMovingAverageComponent,
};

use super::true_range::TrueRange;

pub struct TrueRangeComponent {
    pub biased: bool,
    ctx: ComponentContext,
}

impl TrueRangeComponent {
    pub fn new(ctx: ComponentContext, biased: bool) -> Self {
        return TrueRangeComponent {
            ctx: ctx.clone(),
            biased,
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        let ctx = self.ctx.get();
        let (prev_high, prev_low, prev_close) = if ctx.current_tick == 0 {
            (None, None, None)
        } else {
            (ctx.prev_high(1), ctx.prev_low(1), ctx.prev_close(1))
        };
        let true_range = TrueRange::tr(
            ctx.high().unwrap(),
            ctx.low().unwrap(),
            prev_high,
            prev_low,
            prev_close,
            self.biased,
        );
        return true_range;
    }
}
