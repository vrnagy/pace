use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::{clip_value, scale_value_min_max},
    strategy::trade::{trade_direction_to_f64, TradeDirection},
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
    williams_percent_range_indicator::{
        WILLIAMS_PERCENT_RANGE_MAX_VALUE, WILLIAMS_PERCENT_RANGE_MIN_VALUE,
    },
};

pub struct WilliamsPercentRangeFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for WilliamsPercentRangeFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("raw"), self.raw),
            (String::from("main"), self.main),
            (String::from("trend"), self.trend),
        ]);
        return map;
    }
}

pub struct WilliamsPercentRangeFeatureBuilder {
    ctx: ComponentContext,
}

impl WilliamsPercentRangeFeatureBuilder {
    pub fn next(
        &mut self,
        wpr: Option<f64>,
        wpr_trade: Option<TradeDirection>,
    ) -> WilliamsPercentRangeFeature {
        self.ctx.assert();

        let min = WILLIAMS_PERCENT_RANGE_MIN_VALUE;
        let max = WILLIAMS_PERCENT_RANGE_MAX_VALUE;

        return WilliamsPercentRangeFeature {
            raw: wpr,
            main: wpr.map(|value| {
                return clip_value(scale_value_min_max(value, min, max), -1.0, 1.0);
            }),
            trend: Some(trade_direction_to_f64(wpr_trade)),
        };
    }
}

impl WilliamsPercentRangeFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return WilliamsPercentRangeFeatureBuilder { ctx: ctx.clone() };
    }
}
