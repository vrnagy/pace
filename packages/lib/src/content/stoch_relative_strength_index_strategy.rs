use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::action::TradeDirection,
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

pub struct StochRelativeStrengthIndexStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl ComponentDefault for StochRelativeStrengthIndexStrategyConfig {
    fn default(ctx: ComponentContext) -> Self {
        return Self {
            threshold_oversold: STOCH_RELATIVE_STRENGTH_INDEX_STRATEGY_THRESHOLD_OVERSOLD,
            threshold_overbought: STOCH_RELATIVE_STRENGTH_INDEX_STRATEGY_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct StochRelativeStrengthIndexStrategy {
    pub config: StochRelativeStrengthIndexStrategyConfig,
    ctx: ComponentContext,
    cross_overbought: CrossOverThresholdComponent,
    cross_oversold: CrossUnderThresholdComponent,
}

pub static STOCH_RELATIVE_STRENGTH_INDEX_STRATEGY_THRESHOLD_OVERSOLD: f64 = 20.0;
pub static STOCH_RELATIVE_STRENGTH_INDEX_STRATEGY_THRESHOLD_OVERBOUGHT: f64 = 80.0;

impl StochRelativeStrengthIndexStrategy {
    pub fn new(ctx: ComponentContext, config: StochRelativeStrengthIndexStrategyConfig) -> Self {
        return StochRelativeStrengthIndexStrategy {
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

    pub fn next(
        &mut self,
        stoch_rsi: &StochRelativeStrengthIndexIndicatorResult,
    ) -> Option<TradeDirection> {
        self.ctx.assert();

        let is_cross_over = self.cross_overbought.next(stoch_rsi.k);
        let is_cross_under = self.cross_oversold.next(stoch_rsi.k);

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
