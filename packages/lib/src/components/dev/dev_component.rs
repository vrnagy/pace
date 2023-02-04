use crate::{
    components::{
        batch_validator::recursive_batch_validator::RecursiveBatchValidator,
        component_context::ComponentContext, lifo::recursive_lifo::RecursiveLIFO,
    },
    math::comparison::FloatComparison,
    ta::moving_average::{
        rma_component::RunningMovingAverageComponent, sma_component::SimpleMovingAverageComponent,
    },
};

pub struct DeviationComponent {
    pub length: usize,
    ctx: ComponentContext,
    sma: SimpleMovingAverageComponent,
    value_lifo: RecursiveLIFO,
}

impl DeviationComponent {
    // biased by default
    pub fn new(ctx: ComponentContext, length: usize) -> Self {
        assert!(
            length > 0,
            "DeviationComponent must have a length of at least 1"
        );
        return DeviationComponent {
            ctx: ctx.clone(),
            length,
            sma: SimpleMovingAverageComponent::new(ctx.clone(), length),
            value_lifo: RecursiveLIFO::new(ctx.clone(), length),
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();

        if self.length == 1 {
            return Some(0.0);
        }

        let mean = self.sma.next(value);
        let (first_value, last_value, is_filled) = self.value_lifo.next(value);

        if last_value.is_none() || mean.is_none() || !is_filled {
            return None;
        }

        let mean = mean.unwrap();
        let mut sum: f64 = 0.0;

        if let Some(first_value) = first_value {
            sum += (first_value - mean).abs();
        }

        let values = self.value_lifo.values();

        for i in 0..self.length - 1 {
            let _value = values[i];
            if let Some(_value) = _value {
                sum += (_value - mean).abs();
            }
        }

        let dev = sum / self.length as f64;

        return Some(dev);
    }
}
