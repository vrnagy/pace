use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::{clip_value, scale_value_min_max},
    strategy::trade::{trade_direction_to_f64, TradeDirection},
};

use super::{
    aroon_indicator::{AroonIndicatorResult, AROON_MAX_VALUE, AROON_MIN_VALUE},
    aroon_strategy::AroonStrategyMetadata,
    awesome_oscillator_strategy::AwesomeOscillatorStrategyConfig,
    chaikin_money_flow_strategy::ChaikinMoneyFlowStrategyConfig,
    chande_momentum_oscillator_indicator::CHANDE_MOMENTUM_OSCILLATOR_MAX_VALUE,
    chande_momentum_oscillator_strategy::ChandeMomentumOscillatorStrategyConfig,
};

pub struct ChandeMomentumOscillatorFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for ChandeMomentumOscillatorFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            ("raw".to_string(), self.raw),
            ("main".to_string(), self.main),
            ("trend".to_string(), self.trend),
        ]);
        return map;
    }
}

pub struct ChandeMomentumOscillatorFeatureBuilder {
    ctx: ComponentContext,
}

impl ChandeMomentumOscillatorFeatureBuilder {
    pub fn next(
        &mut self,
        cmo: Option<f64>,
        cmo_trade: Option<TradeDirection>,
        cmo_strategy_config: &ChandeMomentumOscillatorStrategyConfig,
    ) -> ChandeMomentumOscillatorFeature {
        self.ctx.assert();

        return ChandeMomentumOscillatorFeature {
            raw: cmo,
            main: cmo.map(|v| clip_value(v / CHANDE_MOMENTUM_OSCILLATOR_MAX_VALUE, -1.0, 1.0)),
            trend: Some(trade_direction_to_f64(cmo_trade)),
        };
    }
}

impl ChandeMomentumOscillatorFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return ChandeMomentumOscillatorFeatureBuilder { ctx: ctx.clone() };
    }
}
