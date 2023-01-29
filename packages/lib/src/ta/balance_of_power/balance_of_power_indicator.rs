use crate::{
    components::{component_context::ComponentContext, source::Source},
    ta::{
        bars::utils::BarUtils,
        moving_average::{ma::MovingAverageKind, ma_component::MovingAverageComponent},
    },
};

pub struct BalanceOfPowerIndicatorResult {
    pub value: Option<f64>,
}

pub struct BalanceOfPowerIndicator {
    ctx: ComponentContext,
}

impl BalanceOfPowerIndicator {
    pub fn new(ctx: ComponentContext) -> Self {
        return BalanceOfPowerIndicator { ctx: ctx.clone() };
    }

    pub fn next(&mut self) -> BalanceOfPowerIndicatorResult {
        self.ctx.assert();

        let ctx = self.ctx.get();

        let close = ctx.close();
        let open = ctx.open();
        let high = ctx.high();
        let low = ctx.low();

        let value = match (close, open, high, low) {
            (Some(close), Some(open), Some(high), Some(low)) => {
                if high == low {
                    return BalanceOfPowerIndicatorResult { value: None };
                }

                return BalanceOfPowerIndicatorResult {
                    value: Some((close - open) / (high - low)),
                };
            }
            _ => None,
        };

        return BalanceOfPowerIndicatorResult { value };
    }
}
