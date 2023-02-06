use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::scale_value_min_max,
    strategy::trade::{trade_direction_to_f64, TradeDirection},
    ta::rsi_component::{
        RelativeStrengthIndexComponentMetadata, RELATIVE_STRENGTH_INDEX_MAX_VALUE,
        RELATIVE_STRENGTH_INDEX_MIN_VALUE,
    },
};

use super::relative_strength_index_strategy::{
    RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
};

pub struct RelativeStrengthIndexFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for RelativeStrengthIndexFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("raw"), self.raw),
            (String::from("main"), self.main),
            (String::from("trend"), self.trend),
        ]);
        return map;
    }
}

pub struct RelativeStrengthIndexFeatureBuilder {
    ctx: ComponentContext,
}

impl RelativeStrengthIndexFeatureBuilder {
    pub fn next(
        &mut self,
        rsi: Option<f64>,
        rsi_metadata: &RelativeStrengthIndexComponentMetadata,
        rsi_trade: Option<TradeDirection>,
        rsi_strategy_config: &RelativeStrengthIndexStrategyConfig,
    ) -> RelativeStrengthIndexFeature {
        self.ctx.assert();

        let min = RELATIVE_STRENGTH_INDEX_MIN_VALUE;
        let max = RELATIVE_STRENGTH_INDEX_MAX_VALUE;

        return RelativeStrengthIndexFeature {
            raw: rsi,
            main: rsi.map(|value| {
                return scale_value_min_max(value, min, max);
            }),
            trend: Some(trade_direction_to_f64(rsi_trade)),
        };
    }
}

impl RelativeStrengthIndexFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return RelativeStrengthIndexFeatureBuilder { ctx: ctx.clone() };
    }
}
