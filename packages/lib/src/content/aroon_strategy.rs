use crate::base::{
    components::{component_context::ComponentContext, component_default::ComponentDefault},
    strategy::action::TradeDirection,
    ta::{
        cross::CrossMode, cross_component::CrossComponent,
        rsi_component::RelativeStrengthIndexComponentMetadata,
    },
};

use super::{
    aroon_indicator::AroonIndicatorResult,
    relative_strength_index_indicator::RelativeStrengthIndexIndicator,
};

pub struct AroonStrategyMetadata {
    pub up_trend_strength: f64,
    pub down_trend_strength: f64,
    pub cross_mode: bool,
}

pub struct AroonStrategy {
    ctx: ComponentContext,
    cross_over: CrossComponent,
    cross_under: CrossComponent,
    up_trend_confirmation: bool,
    down_trend_confirmation: bool,
    metadata: AroonStrategyMetadata,
}

impl AroonStrategy {
    pub fn new(ctx: ComponentContext) -> Self {
        return AroonStrategy {
            ctx: ctx.clone(),
            cross_over: CrossComponent::new(ctx.clone(), CrossMode::Over),
            cross_under: CrossComponent::new(ctx.clone(), CrossMode::Under),
            up_trend_confirmation: false,
            down_trend_confirmation: false,
            metadata: AroonStrategyMetadata {
                up_trend_strength: 0.0,
                down_trend_strength: 0.0,
                cross_mode: false,
            },
        };
    }

    pub fn metadata(&self) -> &AroonStrategyMetadata {
        return &self.metadata;
    }

    pub fn next(&mut self, aroon: &AroonIndicatorResult) -> Option<TradeDirection> {
        self.ctx.on_next();

        self.metadata.up_trend_strength = match (aroon.up, aroon.down) {
            (Some(up), Some(down)) => {
                if up > 50.0 && down < 50.0 {
                    1.0 - (100.0 - up) / 50.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        };

        self.metadata.down_trend_strength = match (aroon.up, aroon.down) {
            (Some(up), Some(down)) => {
                if down > 50.0 && up < 50.0 {
                    1.0 - (100.0 - down) / 50.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        };

        let is_cross_over = self.cross_over.next(aroon.down, aroon.up);
        let is_cross_under = self.cross_under.next(aroon.down, aroon.up);

        if is_cross_over || is_cross_under {
            self.metadata.cross_mode = true;
        }

        let mut up_trend_confirmation = false;
        let mut down_trend_confirmation = false;

        if self.metadata.cross_mode {
            if self.metadata.up_trend_strength >= 1.0 {
                up_trend_confirmation = true;
                self.metadata.cross_mode = false;
            } else if self.metadata.down_trend_strength >= 1.0 {
                down_trend_confirmation = true;
                self.metadata.cross_mode = false;
            }
        }

        let result = if up_trend_confirmation {
            Some(TradeDirection::Long)
        } else if down_trend_confirmation {
            Some(TradeDirection::Short)
        } else {
            None
        };

        return result;
    }
}
