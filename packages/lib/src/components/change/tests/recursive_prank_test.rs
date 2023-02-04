#[cfg(test)]
mod tests {
    use crate::{
        components::{
            change::{
                recursive_change::RecursiveChange, recursive_prank::RecursivePercentRank,
                recursive_roc::RecursiveRateOfChange,
            },
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
        target: &mut RecursivePercentRank,
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
    fn test_recursive_rank_1_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/prank/btc_1d_length_1_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursivePercentRank::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn test_recursive_rank_2_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/prank/btc_1d_length_2_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursivePercentRank::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn test_recursive_rank_3_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/prank/btc_1d_length_3_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursivePercentRank::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn test_recursive_rank_14_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/prank/btc_1d_length_14_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursivePercentRank::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn test_recursive_rank_100_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/prank/btc_1d_length_100_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursivePercentRank::new(ctx.clone(), 100),
            &expected,
        );
    }

    #[test]
    fn test_recursive_rank_365_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/prank/btc_1d_length_365_close.csv");
        _test(
            &mut ctx.clone(),
            &mut RecursivePercentRank::new(ctx.clone(), 365),
            &expected,
        );
    }

    fn _test_with_roc(
        cctx: &mut ComponentContext,
        target: &mut RecursivePercentRank,
        target_roc: &mut RecursiveRateOfChange,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let output_roc = target_roc.next(ctx.close());
            let output = target.next(output_roc);
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_recursive_rank_14_length_with_rate_of_change_7_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/prank/roc/btc_1d_prank_length_14_roc_length_7_close.csv");
        _test_with_roc(
            &mut ctx.clone(),
            &mut RecursivePercentRank::new(ctx.clone(), 14),
            &mut RecursiveRateOfChange::new(ctx.clone(), 7),
            &expected,
        );
    }

    #[test]
    fn test_recursive_rank_100_length_with_rate_of_change_7_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/prank/roc/btc_1d_prank_length_100_roc_length_7_close.csv");
        _test_with_roc(
            &mut ctx.clone(),
            &mut RecursivePercentRank::new(ctx.clone(), 100),
            &mut RecursiveRateOfChange::new(ctx.clone(), 7),
            &expected,
        );
    }

    #[test]
    fn test_recursive_rank_100_length_with_rate_of_change_2_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/prank/roc/btc_1d_prank_length_100_roc_length_2_close.csv");
        _test_with_roc(
            &mut ctx.clone(),
            &mut RecursivePercentRank::new(ctx.clone(), 100),
            &mut RecursiveRateOfChange::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn test_recursive_rank_100_length_with_rate_of_change_1_length_btc_1d_close() {
        let (_df, ctx, expected) =
            Fixture::load("components/change/tests/fixtures/prank/roc/btc_1d_prank_length_100_roc_length_1_close.csv");
        _test_with_roc(
            &mut ctx.clone(),
            &mut RecursivePercentRank::new(ctx.clone(), 100),
            &mut RecursiveRateOfChange::new(ctx.clone(), 1),
            &expected,
        );
    }

    // #[test]
    // fn test_recursive_rank_7_length_with_rate_of_change_14_length_btc_1d_close() {
    //     let (_df, ctx, expected) =
    //         Fixture::load("components/change/tests/fixtures/prank/roc/btc_1d_prank_length_7_roc_length_14_close.csv");
    //     _test_with_roc(
    //         &mut ctx.clone(),
    //         &mut RecursivePercentRank::new(ctx.clone(), 7),
    //         &mut RecursiveRateOfChange::new(ctx.clone(), 14),
    //         &expected,
    //     );
    // }

    // #[test]
    // fn test_recursive_rank_7_length_with_rate_of_change_1_length_btc_1d_close() {
    //     let (_df, ctx, expected) =
    //         Fixture::load("components/change/tests/fixtures/prank/roc/btc_1d_prank_length_7_roc_length_1_close.csv");
    //     _test_with_roc(
    //         &mut ctx.clone(),
    //         &mut RecursivePercentRank::new(ctx.clone(), 7),
    //         &mut RecursiveRateOfChange::new(ctx.clone(), 1),
    //         &expected,
    //     );
    // }
}
