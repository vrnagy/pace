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
    balance_of_power_strategy::BalanceOfPowerStrategyConfig,
    bollinger_bands_pb_strategy::BollingerBandsPercentBStrategyConfig,
};

pub struct BollingerBandsPercentBFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for BollingerBandsPercentBFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            ("raw".to_string(), self.raw),
            ("main".to_string(), self.main),
            ("trend".to_string(), self.trend),
        ]);
        return map;
    }
}

pub struct BollingerBandsPercentBFeatureBuilder {
    ctx: ComponentContext,
}

impl BollingerBandsPercentBFeatureBuilder {
    pub fn next(
        &mut self,
        bbpb: Option<f64>,
        bbpb_trade: Option<TradeDirection>,
        bbpb_strategy_config: &BollingerBandsPercentBStrategyConfig,
    ) -> BollingerBandsPercentBFeature {
        self.ctx.assert();

        return BollingerBandsPercentBFeature {
            raw: bbpb,
            main: bbpb.map(|v| clip_value(scale_value_min_max(v, -0.5, 1.5), -1.0, 1.0)),
            trend: Some(trade_direction_to_f64(bbpb_trade)),
        };
    }
}

impl BollingerBandsPercentBFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return BollingerBandsPercentBFeatureBuilder { ctx: ctx.clone() };
    }
}
