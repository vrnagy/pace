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
    commodity_channel_index_strategy::CommodityChannelIndexStrategyConfig,
};

pub struct CommodityChannelIndexFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for CommodityChannelIndexFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            ("raw".to_string(), self.raw),
            ("main".to_string(), self.main),
            ("trend".to_string(), self.trend),
        ]);
        return map;
    }
}

pub struct CommodityChannelIndexFeatureBuilder {
    ctx: ComponentContext,
}

impl CommodityChannelIndexFeatureBuilder {
    pub fn next(
        &mut self,
        cci: Option<f64>,
        cci_trade: Option<TradeDirection>,
        cci_strategy_config: &CommodityChannelIndexStrategyConfig,
    ) -> CommodityChannelIndexFeature {
        self.ctx.assert();

        return CommodityChannelIndexFeature {
            raw: cci,
            main: cci.map(|v| clip_value(scale_value_min_max(v, -250.0, 250.0), -1.0, 1.0)),
            trend: Some(trade_direction_to_f64(cci_trade)),
        };
    }
}

impl CommodityChannelIndexFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return CommodityChannelIndexFeatureBuilder { ctx: ctx.clone() };
    }
}
