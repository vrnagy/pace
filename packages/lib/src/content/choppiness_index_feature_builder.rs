use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::{clip_value, scale_value_min_max},
    strategy::action::{trade_direction_to_f64, TradeDirection},
};

use super::{
    aroon_indicator::{AroonIndicatorResult, AROON_MAX_VALUE, AROON_MIN_VALUE},
    aroon_strategy::AroonStrategyMetadata,
    awesome_oscillator_strategy::AwesomeOscillatorStrategyConfig,
    chaikin_money_flow_strategy::ChaikinMoneyFlowStrategyConfig,
    chande_momentum_oscillator_indicator::CHANDE_MOMENTUM_OSCILLATOR_MAX_VALUE,
    chande_momentum_oscillator_strategy::ChandeMomentumOscillatorStrategyConfig,
    choppiness_index_strategy::ChoppinessIndexStrategyConfig,
};

pub struct ChoppinessIndexFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
}

impl Feature for ChoppinessIndexFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            ("raw".to_string(), self.raw),
            ("main".to_string(), self.main),
        ]);
        return map;
    }
}

pub struct ChoppinessIndexFeatureBuilder {
    ctx: ComponentContext,
}

impl ChoppinessIndexFeatureBuilder {
    pub fn next(&mut self, chop: Option<f64>) -> ChoppinessIndexFeature {
        self.ctx.assert();

        return ChoppinessIndexFeature {
            raw: chop,
            main: chop.map(|v| clip_value(scale_value_min_max(v, 0.0, 100.0), -1.0, 1.0)),
        };
    }
}

impl ChoppinessIndexFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return ChoppinessIndexFeatureBuilder { ctx: ctx.clone() };
    }
}
