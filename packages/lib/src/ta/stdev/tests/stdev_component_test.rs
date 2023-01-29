#[cfg(test)]
mod tests {
    use crate::{
        components::{component_context::ComponentContext, testing::ComponentTestSnapshot},
        ta::{
            cross::{cross::CrossMode, cross_component::CrossComponent},
            relative_strength_index::rsi_component::RelativeStrengthIndexComponent,
            stdev::stdev_component::StandardDeviationComponent,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut StandardDeviationComponent,
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
    fn test_stdev_unbiased_length_14_btc_1d() {
        let (_df, ctx, expected) =
            Fixture::load("ta/stdev/tests/fixtures/component/btc_1d_length_14_unbiased.csv");
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 14, false),
            &expected,
        );
    }

    #[test]
    fn test_stdev_unbiased_length_365_btc_1d() {
        let (_df, ctx, expected) =
            Fixture::load("ta/stdev/tests/fixtures/component/btc_1d_length_365_unbiased.csv");
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 365, false),
            &expected,
        );
    }

    #[test]
    fn test_stdev_biased_length_14_btc_1d() {
        let (_df, ctx, expected) =
            Fixture::load("ta/stdev/tests/fixtures/component/btc_1d_length_14_biased.csv");
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 14, true),
            &expected,
        );
    }

    #[test]
    fn test_stdev_biased_length_365_btc_1d() {
        let (_df, ctx, expected) =
            Fixture::load("ta/stdev/tests/fixtures/component/btc_1d_length_365_biased.csv");
        _test(
            &mut ctx.clone(),
            &mut StandardDeviationComponent::new(ctx.clone(), 365, true),
            &expected,
        );
    }
}
