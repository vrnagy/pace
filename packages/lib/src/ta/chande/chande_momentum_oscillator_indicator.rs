use crate::{
    components::{
        component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO, source::Source,
        sum::recursive_sum::RecursiveSum,
    },
    pinescript::utils::{ps_abs, ps_diff, ps_max, ps_min},
    ta::{
        bars::utils::BarUtils,
        moving_average::{ma::MovingAverageKind, ma_component::MovingAverageComponent},
        true_range::atr_component::AverageTrueRangeComponent,
    },
};

pub struct ChandeMomentumOscillatorIndicatorConfig {
    pub length: usize,
    pub src: Source,
}

pub struct ChandeMomentumOscillatorIndicatorResult {
    pub value: Option<f64>,
}

pub struct ChandeMomentumOscillatorIndicator {
    pub config: ChandeMomentumOscillatorIndicatorConfig,
    ctx: ComponentContext,
    prev_src: Option<f64>,
    prev_m1: Option<f64>,
    prev_m2: Option<f64>,
    sm1: RecursiveSum,
    sm2: RecursiveSum,
}

impl ChandeMomentumOscillatorIndicator {
    pub fn new(ctx: ComponentContext, config: ChandeMomentumOscillatorIndicatorConfig) -> Self {
        assert!(
            config.length > 1,
            "ChandeMomentumOscillatorIndicator length must be greater than 1"
        );
        return ChandeMomentumOscillatorIndicator {
            ctx: ctx.clone(),
            prev_src: None,
            prev_m1: None,
            prev_m2: None,
            sm1: RecursiveSum::new(ctx.clone(), config.length),
            sm2: RecursiveSum::new(ctx.clone(), config.length),
            config,
        };
    }

    pub fn next(&mut self) -> ChandeMomentumOscillatorIndicatorResult {
        self.ctx.assert();
        let ctx = self.ctx.get();

        let src = self.config.src.get();
        let momm = ps_diff(src, self.prev_src);

        let m1 = ps_max(Some(0.0), momm);
        let m2 = ps_abs(ps_min(Some(0.0), momm));

        let sm1 = self.sm1.next(m1);
        let sm2 = self.sm2.next(m2);

        let chande_mo: Option<f64> = match (sm1, sm2) {
            (Some(sm1), Some(sm2)) => {
                if sm1 == -sm2 {
                    None
                } else {
                    Some(100.0 * (sm1 - sm2) / (sm1 + sm2))
                }
            }
            _ => None,
        };

        self.prev_src = src;

        return ChandeMomentumOscillatorIndicatorResult { value: chande_mo };
    }
}
