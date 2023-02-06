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
    price_oscillator_strategy::PriceOscillatorStrategyConfig,
};

pub struct PriceOscillatorFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for PriceOscillatorFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            ("raw".to_string(), self.raw),
            ("main".to_string(), self.main),
            ("trend".to_string(), self.trend),
        ]);
        return map;
    }
}

pub struct PriceOscillatorFeatureBuilder {
    ctx: ComponentContext,
}

impl PriceOscillatorFeatureBuilder {
    pub fn next(
        &mut self,
        po: Option<f64>,
        po_trade: Option<TradeDirection>,
        po_strategy_config: &PriceOscillatorStrategyConfig,
    ) -> PriceOscillatorFeature {
        self.ctx.assert();

        let divisor = 100.0;

        return PriceOscillatorFeature {
            raw: po,
            main: po.map(|v| clip_value(v / divisor, -1.0, 1.0)),
            trend: Some(trade_direction_to_f64(po_trade)),
        };
    }
}

impl PriceOscillatorFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return PriceOscillatorFeatureBuilder { ctx: ctx.clone() };
    }
}
