use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::trade::TradeDirection,
    ta::{
        cross::CrossMode, cross_component::CrossComponent,
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_threshold_component::CrossThresholdComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
        rsi_component::RelativeStrengthIndexComponentMetadata,
    },
};

use super::{
    relative_strength_index_indicator::RelativeStrengthIndexIndicator,
    relative_strength_index_strategy::RelativeStrengthIndexStrategyConfig,
    stoch_relative_strength_index_indicator::StochRelativeStrengthIndexIndicatorResult,
};

pub struct WilliamsPercentRangeStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl ComponentDefault for WilliamsPercentRangeStrategyConfig {
    fn default(ctx: ComponentContext) -> Self {
        return Self {
            threshold_oversold: WILLIAMS_PERCENT_RANGE_STRATEGY_THRESHOLD_OVERSOLD,
            threshold_overbought: WILLIAMS_PERCENT_RANGE_STRATEGY_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct WilliamsPercentRangeStrategy {
    pub config: WilliamsPercentRangeStrategyConfig,
    ctx: ComponentContext,
    cross_overbought: CrossOverThresholdComponent,
    cross_oversold: CrossUnderThresholdComponent,
}

pub static WILLIAMS_PERCENT_RANGE_STRATEGY_THRESHOLD_OVERSOLD: f64 = -80.0;
pub static WILLIAMS_PERCENT_RANGE_STRATEGY_THRESHOLD_OVERBOUGHT: f64 = -20.0;

impl WilliamsPercentRangeStrategy {
    pub fn new(ctx: ComponentContext, config: WilliamsPercentRangeStrategyConfig) -> Self {
        return WilliamsPercentRangeStrategy {
            ctx: ctx.clone(),
            cross_overbought: CrossOverThresholdComponent::new(
                ctx.clone(),
                config.threshold_oversold,
            ),
            cross_oversold: CrossUnderThresholdComponent::new(
                ctx.clone(),
                config.threshold_overbought,
            ),
            config,
        };
    }

    pub fn next(&mut self, wpr: Option<f64>) -> Option<TradeDirection> {
        self.ctx.assert();

        let is_cross_over = self.cross_overbought.next(wpr);
        let is_cross_under = self.cross_oversold.next(wpr);

        let result = if is_cross_over {
            Some(TradeDirection::Long)
        } else if is_cross_under {
            Some(TradeDirection::Short)
        } else {
            None
        };

        return result;
    }
}
