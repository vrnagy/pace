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
    price_oscillator_strategy::PriceOscillatorStrategyConfig,
    ultimate_oscillator_indicator::{ULTIMATE_OSCILLATOR_MAX_VALUE, ULTIMATE_OSCILLATOR_MIN_VALUE},
};

pub struct UltimateOscillatorFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
}

impl Feature for UltimateOscillatorFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            ("raw".to_string(), self.raw),
            ("main".to_string(), self.main),
        ]);
        return map;
    }
}

pub struct UltimateOscillatorFeatureBuilder {
    ctx: ComponentContext,
}

impl UltimateOscillatorFeatureBuilder {
    pub fn next(&mut self, uo: Option<f64>) -> UltimateOscillatorFeature {
        self.ctx.assert();

        let min = ULTIMATE_OSCILLATOR_MIN_VALUE;
        let max = ULTIMATE_OSCILLATOR_MAX_VALUE;

        return UltimateOscillatorFeature {
            raw: uo,
            main: uo.map(|v| clip_value(scale_value_min_max(v, min, max), -1.0, 1.0)),
        };
    }
}

impl UltimateOscillatorFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return UltimateOscillatorFeatureBuilder { ctx: ctx.clone() };
    }
}
