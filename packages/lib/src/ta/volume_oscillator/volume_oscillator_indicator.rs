use crate::{
    components::{component_context::ComponentContext, source::Source},
    ta::{
        bars::utils::BarUtils,
        moving_average::{
            ema_component::ExponentialMovingAverageComponent, ma::MovingAverageKind,
            ma_component::MovingAverageComponent,
        },
    },
};

pub struct VolumeOscillatorIndicatorConfig {
    pub short_length: usize,
    pub long_length: usize,
}

pub struct VolumeOscillatorIndicatorResult {
    pub value: Option<f64>,
}

pub struct VolumeOscillatorIndicator {
    config: VolumeOscillatorIndicatorConfig,
    ctx: ComponentContext,
    short_ma: ExponentialMovingAverageComponent,
    long_ma: ExponentialMovingAverageComponent,
}

impl VolumeOscillatorIndicator {
    pub fn new(ctx: ComponentContext, config: VolumeOscillatorIndicatorConfig) -> Self {
        return VolumeOscillatorIndicator {
            ctx: ctx.clone(),
            short_ma: ExponentialMovingAverageComponent::new(ctx.clone(), config.short_length),
            long_ma: ExponentialMovingAverageComponent::new(ctx.clone(), config.long_length),
            config,
        };
    }

    pub fn next(&mut self) -> VolumeOscillatorIndicatorResult {
        self.ctx.assert();

        let ctx = self.ctx.get();
        let volume = ctx.volume();

        let short_ma = self.short_ma.next(volume);
        let long_ma = self.long_ma.next(volume);

        let osc = match (short_ma, long_ma) {
            (Some(short_ma), Some(long_ma)) => {
                if long_ma == 0.0 {
                    None
                } else {
                    Some((short_ma - long_ma) / long_ma * 100.0)
                }
            }
            _ => None,
        };

        return VolumeOscillatorIndicatorResult { value: osc };
    }
}
