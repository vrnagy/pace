use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::{clip_value, scale_value_min_max},
};

use super::{
    aroon_indicator::{AroonIndicatorResult, AROON_MAX_VALUE, AROON_MIN_VALUE},
    aroon_strategy::AroonStrategyMetadata,
    awesome_oscillator_strategy::AwesomeOscillatorStrategyConfig,
    balance_of_power_strategy::BalanceOfPowerStrategyConfig,
    bollinger_bands_pb_strategy::BollingerBandsPercentBStrategyConfig,
};

pub struct BollingerBandsWidthFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
}

impl Feature for BollingerBandsWidthFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            ("raw".to_string(), self.raw),
            ("main".to_string(), self.main),
        ]);
        return map;
    }
}

pub struct BollingerBandsWidthFeatureBuilder {
    ctx: ComponentContext,
}

impl BollingerBandsWidthFeatureBuilder {
    pub fn next(&mut self, bbw: Option<f64>) -> BollingerBandsWidthFeature {
        self.ctx.assert();

        return BollingerBandsWidthFeature {
            raw: bbw,
            main: bbw.map(|v| clip_value(scale_value_min_max(v, 0.0, 1.0), -1.0, 1.0)),
        };
    }
}

impl BollingerBandsWidthFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return BollingerBandsWidthFeatureBuilder { ctx: ctx.clone() };
    }
}
