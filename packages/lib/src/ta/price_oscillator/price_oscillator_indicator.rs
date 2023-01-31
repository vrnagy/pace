use crate::{
    components::{component_context::ComponentContext, source::Source},
    ta::{
        bars::utils::BarUtils,
        moving_average::{ma::MovingAverageKind, ma_component::MovingAverageComponent},
    },
};

pub struct PriceOscillatorIndicatorConfig {
    // default: 10
    pub short_length: usize,
    // default: 21
    pub long_length: usize,
    // default: CLOSE
    pub src: Source,
    // default: EMA
    pub short_ma_type: MovingAverageKind,
    // default: EMA
    pub long_ma_type: MovingAverageKind,
}

pub struct PriceOscillatorIndicatorResult {
    pub value: Option<f64>,
}

pub struct PriceOscillatorIndicator {
    config: PriceOscillatorIndicatorConfig,
    ctx: ComponentContext,
    short_ma: MovingAverageComponent,
    long_ma: MovingAverageComponent,
    prev_ao: Option<f64>,
}

impl PriceOscillatorIndicator {
    pub fn new(ctx: ComponentContext, config: PriceOscillatorIndicatorConfig) -> Self {
        return PriceOscillatorIndicator {
            ctx: ctx.clone(),
            short_ma: MovingAverageComponent::new(
                ctx.clone(),
                config.short_length,
                config.short_ma_type,
            ),
            long_ma: MovingAverageComponent::new(
                ctx.clone(),
                config.long_length,
                config.long_ma_type,
            ),
            config,
            prev_ao: None,
        };
    }

    pub fn next(&mut self) -> PriceOscillatorIndicatorResult {
        self.ctx.assert();

        let ctx = self.ctx.get();

        let src = self.config.src.get();

        let short_ma = self.short_ma.next(src);
        let long_ma = self.long_ma.next(src);

        let po: Option<f64> = match (short_ma, long_ma) {
            (Some(short_ma), Some(long_ma)) => Some((short_ma - long_ma) / long_ma * 100.0),
            _ => None,
        };

        return PriceOscillatorIndicatorResult { value: po };
    }
}
