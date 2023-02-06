use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::action::TradeDirection,
    ta::{
        cross::CrossMode, cross_component::CrossComponent,
        rsi_component::RelativeStrengthIndexComponentMetadata,
    },
};

use super::{
    aroon_indicator::AroonIndicatorResult,
    relative_strength_index_indicator::RelativeStrengthIndexIndicator,
};

pub struct BalanceOfPowerStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl ComponentDefault for BalanceOfPowerStrategyConfig {
    fn default(ctx: ComponentContext) -> Self {
        return BalanceOfPowerStrategyConfig {
            threshold_oversold: BALANCE_OF_POWER_STRATEGY_THRESHOLD_OVERSOLD,
            threshold_overbought: BALANCE_OF_POWER_STRATEGY_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct BalanceOfPowerStrategy {
    pub config: BalanceOfPowerStrategyConfig,
    ctx: ComponentContext,
    cross_over: CrossComponent,
    cross_under: CrossComponent,
}

pub static BALANCE_OF_POWER_STRATEGY_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static BALANCE_OF_POWER_STRATEGY_THRESHOLD_OVERBOUGHT: f64 = 0.0;

impl BalanceOfPowerStrategy {
    pub fn new(ctx: ComponentContext, config: BalanceOfPowerStrategyConfig) -> Self {
        return BalanceOfPowerStrategy {
            ctx: ctx.clone(),
            cross_over: CrossComponent::new(ctx.clone(), CrossMode::Over),
            cross_under: CrossComponent::new(ctx.clone(), CrossMode::Under),
            config,
        };
    }

    pub fn next(&mut self, ao: Option<f64>) -> Option<TradeDirection> {
        self.ctx.on_next();

        let is_cross_over = self
            .cross_over
            .next(ao, Some(self.config.threshold_oversold));

        let is_cross_under = self
            .cross_under
            .next(ao, Some(self.config.threshold_overbought));

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
