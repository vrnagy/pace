use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    pinescript::utils::{ps_diff, ps_div},
    ta::{
        atr_component::AverageTrueRangeComponent,
        bars::{compute_highest, compute_lowest},
        highest_component::HighestComponent,
        lowest_component::LowestComponent,
        sum_component::SumComponent,
    },
};

pub struct ChoppinessIndexIndicatorConfig {
    pub length: usize,
}

impl ComponentDefault for ChoppinessIndexIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self { length: 14 }
    }
}

pub struct ChoppinessIndexIndicator {
    pub config: ChoppinessIndexIndicatorConfig,
    ctx: ComponentContext,
    atr: AverageTrueRangeComponent,
    atr_sum: SumComponent,
    log10_length: f64,
    highest: HighestComponent,
    lowest: LowestComponent,
}

impl ChoppinessIndexIndicator {
    pub fn new(ctx: ComponentContext, config: ChoppinessIndexIndicatorConfig) -> Self {
        return ChoppinessIndexIndicator {
            ctx: ctx.clone(),
            atr: AverageTrueRangeComponent::new(ctx.clone(), 1),
            atr_sum: SumComponent::new(ctx.clone(), config.length),
            log10_length: f64::log10(config.length as f64),
            highest: HighestComponent::new(ctx.clone(), config.length),
            lowest: LowestComponent::new(ctx.clone(), config.length),
            config,
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let atr = self.atr.next();
        let atr_sum = self.atr_sum.next(atr);

        let highest = self.highest.next(ctx.high());
        let lowest = self.lowest.next(ctx.low());

        let chop: Option<f64> = match (atr_sum, highest, lowest) {
            (Some(atr_sum), Some(highest), Some(lowest)) => {
                let diff = highest - lowest;
                if diff == 0.0 {
                    None
                } else {
                    Some(100.0 * f64::log10(atr_sum / diff) / self.log10_length)
                }
            }
            _ => None,
        };

        return chop;
    }
}
