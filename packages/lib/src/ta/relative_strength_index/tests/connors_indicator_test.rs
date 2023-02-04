#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component_context::ComponentContext,
            source::{Source, SourceKind},
            testing::ComponentTestSnapshot,
        },
        ta::relative_strength_index::{
            connors_rsi_indicator::{
                ConnorsRelativeStrengthIndexIndicator, ConnorsRelativeStrengthIndexIndicatorConfig,
            },
            rsi_indicator::{RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig},
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut ConnorsRelativeStrengthIndexIndicator,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(output.value);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_connors_rsi_btc_1d_length_14_close() {
        let (_df, ctx, expected) =
            Fixture::load("ta/relative_strength_index/tests/fixtures/connors_indicator/btc_1d_rsi_length_3_up_down_len_2_roc_length_100_close.csv");
        _test(
            &mut ctx.clone(),
            &mut ConnorsRelativeStrengthIndexIndicator::new(
                ctx.clone(),
                ConnorsRelativeStrengthIndexIndicatorConfig {
                    length_rsi: 3,
                    length_up_down: 2,
                    length_roc: 100,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
