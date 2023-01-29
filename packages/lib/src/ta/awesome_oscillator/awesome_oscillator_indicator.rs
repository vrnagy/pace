use crate::{
    components::{component_context::ComponentContext, source::Source},
    ta::{
        bars::utils::BarUtils,
        moving_average::{ma::MovingAverageKind, ma_component::MovingAverageComponent},
    },
};

pub struct AwesomeOscillatorIndicatorConfig {
    // default: 5
    pub short_length: usize,
    // default: 34
    pub long_length: usize,
    // default: HL2
    pub short_source: Source,
    // default: HL2
    pub long_source: Source,
    // default: SMA
    pub short_ma_type: MovingAverageKind,
    // default: SMA
    pub long_ma_type: MovingAverageKind,
}

pub struct AwesomeOscillatorIndicatorResult {
    pub value: Option<f64>,
}

pub struct AwesomeOscillatorIndicator {
    config: AwesomeOscillatorIndicatorConfig,
    ctx: ComponentContext,
    short_ma: MovingAverageComponent,
    long_ma: MovingAverageComponent,
    prev_ao: Option<f64>,
}

impl AwesomeOscillatorIndicator {
    pub fn new(ctx: ComponentContext, config: AwesomeOscillatorIndicatorConfig) -> Self {
        return AwesomeOscillatorIndicator {
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

    pub fn next(&mut self) -> AwesomeOscillatorIndicatorResult {
        self.ctx.assert();

        let ctx = self.ctx.get();

        let short_ma_src = self.config.short_source.get();
        let long_ma_src = self.config.long_source.get();

        let short_ma = self.short_ma.next(short_ma_src);
        let long_ma = self.long_ma.next(long_ma_src);

        let ao = match (short_ma, long_ma) {
            (Some(short_ma), Some(long_ma)) => Some(short_ma - long_ma),
            _ => None,
        };

        let diff = match (ao, self.prev_ao) {
            (Some(ao), Some(prev_ao)) => Some(ao - prev_ao),
            _ => None,
        };

        self.prev_ao = ao;

        return AwesomeOscillatorIndicatorResult { value: diff };
    }
}
