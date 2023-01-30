#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component_context::ComponentContext, stoch::recursive_stoch::RecursiveStoch,
            testing::ComponentTestSnapshot,
        },
        ta::{
            cross::{cross::CrossMode, cross_component::CrossComponent},
            relative_strength_index::rsi_component::RelativeStrengthIndexComponent,
        },
        testing::fixture::Fixture,
    };

    fn _test_close_high_low(
        cctx: &mut ComponentContext,
        target: &mut RecursiveStoch,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let ouptut = target.next(ctx.close(), ctx.high(), ctx.low());
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    fn _test_close_close_close(
        cctx: &mut ComponentContext,
        target: &mut RecursiveStoch,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let ouptut = target.next(ctx.close(), ctx.close(), ctx.close());
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_stoch_btc_1d_length_14_close_high_low() {
        let (_df, ctx, expected) =
            Fixture::load("components/stoch/tests/fixtures/btc_1d_length_14_close_high_low.csv");
        _test_close_high_low(
            &mut ctx.clone(),
            &mut RecursiveStoch::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn test_stoch_btc_1d_length_1_close_high_low() {
        let (_df, ctx, expected) =
            Fixture::load("components/stoch/tests/fixtures/btc_1d_length_1_close_high_low.csv");
        _test_close_high_low(
            &mut ctx.clone(),
            &mut RecursiveStoch::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn test_stoch_btc_1d_length_2_close_high_low() {
        let (_df, ctx, expected) =
            Fixture::load("components/stoch/tests/fixtures/btc_1d_length_2_close_high_low.csv");
        _test_close_high_low(
            &mut ctx.clone(),
            &mut RecursiveStoch::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn test_stoch_btc_1d_length_3_close_high_low() {
        let (_df, ctx, expected) =
            Fixture::load("components/stoch/tests/fixtures/btc_1d_length_3_close_high_low.csv");
        _test_close_high_low(
            &mut ctx.clone(),
            &mut RecursiveStoch::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn test_stoch_btc_1d_length_1_close_close_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/stoch/tests/fixtures/btc_1d_length_1_close_close_close.csv");
        _test_close_close_close(
            &mut ctx.clone(),
            &mut RecursiveStoch::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn test_stoch_btc_1d_length_2_close_close_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/stoch/tests/fixtures/btc_1d_length_2_close_close_close.csv");
        _test_close_close_close(
            &mut ctx.clone(),
            &mut RecursiveStoch::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn test_stoch_btc_1d_length_3_close_close_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/stoch/tests/fixtures/btc_1d_length_3_close_close_close.csv");
        _test_close_close_close(
            &mut ctx.clone(),
            &mut RecursiveStoch::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn test_stoch_btc_1d_length_14_close_close_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/stoch/tests/fixtures/btc_1d_length_14_close_close_close.csv");
        _test_close_close_close(
            &mut ctx.clone(),
            &mut RecursiveStoch::new(ctx.clone(), 14),
            &expected,
        );
    }
}
