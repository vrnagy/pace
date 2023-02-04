use crate::{
    components::{
        change::{recursive_prank::RecursivePercentRank, recursive_roc::RecursiveRateOfChange},
        component_context::ComponentContext,
        lifo::recursive_lifo::RecursiveLIFO,
        source::Source,
        stoch::recursive_stoch::RecursiveStoch,
    },
    pinescript::utils::ps_nz,
    ta::moving_average::sma_component::SimpleMovingAverageComponent,
};

use super::rsi_component::{RelativeStrengthIndexComponent, RelativeStrengthIndexComponentResult};

pub struct ConnorsRelativeStrengthIndexIndicatorConfig {
    pub length_rsi: usize,
    pub length_up_down: usize,
    pub length_roc: usize,
    pub src: Source,
}

pub struct ConnorsRelativeStrengthIndexIndicator {
    config: ConnorsRelativeStrengthIndexIndicatorConfig,
    ctx: ComponentContext,
    prev_src: Option<f64>,
    prev_ud: Option<f64>,
    rsi: RelativeStrengthIndexComponent,
    up_down_rsi: RelativeStrengthIndexComponent,
    percent_rank: RecursivePercentRank,
    roc: RecursiveRateOfChange,
}

pub struct ConnorsRelativeStrengthIndexIndicatorResult {
    pub value: Option<f64>,
}

impl ConnorsRelativeStrengthIndexIndicator {
    pub fn new(ctx: ComponentContext, config: ConnorsRelativeStrengthIndexIndicatorConfig) -> Self {
        return ConnorsRelativeStrengthIndexIndicator {
            ctx: ctx.clone(),
            rsi: RelativeStrengthIndexComponent::new(ctx.clone(), config.length_rsi),
            up_down_rsi: RelativeStrengthIndexComponent::new(ctx.clone(), config.length_up_down),
            percent_rank: RecursivePercentRank::new(ctx.clone(), config.length_roc),
            roc: RecursiveRateOfChange::new(ctx.clone(), 1),
            config,
            prev_src: None,
            prev_ud: None,
        };
    }

    fn compute_up_down(
        src: Option<f64>,
        prev_src: Option<f64>,
        prev_ud: Option<f64>,
    ) -> Option<f64> {
        if prev_src == src {
            return Some(0.0);
        }
        let prev_ud = ps_nz(prev_ud);
        if !src.is_none() && !prev_src.is_none() && src.unwrap() > prev_src.unwrap() {
            if prev_ud <= 0.0 {
                return Some(1.0);
            } else {
                return Some(prev_ud + 1.0);
            }
        } else {
            if prev_ud >= 0.0 {
                return Some(-1.0);
            } else {
                return Some(prev_ud - 1.0);
            }
        }
    }

    pub fn next(&mut self) -> ConnorsRelativeStrengthIndexIndicatorResult {
        self.ctx.assert();
        let src = self.config.src.get();

        let rsi = self.rsi.next(src);

        let up_down = Self::compute_up_down(src, self.prev_src, self.prev_ud);
        let up_down_rsi = self.up_down_rsi.next(up_down);

        let roc = self.roc.next(src);
        let percent_rank = self.percent_rank.next(roc);

        let crsi = match (rsi.rsi, up_down_rsi.rsi, percent_rank) {
            (Some(rsi), Some(up_down_rsi), Some(percent_rank)) => {
                Some((rsi + up_down_rsi + percent_rank) / 3.0)
            }
            _ => None,
        };

        self.prev_ud = up_down;
        self.prev_src = src;

        return ConnorsRelativeStrengthIndexIndicatorResult { value: crsi };
    }
}
