use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::scale_value_min_max,
    strategy::action::{trade_direction_to_f64, TradeDirection},
    ta::rsi_component::{RelativeStrengthIndexComponentMetadata, RSI_MAX_VALUE, RSI_MIN_VALUE},
};

use super::{
    connors_relative_strength_index_indicator::{
        CONNORS_RELATIVE_STRENGTH_INDEX_MAX_VALUE, CONNORS_RELATIVE_STRENGTH_INDEX_MIN_VALUE,
    },
    connors_relative_strength_index_strategy::ConnorsRelativeStrengthIndexStrategyConfig,
    relative_strength_index_strategy::{
        RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
    },
};

pub struct ConnorsRelativeStrengthIndexFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for ConnorsRelativeStrengthIndexFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("raw"), self.raw),
            (String::from("main"), self.main),
            (String::from("trend"), self.trend),
        ]);
        return map;
    }
}

pub struct ConnorsRelativeStrengthIndexFeatureBuilder {
    ctx: ComponentContext,
}

impl ConnorsRelativeStrengthIndexFeatureBuilder {
    pub fn next(
        &mut self,
        crsi: Option<f64>,
        crsi_trade: Option<TradeDirection>,
        crsi_strategy_config: &ConnorsRelativeStrengthIndexStrategyConfig,
    ) -> ConnorsRelativeStrengthIndexFeature {
        self.ctx.assert();

        return ConnorsRelativeStrengthIndexFeature {
            raw: crsi,
            main: crsi.map(|value| {
                scale_value_min_max(
                    value,
                    CONNORS_RELATIVE_STRENGTH_INDEX_MIN_VALUE,
                    CONNORS_RELATIVE_STRENGTH_INDEX_MAX_VALUE,
                )
            }),
            trend: Some(trade_direction_to_f64(crsi_trade)),
        };
    }
}

impl ConnorsRelativeStrengthIndexFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return ConnorsRelativeStrengthIndexFeatureBuilder { ctx: ctx.clone() };
    }
}
