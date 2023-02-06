use std::collections::HashMap;

use crate::base::{
    components::component_context::ComponentContext,
    features::{feature::Feature, feature_regions::FeatureTernaryTrendRegions},
    statistics::{clip_value, scale_value_min_max},
    strategy::trade::{trade_direction_to_f64, TradeDirection},
};

use super::{
    connors_relative_strength_index_indicator::{
        CONNORS_RELATIVE_STRENGTH_INDEX_MAX_VALUE, CONNORS_RELATIVE_STRENGTH_INDEX_MIN_VALUE,
    },
    connors_relative_strength_index_strategy::ConnorsRelativeStrengthIndexStrategyConfig,
    directional_movement_index_indicator::{
        DirectionalMovementIndexIndicatorResult, DIRECTIONAL_MOVEMENT_INDEX_MAX_VALUE,
        DIRECTIONAL_MOVEMENT_INDEX_MIN_VALUE,
    },
    directional_movement_index_strategy::DirectionalMovementIndexStrategyConfig,
    relative_strength_index_strategy::{
        RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
    },
    relative_vigor_index_indicator::{
        RelativeVigorIndexIndicatorResult, RELATIVE_VIGOR_INDEX_MAX_VALUE,
        RELATIVE_VIGOR_INDEX_MIN_VALUE,
    },
    volume_oscillator_indicator::{VOLUME_OSCILLATOR_MAX_VALUE, VOLUME_OSCILLATOR_MIN_VALUE},
};

pub struct VolumeOscillatorFeature {
    pub raw: Option<f64>,
    pub main: Option<f64>,
    pub trend: Option<f64>,
}

impl Feature for VolumeOscillatorFeature {
    fn flatten(&self) -> HashMap<String, Option<f64>> {
        let mut map = HashMap::from([
            (String::from("raw"), self.raw),
            (String::from("main"), self.main),
            (String::from("trend"), self.trend),
        ]);
        return map;
    }
}

pub struct VolumeOscillatorFeatureBuilder {
    ctx: ComponentContext,
}

impl VolumeOscillatorFeatureBuilder {
    pub fn next(
        &mut self,
        vo: Option<f64>,
        vo_trade: Option<TradeDirection>,
    ) -> VolumeOscillatorFeature {
        self.ctx.assert();

        let min = VOLUME_OSCILLATOR_MIN_VALUE;
        let max = VOLUME_OSCILLATOR_MAX_VALUE;

        return VolumeOscillatorFeature {
            raw: vo,
            main: vo.map(|v| scale_value_min_max(v, min, max)),
            trend: Some(trade_direction_to_f64(vo_trade)),
        };
    }
}

impl VolumeOscillatorFeatureBuilder {
    pub fn new(ctx: ComponentContext) -> Self {
        return VolumeOscillatorFeatureBuilder { ctx: ctx.clone() };
    }
}
