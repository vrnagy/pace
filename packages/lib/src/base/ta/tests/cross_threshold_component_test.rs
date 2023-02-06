#[cfg(test)]
mod tests {
    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::{
            cross::CrossMode, cross_component::CrossComponent,
            cross_threshold_component::CrossThresholdComponent,
            rsi_component::RelativeStrengthIndexComponent,
        },
    };

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/cross/{}", path)
    }

    fn _test(
        cctx: &mut ComponentContext,
        target_cross: &mut CrossThresholdComponent,
        target_rsi: &mut RelativeStrengthIndexComponent,
        mode: CrossMode,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output_rsi = target_rsi.next(cctx.get().close());
            let output = target_cross.next(output_rsi);
            let output = match output {
                Some(output) => output == mode,
                None => false,
            };
            let output = if output { 1.0 } else { 0.0 };
            snapshot.push(Some(output));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn over_with_rsi_length_14_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("over/rsi/length_14_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut CrossThresholdComponent::new(ctx.clone(), 30.0),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 14),
            CrossMode::Over,
            &expected,
        );
    }

    #[test]
    fn under_with_rsi_length_14_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("under/rsi/length_14_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut CrossThresholdComponent::new(ctx.clone(), 70.0),
            &mut RelativeStrengthIndexComponent::new(ctx.clone(), 14),
            CrossMode::Under,
            &expected,
        );
    }
}
