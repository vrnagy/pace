use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::action::TradeDirection,
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

pub struct ChandeMomentumOscillatorStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl ComponentDefault for ChandeMomentumOscillatorStrategyConfig {
    fn default(ctx: ComponentContext) -> Self {
        return ChandeMomentumOscillatorStrategyConfig {
            threshold_oversold: CHANDE_MOMENTUM_OSCILLATOR_STRATEGY_THRESHOLD_OVERSOLD,
            threshold_overbought: CHANDE_MOMENTUM_OSCILLATOR_STRATEGY_THRESHOLD_OVERBOUGHT,
        };
    }
}

pub struct ChandeMomentumOscillatorStrategy {
    pub config: ChandeMomentumOscillatorStrategyConfig,
    ctx: ComponentContext,
    cross_over: CrossOverThresholdComponent,
    cross_under: CrossUnderThresholdComponent,
}

pub static CHANDE_MOMENTUM_OSCILLATOR_STRATEGY_THRESHOLD_OVERSOLD: f64 = -50.0;
pub static CHANDE_MOMENTUM_OSCILLATOR_STRATEGY_THRESHOLD_OVERBOUGHT: f64 = 50.0;

impl ChandeMomentumOscillatorStrategy {
    pub fn new(ctx: ComponentContext, config: ChandeMomentumOscillatorStrategyConfig) -> Self {
        return ChandeMomentumOscillatorStrategy {
            ctx: ctx.clone(),
            cross_over: CrossOverThresholdComponent::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThresholdComponent::new(
                ctx.clone(),
                config.threshold_overbought,
            ),
            config,
        };
    }

    pub fn next(&mut self, cmf: Option<f64>) -> Option<TradeDirection> {
        self.ctx.on_next();

        let is_cross_over = self.cross_over.next(cmf);
        let is_cross_under = self.cross_under.next(cmf);

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
