use crate::{
    components::{
        component_context::ComponentContext, dev::stdev_component::StandardDeviationComponent,
        source::Source,
    },
    ta::{
        bars::utils::BarUtils,
        moving_average::{
            ma::MovingAverageKind, ma_component::MovingAverageComponent,
            sma_component::SimpleMovingAverageComponent,
        },
    },
};

pub struct BollingerBandsWidthIndicatorConfig {
    pub length: usize,
    pub source: Source,
    pub mult: f64,
}

pub struct BollingerBandsWidthIndicatorResult {
    pub value: Option<f64>,
}

pub struct BollingerBandsWidthIndicator {
    pub config: BollingerBandsWidthIndicatorConfig,
    ctx: ComponentContext,
    basis: SimpleMovingAverageComponent,
    stdev: StandardDeviationComponent,
}

pub static BOLLINGER_BANDS_WIDTH_MULT: f64 = 2.0;

impl BollingerBandsWidthIndicator {
    pub fn new(ctx: ComponentContext, config: BollingerBandsWidthIndicatorConfig) -> Self {
        return BollingerBandsWidthIndicator {
            ctx: ctx.clone(),
            basis: SimpleMovingAverageComponent::new(ctx.clone(), config.length),
            stdev: StandardDeviationComponent::new(ctx.clone(), config.length, true),
            config,
        };
    }

    pub fn next(&mut self) -> BollingerBandsWidthIndicatorResult {
        self.ctx.assert();

        let ctx = self.ctx.get();

        let src = self.config.source.get();
        let basis = self.basis.next(src);
        let dev = self.stdev.next(src);

        if src.is_none() || basis.is_none() || dev.is_none() {
            return BollingerBandsWidthIndicatorResult { value: None };
        }

        let basis = basis.unwrap();

        if basis == 0.0 {
            return BollingerBandsWidthIndicatorResult { value: None };
        }

        let src = src.unwrap();
        let dev = dev.unwrap() * self.config.mult;
        let upper = basis + dev;
        let lower = basis - dev;
        let bbw = (upper - lower) / basis;

        return BollingerBandsWidthIndicatorResult { value: Some(bbw) };
    }
}
