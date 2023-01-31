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

pub struct DonchianChannelsIndicatorConfig {
    pub length: usize,
}

pub struct DonchianChannelsIndicatorResult {
    pub upper: Option<f64>,
    pub basis: Option<f64>,
    pub lower: Option<f64>,
}

pub struct DonchianChannelsIndicator {
    pub config: DonchianChannelsIndicatorConfig,
    ctx: ComponentContext,
}

impl DonchianChannelsIndicator {
    pub fn new(ctx: ComponentContext, config: DonchianChannelsIndicatorConfig) -> Self {
        return DonchianChannelsIndicator {
            ctx: ctx.clone(),
            config,
        };
    }

    pub fn next(&mut self) -> DonchianChannelsIndicatorResult {
        self.ctx.assert();
        let ctx = self.ctx.get();

        if (!ctx.at_length(self.config.length)) {
            return DonchianChannelsIndicatorResult {
                upper: None,
                basis: None,
                lower: None,
            };
        }

        let upper = BarUtils::highest(ctx.prev_highs(self.config.length));
        let lower = BarUtils::lowest(ctx.prev_lows(self.config.length));
        let basis = match (upper, lower) {
            (Some(upper), Some(lower)) => Some((upper + lower) / 2.0),
            _ => None,
        };

        return DonchianChannelsIndicatorResult {
            upper,
            basis,
            lower,
        };
    }
}
