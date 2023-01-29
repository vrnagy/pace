use crate::{
    components::{
        component_context::ComponentContext, source::Source, sum::recursive_sum::RecursiveSum,
    },
    ta::{
        bars::utils::BarUtils,
        moving_average::{ma::MovingAverageKind, ma_component::MovingAverageComponent},
    },
};

pub struct ChaikinMoneyFlowIndicatorConfig {
    pub length: usize,
}

pub struct ChaikinMoneyFlowIndicatorResult {
    pub value: Option<f64>,
}

pub struct ChaikinMoneyFlowIndicator {
    pub config: ChaikinMoneyFlowIndicatorConfig,
    ctx: ComponentContext,
    volume_sum: RecursiveSum,
    ad_sum: RecursiveSum,
}

impl ChaikinMoneyFlowIndicator {
    pub fn new(ctx: ComponentContext, config: ChaikinMoneyFlowIndicatorConfig) -> Self {
        return ChaikinMoneyFlowIndicator {
            ctx: ctx.clone(),
            volume_sum: RecursiveSum::new(ctx.clone(), config.length),
            ad_sum: RecursiveSum::new(ctx.clone(), config.length),
            config,
        };
    }

    pub fn next(&mut self) -> ChaikinMoneyFlowIndicatorResult {
        self.ctx.assert();

        let ctx = self.ctx.get();

        let close = ctx.close();
        let high = ctx.high();
        let low = ctx.low();
        let volume = ctx.volume();

        let volume_sum = self.volume_sum.next(volume);

        let ad: Option<f64> = match (close, high, low, volume) {
            (Some(close), Some(high), Some(low), Some(volume)) => {
                if close == high && close == low || high == low {
                    Some(0.0)
                } else {
                    Some(((2.0 * close - low - high) / (high - low)) * volume)
                }
            }
            _ => None,
        };

        let ad_sum = self.ad_sum.next(ad);

        if ad_sum.is_none() || volume_sum.is_none() || volume_sum.unwrap() == 0.0 {
            return ChaikinMoneyFlowIndicatorResult { value: None };
        }

        let cmf = ad_sum.unwrap() / volume_sum.unwrap();

        return ChaikinMoneyFlowIndicatorResult { value: Some(cmf) };
    }
}
