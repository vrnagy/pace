use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::scale_value_min_max,
    strategy::action::{trade_direction_to_f64, TradeDirection},
};

use super::{
    relative_strength_index_strategy::{
        RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
    },
    relative_volatility_index_indicator::{
        RELATIVE_VOLATILITY_INDEX_MAX_VALUE, RELATIVE_VOLATILITY_INDEX_MIN_VALUE,
    },
    relative_volatility_index_strategy::RelativeVolatilityIndexStrategyConfig,
};

pub struct RelativeVolatilityIndexFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for RelativeVolatilityIndexFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("raw"), self.raw),
            (String::from("main"), self.main),
            (String::from("trend"), self.trend),
        ]);
        return map;
    }
}

pub struct RelativeVolatilityIndexFeatureBuilder {
    ctx: ComponentContext,
}

impl RelativeVolatilityIndexFeatureBuilder {
    pub fn next(
        &mut self,
        rvi: Option<f64>,
        rvi_trade: Option<TradeDirection>,
        rvi_strategy_config: &RelativeVolatilityIndexStrategyConfig,
    ) -> RelativeVolatilityIndexFeature {
        self.ctx.assert();

        let min = RELATIVE_VOLATILITY_INDEX_MIN_VALUE;
        let max = RELATIVE_VOLATILITY_INDEX_MAX_VALUE;

        return RelativeVolatilityIndexFeature {
            raw: rvi,
            main: rvi.map(|value| {
                return scale_value_min_max(value, min, max);
            }),
            trend: Some(trade_direction_to_f64(rvi_trade)),
        };
    }
}

impl RelativeVolatilityIndexFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return RelativeVolatilityIndexFeatureBuilder { ctx: ctx.clone() };
    }
}
