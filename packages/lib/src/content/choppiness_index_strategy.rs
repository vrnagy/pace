use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::trade::TradeDirection,
    ta::{
        cross::CrossMode, cross_component::CrossComponent,
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
        rsi_component::RelativeStrengthIndexComponentMetadata,
    },
};

use super::{
    aroon_indicator::AroonIndicatorResult,
    relative_strength_index_indicator::RelativeStrengthIndexIndicator,
};

pub struct ChoppinessIndexStrategyConfig {
    pub threshold_trend: f64,
    pub threshold_sideways: f64,
}

impl ComponentDefault for ChoppinessIndexStrategyConfig {
    fn default(ctx: ComponentContext) -> Self {
        return ChoppinessIndexStrategyConfig {
            threshold_trend: CHOPPINESS_INDEX_STRATEGY_THRESHOLD_TREND,
            threshold_sideways: CHOPPINESS_INDEX_STRATEGY_THRESHOLD_SIDEWAYS,
        };
    }
}

pub struct ChoppinessIndexStrategy {
    pub config: ChoppinessIndexStrategyConfig,
    ctx: ComponentContext,
}

pub static CHOPPINESS_INDEX_STRATEGY_THRESHOLD_TREND: f64 = 38.2;
pub static CHOPPINESS_INDEX_STRATEGY_THRESHOLD_SIDEWAYS: f64 = 61.8;

impl ChoppinessIndexStrategy {
    pub fn new(ctx: ComponentContext, config: ChoppinessIndexStrategyConfig) -> Self {
        todo!("Not implemented");
        return ChoppinessIndexStrategy {
            ctx: ctx.clone(),
            config,
        };
    }

    pub fn next(&mut self, cmf: Option<f64>) -> Option<TradeDirection> {
        return None;
    }
}
