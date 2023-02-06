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
    balance_of_power_strategy::BalanceOfPowerStrategyConfig,
};

pub struct BalanceOfPowerFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for BalanceOfPowerFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            ("raw".to_string(), self.raw),
            ("main".to_string(), self.main),
            ("trend".to_string(), self.trend),
        ]);
        return map;
    }
}

pub struct BalanceOfPowerFeatureBuilder {
    ctx: ComponentContext,
}

impl BalanceOfPowerFeatureBuilder {
    pub fn next(
        &mut self,
        bp: Option<f64>,
        bp_trade: Option<TradeDirection>,
        bp_strategy_config: &BalanceOfPowerStrategyConfig,
    ) -> BalanceOfPowerFeature {
        self.ctx.assert();

        return BalanceOfPowerFeature {
            raw: bp,
            main: bp.map(|v| clip_value(v, -1.0, 1.0)),
            trend: Some(trade_direction_to_f64(bp_trade)),
        };
    }
}

impl BalanceOfPowerFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return BalanceOfPowerFeatureBuilder { ctx: ctx.clone() };
    }
}
