#[cfg(test)]
mod tests {
    use crate::{
        components::{
            change::{recursive_change::RecursiveChange, recursive_roc::RecursiveRateOfChange},
            component_context::ComponentContext,
            stoch::recursive_stoch::RecursiveStoch,
            testing::ComponentTestSnapshot,
        },
        ta::{
            cross::{cross::CrossMode, cross_component::CrossComponent},
            relative_strength_index::rsi_component::RelativeStrengthIndexComponent,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RecursiveRateOfChange,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let ouptut = target.next(ctx.close());
            snapshot.push(ouptut);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_recursive_rate_of_change_1_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/roc/btc_1d_length_1_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursiveRateOfChange::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn test_recursive_rate_of_change_2_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/roc/btc_1d_length_2_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursiveRateOfChange::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn test_recursive_rate_of_change_3_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/roc/btc_1d_length_3_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursiveRateOfChange::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn test_recursive_rate_of_change_365_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/roc/btc_1d_length_365_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursiveRateOfChange::new(ctx.clone(), 365),
            &expected,
        );
    }
}
