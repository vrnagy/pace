use crate::components::{
    component_context::ComponentContext, position::recursive_position::RecursivePosition,
};

pub struct RecursiveFixNan {
    ctx: ComponentContext,
    last_non_nan_value: Option<f64>,
}

impl RecursiveFixNan {
    pub fn new(ctx: ComponentContext) -> Self {
        return RecursiveFixNan {
            ctx: ctx.clone(),
            last_non_nan_value: None,
        };
    }

    pub fn next(&mut self, value: Option<f64>) -> Option<f64> {
        self.ctx.assert();

        match value {
            Some(value) => {
                self.last_non_nan_value = Some(value);
                return Some(value);
            }
            None => {
                return self.last_non_nan_value;
            }
        }
    }
}
