use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::trade::TradeDirection,
    ta::{
        cross::CrossMode, cross_component::CrossComponent,
        cross_over_threshold_component::CrossOverThresholdComponent,
        cross_threshold_component::CrossThresholdComponent,
        cross_under_component::CrossUnderComponent,
        cross_under_threshold_component::CrossUnderThresholdComponent,
        rsi_component::RelativeStrengthIndexComponentMetadata,
    },
};

use super::{
    directional_movement_index_indicator::DirectionalMovementIndexIndicatorResult,
    relative_strength_index_indicator::RelativeStrengthIndexIndicator,
    relative_vigor_index_indicator::RelativeVigorIndexIndicatorResult,
};

pub struct RelativeVigorIndexStrategy {
    ctx: ComponentContext,
    cross: CrossComponent,
}

impl RelativeVigorIndexStrategy {
    pub fn new(ctx: ComponentContext) -> Self {
        return RelativeVigorIndexStrategy {
            ctx: ctx.clone(),
            cross: CrossComponent::new(ctx.clone()),
        };
    }

    pub fn next(&mut self, rvgi: &RelativeVigorIndexIndicatorResult) -> Option<TradeDirection> {
        self.ctx.assert();

        let rvi_s_cross = self.cross.next(rvgi.rvi, rvgi.sig);

        let mut result: Option<TradeDirection> = None;

        if let Some(plus_minus_cross) = rvi_s_cross {
            result = match plus_minus_cross {
                CrossMode::Over => Some(TradeDirection::Long),
                CrossMode::Under => Some(TradeDirection::Short),
            }
        }

        return result;
    }
}
