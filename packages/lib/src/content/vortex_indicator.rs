use crate::base::{
    asset::source::{Source, SourceKind},
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    ta::{
        atr_component::AverageTrueRangeComponent, ma::MovingAverageKind,
        ma_component::MovingAverageComponent, sum_component::SumComponent,
    },
};

pub struct VortexIndicatorConfig {
    pub length: usize,
}

impl ComponentDefault for VortexIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self { length: 14 }
    }
}

pub struct VortexIndicatorResult {
    pub plus: Option<f64>,
    pub minus: Option<f64>,
}

pub struct VortexIndicator {
    config: VortexIndicatorConfig,
    ctx: ComponentContext,
    vmp_sum: SumComponent,
    vmm_sum: SumComponent,
    atr_sum: SumComponent,
    atr: AverageTrueRangeComponent,
}

impl VortexIndicator {
    pub fn new(ctx: ComponentContext, config: VortexIndicatorConfig) -> Self {
        return VortexIndicator {
            ctx: ctx.clone(),
            vmp_sum: SumComponent::new(ctx.clone(), config.length),
            vmm_sum: SumComponent::new(ctx.clone(), config.length),
            atr_sum: SumComponent::new(ctx.clone(), config.length),
            atr: AverageTrueRangeComponent::new(ctx.clone(), 1),
            config,
        };
    }

    pub fn next(&mut self) -> VortexIndicatorResult {
        self.ctx.assert();

        let ctx = self.ctx.get();
        let current_tick = ctx.current_tick;
        let high = ctx.high();
        let low = ctx.low();
        let (prev_high, prev_low) = if current_tick > 0 {
            (ctx.prev_high(1), ctx.prev_low(1))
        } else {
            (None, None)
        };

        let high_prev_low_diff = match (high, prev_low) {
            (Some(high), Some(prev_low)) => Some((high - prev_low).abs()),
            _ => None,
        };

        let low_prev_high_diff = match (low, prev_high) {
            (Some(low), Some(prev_high)) => Some((low - prev_high).abs()),
            _ => None,
        };

        let vmp = self.vmp_sum.next(high_prev_low_diff);
        let vmm = self.vmm_sum.next(low_prev_high_diff);

        let atr = self.atr.next();
        let str = self.atr_sum.next(atr);

        let (vip, vim) = match (vmp, vmm, str) {
            (Some(vmp), Some(vmm), Some(str)) => {
                if str == 0.0 {
                    (None, None)
                } else {
                    (Some(vmp / str), Some(vmm / str))
                }
            }
            _ => (None, None),
        };

        return VortexIndicatorResult {
            plus: vip,
            minus: vim,
        };
    }
}
