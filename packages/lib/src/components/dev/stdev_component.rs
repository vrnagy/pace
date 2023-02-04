use crate::{
    components::{component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO},
    math::comparison::FloatComparison,
    ta::moving_average::{
        rma_component::RunningMovingAverageComponent, sma_component::SimpleMovingAverageComponent,
    },
};

pub struct StandardDeviationComponent {
    pub length: usize,
    pub is_biased: bool,
    ctx: ComponentContext,
    sma: SimpleMovingAverageComponent,
    value_lifo: RecursiveLIFO,
}

impl StandardDeviationComponent {
    // biased by default
    pub fn new(ctx: ComponentContext, length: usize, is_biased: bool) -> Self {
        assert!(
            length > 0,
            "StandardDeviationComponent must have a length of at least 1"
        );
        return StandardDeviationComponent {
            ctx: ctx.clone(),
            length,
            is_biased,
            sma: SimpleMovingAverageComponent::new(ctx.clone(), length),
            value_lifo: RecursiveLIFO::new(ctx.clone(), length),
        };
    }

    fn compute_sum(fst: f64, snd: f64) -> f64 {
        let sum = fst + snd;
        if sum.compare_with_precision(0.0, 1e-10) {
            return 0.0;
        }
        return sum;
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();

        if self.length == 1 {
            if self.is_biased {
                return Some(0.0);
            } else {
                return None;
            }
        }

        let avg = self.sma.next(value);
        let (first_value, last_value, is_filled) = self.value_lifo.next(value);

        if last_value.is_none() || avg.is_none() {
            return None;
        }

        let mut sum_of_square_deviations: f64 = 0.0;
        let avg = -avg.unwrap();

        if let Some(first_value) = first_value {
            let sum = Self::compute_sum(first_value, avg);
            sum_of_square_deviations += sum.powf(2.0);
        }

        let values = self.value_lifo.values();

        for i in 0..self.length - 1 {
            let _value = values[i];
            if let Some(_value) = _value {
                let sum = Self::compute_sum(_value, avg);
                sum_of_square_deviations += sum.powf(2.0);
            }
        }

        let stdev = if self.is_biased {
            (sum_of_square_deviations / self.length as f64).sqrt()
        } else {
            (sum_of_square_deviations / (self.length - 1) as f64).sqrt()
        };

        return Some(stdev);
    }
}
