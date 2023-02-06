use crate::base::{
    asset::source::{Source, SourceKind},
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    pinescript::utils::{ps_diff, ps_div},
    ta::{
        highest_component::HighestComponent, lowest_component::LowestComponent,
        roc_component::RateOfChangeComponent, wma_component::WeightedMovingAverageComponent,
    },
};

pub struct WilliamsPercentRangeIndicatorConfig {
    pub src: Source,
    pub length: usize,
}

impl ComponentDefault for WilliamsPercentRangeIndicatorConfig {
    fn default(ctx: ComponentContext) -> Self {
        Self {
            length: 14,
            src: Source::from_kind(ctx.clone(), SourceKind::Close),
        }
    }
}

pub struct WilliamsPercentRangeIndicator {
    pub config: WilliamsPercentRangeIndicatorConfig,
    ctx: ComponentContext,
    highest: HighestComponent,
    lowest: LowestComponent,
}

impl WilliamsPercentRangeIndicator {
    pub fn new(ctx: ComponentContext, config: WilliamsPercentRangeIndicatorConfig) -> Self {
        return WilliamsPercentRangeIndicator {
            ctx: ctx.clone(),
            highest: HighestComponent::new(ctx.clone(), config.length),
            lowest: LowestComponent::new(ctx.clone(), config.length),
            config,
        };
    }

    pub fn next(&mut self) -> Option<f64> {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let src = self.config.src.get();

        let max = self.highest.next(ctx.high());
        let min = self.lowest.next(ctx.low());

        let pr = ps_div(ps_diff(src, max), ps_diff(max, min)).map(|x| x * 100.0);

        return pr;
    }
}
