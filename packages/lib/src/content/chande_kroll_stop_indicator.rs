use crate::base::{
    components::{
        common::fixed_value_cache_component::FixedValueCacheComponent,
        component_context::ComponentContext, component_default::ComponentDefault,
    },
    pinescript::utils::ps_diff,
    ta::{
        atr_component::AverageTrueRangeComponent,
        bars::{compute_highest, compute_lowest},
        highest_component::HighestComponent,
        lowest_component::LowestComponent,
    },
};

pub struct ChandeKrollStopIndicatorConfig {
    pub p: usize,
    pub q: usize,
    pub x: f64,
}

impl ComponentDefault for ChandeKrollStopIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            p: 10,
            x: 1.0,
            q: 9,
        }
    }
}

pub struct ChandeKrollStopIndicatorResult {
    pub first_high_stop: Option<f64>,
    pub first_low_stop: Option<f64>,
    pub stop_long: Option<f64>,
    pub stop_short: Option<f64>,
}

pub struct ChandeKrollStopIndicator {
    pub config: ChandeKrollStopIndicatorConfig,
    ctx: ComponentContext,
    atr: AverageTrueRangeComponent,
    first_high_stop_highest_cache: FixedValueCacheComponent,
    first_low_stop_lowest_cache: FixedValueCacheComponent,
    first_high_stop_highest: HighestComponent,
    first_low_stop_lowest: LowestComponent,
    stop_short_highest: HighestComponent,
    stop_long_lowest: LowestComponent,
}

impl ChandeKrollStopIndicator {
    pub fn new(ctx: ComponentContext, config: ChandeKrollStopIndicatorConfig) -> Self {
        return ChandeKrollStopIndicator {
            ctx: ctx.clone(),
            atr: AverageTrueRangeComponent::new(ctx.clone(), config.p),
            first_high_stop_highest_cache: FixedValueCacheComponent::new(ctx.clone(), config.q),
            first_low_stop_lowest_cache: FixedValueCacheComponent::new(ctx.clone(), config.q),
            first_high_stop_highest: HighestComponent::new(ctx.clone(), config.p),
            first_low_stop_lowest: LowestComponent::new(ctx.clone(), config.p),
            stop_short_highest: HighestComponent::new(ctx.clone(), config.q),
            stop_long_lowest: LowestComponent::new(ctx.clone(), config.q),
            config,
        };
    }

    pub fn next(&mut self) -> ChandeKrollStopIndicatorResult {
        self.ctx.on_next();
        let ctx = self.ctx.get();

        let atr = self.atr.next();

        let first_high_stop_highest = self.first_high_stop_highest.next(ctx.high());
        let first_low_stop_lowest = self.first_low_stop_lowest.next(ctx.low());

        let (first_high_stop, first_low_stop) =
            match (first_high_stop_highest, first_low_stop_lowest, atr) {
                (Some(first_high_stop_highest), Some(first_low_stop_lowest), Some(atr)) => {
                    let first_high_stop = first_high_stop_highest - self.config.x * atr;
                    let first_low_stop = first_low_stop_lowest + self.config.x * atr;
                    (Some(first_high_stop), Some(first_low_stop))
                }
                _ => (None, None),
            };

        let stop_short = self.stop_short_highest.next(first_high_stop);
        let stop_long = self.stop_long_lowest.next(first_low_stop);

        return ChandeKrollStopIndicatorResult {
            first_high_stop,
            first_low_stop,
            stop_short,
            stop_long,
        };
    }
}
