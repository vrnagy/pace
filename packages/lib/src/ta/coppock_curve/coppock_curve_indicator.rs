use crate::{
    components::{
        change::recursive_roc::RecursiveRateOfChange, component_context::ComponentContext,
        dev::dev_component::DeviationComponent, source::Source, sum::recursive_sum::RecursiveSum,
    },
    ta::{
        bars::utils::BarUtils,
        moving_average::{
            ma::MovingAverageKind, ma_component::MovingAverageComponent,
            sma_component::SimpleMovingAverageComponent,
            wma_component::WeightedMovingAverageComponent,
        },
        true_range::atr_component::AverageTrueRangeComponent,
    },
};

pub struct CoppockCurveIndicatorConfig {
    pub src: Source,
    pub long_roc_length: usize,
    pub short_roc_length: usize,
    pub ma_length: usize,
}

pub struct CoppockCurveIndicatorResult {
    pub value: Option<f64>,
}

pub struct CoppockCurveIndicator {
    pub config: CoppockCurveIndicatorConfig,
    ctx: ComponentContext,
    ma: WeightedMovingAverageComponent,
    long_roc: RecursiveRateOfChange,
    short_roc: RecursiveRateOfChange,
}

impl CoppockCurveIndicator {
    pub fn new(ctx: ComponentContext, config: CoppockCurveIndicatorConfig) -> Self {
        return CoppockCurveIndicator {
            ctx: ctx.clone(),
            ma: WeightedMovingAverageComponent::new(ctx.clone(), config.ma_length),
            long_roc: RecursiveRateOfChange::new(ctx.clone(), config.long_roc_length),
            short_roc: RecursiveRateOfChange::new(ctx.clone(), config.short_roc_length),
            config,
        };
    }

    pub fn next(&mut self) -> CoppockCurveIndicatorResult {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let src = self.config.src.get();

        let long_roc = self.long_roc.next(src);
        let short_roc = self.short_roc.next(src);
        let roc = match (long_roc, short_roc) {
            (Some(long_roc), Some(short_roc)) => Some(long_roc + short_roc),
            _ => None,
        };
        let curve = self.ma.next(roc);

        return CoppockCurveIndicatorResult { value: curve };
    }
}
