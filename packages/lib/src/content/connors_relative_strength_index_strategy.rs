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

use super::relative_strength_index_indicator::RelativeStrengthIndexIndicator;

pub struct ConnorsRelativeStrengthIndexStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl ComponentDefault for ConnorsRelativeStrengthIndexStrategyConfig {
    fn default(ctx: ComponentContext) -> Self {
        return ConnorsRelativeStrengthIndexStrategyConfig {
            threshold_oversold: CONNORS_RSI_STRATEGY_THRESHOLD_OVERSOLD,
            threshold_overbought: CONNORS_RSI_STRATEGY_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct ConnorsRelativeStrengthIndexStrategy {
    pub config: ConnorsRelativeStrengthIndexStrategyConfig,
    ctx: ComponentContext,
    cross_overbought: CrossOverThresholdComponent,
    cross_oversold: CrossUnderThresholdComponent,
}

pub static CONNORS_RSI_STRATEGY_THRESHOLD_OVERSOLD: f64 = 20.0;
pub static CONNORS_RSI_STRATEGY_THRESHOLD_OVERBOUGHT: f64 = 80.0;

impl ConnorsRelativeStrengthIndexStrategy {
    pub fn new(ctx: ComponentContext, config: ConnorsRelativeStrengthIndexStrategyConfig) -> Self {
        return ConnorsRelativeStrengthIndexStrategy {
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

    pub fn next(&mut self, rsi: Option<f64>) -> Option<TradeDirection> {
        self.ctx.assert();

        let is_cross_over = self.cross_overbought.next(rsi);
        let is_cross_under = self.cross_oversold.next(rsi);

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
