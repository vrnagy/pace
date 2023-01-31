#[cfg(test)]
mod tests {
    use crate::components::dev::dev_component::DeviationComponent;
    use crate::components::dev::stdev_component::StandardDeviationComponent;
    use crate::{
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        ta::{
            cross::{cross::CrossMode, cross_component::CrossComponent},
            relative_strength_index::rsi_component::RelativeStrengthIndexComponent,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut DeviationComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let output = target.next(ctx.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_dev_length_14_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/dev/tests/fixtures/dev/btc_1d_length_14_close.csv");
        _test(
            &mut ctx.clone(),
            &mut DeviationComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn test_dev_length_1_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/dev/tests/fixtures/dev/btc_1d_length_1_close.csv");
        _test(
            &mut ctx.clone(),
            &mut DeviationComponent::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn test_dev_length_2_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/dev/tests/fixtures/dev/btc_1d_length_2_close.csv");
        _test(
            &mut ctx.clone(),
            &mut DeviationComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn test_dev_length_3_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/dev/tests/fixtures/dev/btc_1d_length_3_close.csv");
        _test(
            &mut ctx.clone(),
            &mut DeviationComponent::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn test_dev_length_365_btc_1d() {
        let (_df, ctx, expected) =
            Fixture::load("components/dev/tests/fixtures/dev/btc_1d_length_365_close.csv");
        _test(
            &mut ctx.clone(),
            &mut DeviationComponent::new(ctx.clone(), 365),
            &expected,
        );
    }
}
