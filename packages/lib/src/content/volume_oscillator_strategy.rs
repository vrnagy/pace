use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::action::TradeDirection,
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
    relative_vigor_index_indicator::RelativeVigorIndexIndicatorResult,
};

pub struct VolumeOscillatorStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl ComponentDefault for VolumeOscillatorStrategyConfig {
    fn default(ctx: ComponentContext) -> Self {
        return Self {
            threshold_oversold: VOLUME_OSCILLATOR_STRATEGY_THRESHOLD_OVERSOLD,
            threshold_overbought: VOLUME_OSCILLATOR_STRATEGY_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct VolumeOscillatorStrategy {
    ctx: ComponentContext,
    cross_over: CrossOverThresholdComponent,
    cross_under: CrossUnderThresholdComponent,
}

pub static VOLUME_OSCILLATOR_STRATEGY_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static VOLUME_OSCILLATOR_STRATEGY_THRESHOLD_OVERBOUGHT: f64 = 0.0;

impl VolumeOscillatorStrategy {
    pub fn new(ctx: ComponentContext, config: VolumeOscillatorStrategyConfig) -> Self {
        return VolumeOscillatorStrategy {
            ctx: ctx.clone(),
            cross_over: CrossOverThresholdComponent::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThresholdComponent::new(
                ctx.clone(),
                config.threshold_overbought,
            ),
        };
    }

    pub fn next(&mut self, vo: Option<f64>) -> Option<TradeDirection> {
        self.ctx.assert();

        let is_cross_over = self.cross_over.next(vo);
        let is_cross_under = self.cross_under.next(vo);

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
