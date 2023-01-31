use crate::{
    components::{
        component_context::ComponentContext, dev::dev_component::DeviationComponent,
        source::Source, sum::recursive_sum::RecursiveSum,
    },
    ta::{
        bars::utils::BarUtils,
        moving_average::{
            ma::MovingAverageKind, ma_component::MovingAverageComponent,
            sma_component::SimpleMovingAverageComponent,
        },
        true_range::atr_component::AverageTrueRangeComponent,
    },
};

pub struct CommodityChannelIndexIndicatorConfig {
    pub length: usize,
    pub src: Source,
    pub ma_kind: MovingAverageKind,
}

pub struct CommodityChannelIndexIndicatorResult {
    pub value: Option<f64>,
}

pub struct CommodityChannelIndexIndicator {
    pub config: CommodityChannelIndexIndicatorConfig,
    ctx: ComponentContext,
    sma: SimpleMovingAverageComponent,
    dev: DeviationComponent,
}

impl CommodityChannelIndexIndicator {
    pub fn new(ctx: ComponentContext, config: CommodityChannelIndexIndicatorConfig) -> Self {
        return CommodityChannelIndexIndicator {
            ctx: ctx.clone(),
            sma: SimpleMovingAverageComponent::new(ctx.clone(), config.length),
            dev: DeviationComponent::new(ctx.clone(), config.length),
            config,
        };
    }

    pub fn next(&mut self) -> CommodityChannelIndexIndicatorResult {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let src = self.config.src.get();
        let ma = self.sma.next(src);
        let dev = self.dev.next(src);

        let cci = match (src, ma, dev) {
            (Some(src), Some(ma), Some(dev)) => {
                if dev == 0.0 {
                    None
                } else {
                    Some((src - ma) / (0.015 * dev))
                }
            }
            _ => None,
        };

        return CommodityChannelIndexIndicatorResult { value: cci };
    }
}
