use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    pinescript::utils::{ps_diff, ps_div, ps_max, ps_min, ps_nz},
    ta::{
        ema_component::ExponentialMovingAverageComponent, ma::MovingAverageKind,
        ma_component::MovingAverageComponent, sum_component::SumComponent,
    },
};

pub struct UltimateOscillatorIndicatorConfig {
    pub short_length: usize,
    pub mid_length: usize,
    pub long_length: usize,
}

impl ComponentDefault for UltimateOscillatorIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            short_length: 7,
            mid_length: 14,
            long_length: 28,
        }
    }
}

pub struct UltimateOscillatorIndicator {
    config: UltimateOscillatorIndicatorConfig,
    ctx: ComponentContext,
    short_sum_bp: SumComponent,
    short_sum_tr: SumComponent,
    mid_sum_bp: SumComponent,
    mid_sum_tr: SumComponent,
    long_sum_bp: SumComponent,
    long_sum_tr: SumComponent,
}

impl UltimateOscillatorIndicator {
    pub fn new(ctx: ComponentContext, config: UltimateOscillatorIndicatorConfig) -> Self {
        return UltimateOscillatorIndicator {
            ctx: ctx.clone(),
            short_sum_bp: SumComponent::new(ctx.clone(), config.short_length),
            short_sum_tr: SumComponent::new(ctx.clone(), config.short_length),
            mid_sum_bp: SumComponent::new(ctx.clone(), config.mid_length),
            mid_sum_tr: SumComponent::new(ctx.clone(), config.mid_length),
            long_sum_bp: SumComponent::new(ctx.clone(), config.long_length),
            long_sum_tr: SumComponent::new(ctx.clone(), config.long_length),
            config,
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();

        let ctx = self.ctx.get();
        let high = ctx.high();
        let low = ctx.low();
        let close = ctx.close();
        let current_tick = ctx.current_tick;

        let prev_close = if current_tick > 0 {
            ctx.prev_close(1)
        } else {
            None
        };

        let high_ = ps_max(high, prev_close);
        let low_ = ps_min(low, prev_close);
        let bp = ps_diff(close, low_);
        let tr_ = ps_diff(high_, low_);

        let fast_bp_sum = self.short_sum_bp.next(bp);
        let fast_tr_sum = self.short_sum_tr.next(tr_);

        let mid_bp_sum = self.mid_sum_bp.next(bp);
        let mid_tr_sum = self.mid_sum_tr.next(tr_);

        let slow_bp_sum = self.long_sum_bp.next(bp);
        let slow_tr_sum = self.long_sum_tr.next(tr_);

        let fast = ps_div(fast_bp_sum, fast_tr_sum);
        let mid = ps_div(mid_bp_sum, mid_tr_sum);
        let slow = ps_div(slow_bp_sum, slow_tr_sum);

        let uo = match (fast, mid, slow) {
            (Some(fast), Some(mid), Some(slow)) => {
                Some(100.0 * (4.0 * fast + 2.0 * mid + slow) / 7.0)
            }
            _ => None,
        };

        return uo;
    }
}
