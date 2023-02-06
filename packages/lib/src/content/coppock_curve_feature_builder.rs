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
    coppock_curve_strategy::CoppockCurveStrategyConfig,
};

pub struct CoppockCurveFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for CoppockCurveFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            ("raw".to_string(), self.raw),
            ("main".to_string(), self.main),
            ("trend".to_string(), self.trend),
        ]);
        return map;
    }
}

pub struct CoppockCurveFeatureBuilder {
    ctx: ComponentContext,
}

impl CoppockCurveFeatureBuilder {
    pub fn next(
        &mut self,
        cc: Option<f64>,
        cc_trade: Option<TradeDirection>,
        cc_strategy_config: &CoppockCurveStrategyConfig,
    ) -> CoppockCurveFeature {
        self.ctx.assert();

        return CoppockCurveFeature {
            raw: cc,
            main: cc.map(|v| clip_value(v / 100.0, -1.0, 1.0)),
            trend: Some(trade_direction_to_f64(cc_trade)),
        };
    }
}

impl CoppockCurveFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return CoppockCurveFeatureBuilder { ctx: ctx.clone() };
    }
}
