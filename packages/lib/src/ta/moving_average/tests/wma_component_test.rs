#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component_context::ComponentContext, dev::stdev_component::StandardDeviationComponent,
            testing::ComponentTestSnapshot,
        },
        ta::moving_average::{
            sma_component::SimpleMovingAverageComponent,
            wma_component::WeightedMovingAverageComponent,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut WeightedMovingAverageComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next(cctx.get().close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_wma_btc_1d_length_1_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/wma/btc_1d_length_1_close.csv");
        _test(
            &mut ctx.clone(),
            &mut WeightedMovingAverageComponent::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn test_wma_btc_1d_length_2_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/wma/btc_1d_length_2_close.csv");
        _test(
            &mut ctx.clone(),
            &mut WeightedMovingAverageComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn test_wma_btc_1d_length_3_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/wma/btc_1d_length_3_close.csv");
        _test(
            &mut ctx.clone(),
            &mut WeightedMovingAverageComponent::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn test_wma_btc_1d_length_14_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/wma/btc_1d_length_14_close.csv");
        _test(
            &mut ctx.clone(),
            &mut WeightedMovingAverageComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn test_wma_btc_1d_length_365_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/wma/btc_1d_length_365_close.csv");
        _test(
            &mut ctx.clone(),
            &mut WeightedMovingAverageComponent::new(ctx.clone(), 365),
            &expected,
        );
    }
}