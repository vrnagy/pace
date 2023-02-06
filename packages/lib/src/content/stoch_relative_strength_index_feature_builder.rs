use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::scale_value_min_max,
    strategy::action::{trade_direction_to_f64, TradeDirection},
    ta::rsi_component::{
        RelativeStrengthIndexComponentMetadata, RELATIVE_STRENGTH_INDEX_MAX_VALUE,
        RELATIVE_STRENGTH_INDEX_MIN_VALUE,
    },
};

use super::{
    relative_strength_index_strategy::{
        RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
    },
    stoch_relative_strength_index_indicator::{
        StochRelativeStrengthIndexIndicatorResult, STOCH_RELATIVE_STRENGTH_INDEX_MAX_VALUE,
        STOCH_RELATIVE_STRENGTH_INDEX_MIN_VALUE,
    },
    stoch_relative_strength_index_strategy::StochRelativeStrengthIndexStrategyConfig,
};

pub struct StochRelativeStrengthIndexFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for StochRelativeStrengthIndexFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("raw"), self.raw),
            (String::from("main"), self.main),
            (String::from("trend"), self.trend),
        ]);
        return map;
    }
}

pub struct StochRelativeStrengthIndexFeatureBuilder {
    ctx: ComponentContext,
}

impl StochRelativeStrengthIndexFeatureBuilder {
    pub fn next(
        &mut self,
        stoch_rsi: &StochRelativeStrengthIndexIndicatorResult,
        stoch_rsi_trade: Option<TradeDirection>,
        stoch_rsi_strategy_config: &StochRelativeStrengthIndexStrategyConfig,
    ) -> StochRelativeStrengthIndexFeature {
        self.ctx.assert();

        let min = STOCH_RELATIVE_STRENGTH_INDEX_MIN_VALUE;
        let max = STOCH_RELATIVE_STRENGTH_INDEX_MAX_VALUE;

        return StochRelativeStrengthIndexFeature {
            raw: stoch_rsi.k,
            main: stoch_rsi.k.map(|value| {
                return scale_value_min_max(value, min, max);
            }),
            trend: Some(trade_direction_to_f64(stoch_rsi_trade)),
        };
    }
}

impl StochRelativeStrengthIndexFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return StochRelativeStrengthIndexFeatureBuilder { ctx: ctx.clone() };
    }
}
