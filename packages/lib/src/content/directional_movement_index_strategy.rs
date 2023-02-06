use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::trade::TradeDirection,
    ta::{
        cross::CrossMode, cross_component::CrossComponent,
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_threshold_component::CrossThresholdComponent,
        cross_under_component::CrossUnderComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
        rsi_component::RelativeStrengthIndexComponentMetadata,
    },
};

use super::{
    directional_movement_index_indicator::DirectionalMovementIndexIndicatorResult,
    relative_strength_index_indicator::RelativeStrengthIndexIndicator,
};

pub struct DirectionalMovementIndexStrategyConfig {
    pub threshold_strong_trend: f64,
    pub threshold_weak_trend: f64,
}

impl ComponentDefault for DirectionalMovementIndexStrategyConfig {
    fn default(ctx: ComponentContext) -> Self {
        return DirectionalMovementIndexStrategyConfig {
            threshold_strong_trend: DIRECTIONAL_MOVEMENT_INDEX_STRATEGY_THRESHOLD_STRONG_TREND,
            threshold_weak_trend: DIRECTIONAL_MOVEMENT_INDEX_STRATEGY_THRESHOLD_WEAK_TREND,
        };
    }
}

pub struct DirectionalMovementIndexStrategy {
    pub config: DirectionalMovementIndexStrategyConfig,
    ctx: ComponentContext,
    cross: CrossComponent,
}

pub static DIRECTIONAL_MOVEMENT_INDEX_STRATEGY_THRESHOLD_STRONG_TREND: f64 = 25.0;
pub static DIRECTIONAL_MOVEMENT_INDEX_STRATEGY_THRESHOLD_WEAK_TREND: f64 = 20.0;

impl DirectionalMovementIndexStrategy {
    pub fn new(ctx: ComponentContext, config: DirectionalMovementIndexStrategyConfig) -> Self {
        return DirectionalMovementIndexStrategy {
            ctx: ctx.clone(),
            cross: CrossComponent::new(ctx.clone()),
            config,
        };
    }

    pub fn next(
        &mut self,
        dmi: &DirectionalMovementIndexIndicatorResult,
    ) -> Option<TradeDirection> {
        self.ctx.assert();

        let is_strong_trend = dmi
            .adx
            .map(|x| x > self.config.threshold_strong_trend)
            .unwrap_or(false);

        let is_weak_trend = dmi
            .adx
            .map(|x| x < self.config.threshold_weak_trend)
            .unwrap_or(false);

        let plus_minus_cross = self.cross.next(dmi.plus, dmi.minus);

        let mut result: Option<TradeDirection> = None;

        if is_strong_trend {
            if let Some(plus_minus_cross) = plus_minus_cross {
                result = match plus_minus_cross {
                    CrossMode::Over => Some(TradeDirection::Long),
                    CrossMode::Under => Some(TradeDirection::Short),
                }
            }
        }

        return result;
    }
}
