use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    pinescript::utils::{ps_add, ps_diff},
    ta::{
        bars::{compute_highest, compute_lowest},
        highest_component::HighestComponent,
        lowest_component::LowestComponent,
    },
};

pub struct DonchianChannelsIndicatorConfig {
    pub length: usize,
}

impl ComponentDefault for DonchianChannelsIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self { length: 20 }
    }
}

pub struct DonchianChannelsIndicatorResult {
    pub upper: Option<f64>,
    pub basis: Option<f64>,
    pub lower: Option<f64>,
}

pub struct DonchianChannelsIndicator {
    pub config: DonchianChannelsIndicatorConfig,
    ctx: ComponentContext,
    highest: HighestComponent,
    lowest: LowestComponent,
}

impl DonchianChannelsIndicator {
    pub fn new(ctx: ComponentContext, config: DonchianChannelsIndicatorConfig) -> Self {
        return DonchianChannelsIndicator {
            ctx: ctx.clone(),
            highest: HighestComponent::new(ctx.clone(), config.length),
            lowest: LowestComponent::new(ctx.clone(), config.length),
            config,
        };
    }

    pub fn next(&mut self) -> DonchianChannelsIndicatorResult {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let upper = self.highest.next(ctx.high());
        let lower = self.lowest.next(ctx.low());

        let basis = ps_add(upper, lower).map(|x| x / 2.0);

        return DonchianChannelsIndicatorResult {
            upper,
            basis,
            lower,
        };
    }
}
