use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::{clip_value, scale_value_min_max},
    strategy::action::{trade_direction_to_f64, TradeDirection},
    ta::rsi_component::{RelativeStrengthIndexComponentMetadata, RSI_MAX_VALUE, RSI_MIN_VALUE},
};

use super::{
    aroon_indicator::{AroonIndicatorResult, AROON_MAX_VALUE, AROON_MIN_VALUE},
    aroon_strategy::AroonStrategyMetadata,
    awesome_oscillator_strategy::AwesomeOscillatorStrategyConfig,
    chaikin_money_flow_strategy::ChaikinMoneyFlowStrategyConfig,
};

pub struct ChaikinMoneyFlowFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for ChaikinMoneyFlowFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            ("raw".to_string(), self.raw),
            ("main".to_string(), self.main),
            ("trend".to_string(), self.trend),
        ]);
        return map;
    }
}

pub struct ChaikinMoneyFlowFeatureBuilder {
    ctx: ComponentContext,
}

impl ChaikinMoneyFlowFeatureBuilder {
    pub fn next(
        &mut self,
        cmf: Option<f64>,
        cmf_trade: Option<TradeDirection>,
        cmf_strategy_config: &ChaikinMoneyFlowStrategyConfig,
    ) -> ChaikinMoneyFlowFeature {
        self.ctx.assert();

        return ChaikinMoneyFlowFeature {
            raw: cmf,
            main: cmf.map(|v| clip_value(v, -1.0, 1.0)),
            trend: Some(trade_direction_to_f64(cmf_trade)),
        };
    }
}

impl ChaikinMoneyFlowFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return ChaikinMoneyFlowFeatureBuilder { ctx: ctx.clone() };
    }
}
