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

pub struct BollingerBandsPercentBStrategyConfig {
    pub threshold_oversold: f64,
    pub threshold_overbought: f64,
}

impl ComponentDefault for BollingerBandsPercentBStrategyConfig {
    fn default(ctx: ComponentContext) -> Self {
        return BollingerBandsPercentBStrategyConfig {
            threshold_overbought: BOLLINGER_BANDS_PB_STRATEGY_THRESHOLD_OVERBOUGHT,
            threshold_oversold: BOLLINGER_BANDS_PB_STRATEGY_THRESHOLD_OVERSOLD,
        };
    }
}

pub struct BollingerBandsPercentBStrategy {
    pub config: BollingerBandsPercentBStrategyConfig,
    ctx: ComponentContext,
    cross_over: CrossOverThresholdComponent,
    cross_under: CrossUnderThresholdComponent,
}

pub static BOLLINGER_BANDS_PB_STRATEGY_THRESHOLD_OVERSOLD: f64 = 0.0;
pub static BOLLINGER_BANDS_PB_STRATEGY_THRESHOLD_OVERBOUGHT: f64 = 1.0;

impl BollingerBandsPercentBStrategy {
    pub fn new(ctx: ComponentContext, config: BollingerBandsPercentBStrategyConfig) -> Self {
        return BollingerBandsPercentBStrategy {
            ctx: ctx.clone(),
            cross_over: CrossOverThresholdComponent::new(ctx.clone(), config.threshold_oversold),
            cross_under: CrossUnderThresholdComponent::new(
                ctx.clone(),
                config.threshold_overbought,
            ),
            config,
        };
    }

    pub fn next(&mut self, bbpb: Option<f64>) -> Option<TradeDirection> {
        self.ctx.on_next();

        let is_cross_over = self.cross_over.next(bbpb);
        let is_cross_under = self.cross_under.next(bbpb);

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
