use crate::base::{
    components::{
        common::fixnan_component::FixNanComponent, component_context::ComponentContext,
        component_default::ComponentDefault,
    },
    pinescript::utils::{ps_diff, ps_div},
    ta::{rma_component::RunningMovingAverageComponent, tr_component::TrueRangeComponent},
};

pub struct DirectionalMovementIndexIndicatorConfig {
    pub length: usize,
    pub lensig: usize,
}

impl ComponentDefault for DirectionalMovementIndexIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            length: 14,
            lensig: 14,
        }
    }
}

pub struct DirectionalMovementIndexIndicatorResult {
    pub plus: Option<f64>,
    pub minus: Option<f64>,
    pub adx: Option<f64>,
}

pub struct DirectionalMovementIndexIndicator {
    config: DirectionalMovementIndexIndicatorConfig,
    ctx: ComponentContext,
    true_range: TrueRangeComponent,
    true_range_rma: RunningMovingAverageComponent,
    plus_dm_rma: RunningMovingAverageComponent,
    minus_dm_rma: RunningMovingAverageComponent,
    plus_minus_diff_rma: RunningMovingAverageComponent,
    plus_fix_nan: FixNanComponent,
    minus_fix_nan: FixNanComponent,
    adx: RunningMovingAverageComponent,
}

pub static DIRECTIONAL_MOVEMENT_INDEX_MIN_VALUE: f64 = 0.0;
pub static DIRECTIONAL_MOVEMENT_INDEX_MAX_VALUE: f64 = 100.0;

impl DirectionalMovementIndexIndicator {
    pub fn new(ctx: ComponentContext, config: DirectionalMovementIndexIndicatorConfig) -> Self {
        return DirectionalMovementIndexIndicator {
            ctx: ctx.clone(),
            true_range: TrueRangeComponent::new(ctx.clone(), false),
            true_range_rma: RunningMovingAverageComponent::new(ctx.clone(), config.length),
            plus_dm_rma: RunningMovingAverageComponent::new(ctx.clone(), config.length),
            minus_dm_rma: RunningMovingAverageComponent::new(ctx.clone(), config.length),
            plus_minus_diff_rma: RunningMovingAverageComponent::new(ctx.clone(), config.lensig),
            plus_fix_nan: FixNanComponent::new(ctx.clone()),
            minus_fix_nan: FixNanComponent::new(ctx.clone()),
            adx: RunningMovingAverageComponent::new(ctx.clone(), config.lensig),
            config,
        };
    }

    pub fn next(&mut self) -> DirectionalMovementIndexIndicatorResult {
        self.ctx.assert();

        let ctx = self.ctx.get();
        let high = ctx.high();
        let low = ctx.low();
        let prev_high = ctx.prev_high(1);
        let prev_low = ctx.prev_low(1);
        let prev_close = ctx.prev_close(1);

        let up = ps_diff(high, prev_high);
        let down = ps_diff(prev_low, low);

        let plus_dm = match (up, down) {
            (Some(up), Some(down)) => {
                if up > down && up > 0.0 {
                    Some(up)
                } else {
                    Some(0.0)
                }
            }
            _ => None,
        };

        let minus_dm = match (up, down) {
            (Some(up), Some(down)) => {
                if down > up && down > 0.0 {
                    Some(down)
                } else {
                    Some(0.0)
                }
            }
            _ => None,
        };

        let true_range = self.true_range.next();
        let true_range_rma = self.true_range_rma.next(true_range);

        let plus_dm_rma = self.plus_dm_rma.next(plus_dm);
        let minus_dm_rma = self.minus_dm_rma.next(minus_dm);

        let plus = ps_div(plus_dm_rma, true_range_rma).map(|x| x * 100.0);
        let minus = ps_div(minus_dm_rma, true_range_rma).map(|x| x * 100.0);

        let plus = self.plus_fix_nan.next(plus);
        let minus = self.minus_fix_nan.next(minus);

        let adx: Option<f64> = match (plus, minus) {
            (Some(plus), Some(minus)) => {
                Some((plus - minus).abs() / (if plus == -minus { 0.0 } else { plus + minus }))
            }
            _ => None,
        };
        let adx = self.adx.next(adx).map(|x| x * 100.0);

        return DirectionalMovementIndexIndicatorResult { plus, minus, adx };
    }
}
