use crate::{
    components::{
        component_context::ComponentContext, source::Source, sum::recursive_sum::RecursiveSum,
    },
    ta::{
        bars::utils::BarUtils,
        moving_average::{ma::MovingAverageKind, ma_component::MovingAverageComponent},
        true_range::atr_component::AverageTrueRangeComponent,
    },
};

pub struct ChoppinessIndexIndicatorConfig {
    pub length: usize,
}

pub struct ChoppinessIndexIndicatorResult {
    pub value: Option<f64>,
}

pub struct ChoppinessIndexIndicator {
    pub config: ChoppinessIndexIndicatorConfig,
    ctx: ComponentContext,
    atr: AverageTrueRangeComponent,
    atr_sum: RecursiveSum,
    log10_length: f64,
}

impl ChoppinessIndexIndicator {
    pub fn new(ctx: ComponentContext, config: ChoppinessIndexIndicatorConfig) -> Self {
        return ChoppinessIndexIndicator {
            ctx: ctx.clone(),
            atr: AverageTrueRangeComponent::new(ctx.clone(), 1),
            atr_sum: RecursiveSum::new(ctx.clone(), config.length),
            log10_length: f64::log10(config.length as f64),
            config,
        };
    }

    pub fn next(&mut self) -> ChoppinessIndexIndicatorResult {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let atr = self.atr.next();
        let atr_sum = self.atr_sum.next(atr);

        if (atr_sum.is_none() || !ctx.at_length(self.config.length)) {
            return ChoppinessIndexIndicatorResult { value: None };
        }

        let highest = BarUtils::highest(ctx.prev_highs(self.config.length));
        let lowest = BarUtils::lowest(ctx.prev_lows(self.config.length));

        let chop: Option<f64> = match (highest, lowest) {
            (Some(highest), Some(lowest)) => {
                let diff = highest - lowest;
                if diff == 0.0 {
                    None
                } else {
                    Some(100.0 * f64::log10(atr_sum.unwrap() / diff) / self.log10_length)
                }
            }
            _ => None,
        };

        return ChoppinessIndexIndicatorResult { value: chop };
    }
}
