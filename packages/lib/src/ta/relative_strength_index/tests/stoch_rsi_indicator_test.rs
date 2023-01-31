#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component_context::ComponentContext,
            source::{Source, SourceKind},
            testing::ComponentTestSnapshot,
        },
        data::polars::SeriesCastUtils,
        ta::relative_strength_index::{
            rsi_indicator::{RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig},
            stoch_rsi_indicator::{
                StochRelativeStrengthIndexIndicator, StochRelativeStrengthIndexIndicatorConfig,
            },
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut StochRelativeStrengthIndexIndicator,
        expected: &[Option<(Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Option<f64>, Option<f64>)>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(Some((output.k, output.d)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_stoch_rsi_length_14_stoch_length_14_k_3_d_3_close_btc_1d() {
        let (_df, ctx) = Fixture::raw(
            "ta/relative_strength_index/tests/fixtures/stoch_rsi/indicator/btc_1d_rsi_length_14_stoch_length_14_k_3_d_3_close.csv",
        );
        let k_values = _df.column("_target_k_").unwrap().to_f64();
        let d_values = _df.column("_target_d_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>)>> = k_values
            .iter()
            .zip(d_values.iter())
            .map(|(k, d)| Some((*k, *d)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut StochRelativeStrengthIndexIndicator::new(
                ctx.clone(),
                StochRelativeStrengthIndexIndicatorConfig {
                    length_rsi: 14,
                    length_stoch: 14,
                    smooth_d: 3,
                    smooth_k: 3,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }

    #[test]
    fn test_stoch_rsi_length_2_stoch_length_2_k_3_d_3_close_btc_1d() {
        let (_df, ctx) = Fixture::raw(
            "ta/relative_strength_index/tests/fixtures/stoch_rsi/indicator/btc_1d_rsi_length_2_stoch_length_2_k_3_d_3_close.csv",
        );
        let k_values = _df.column("_target_k_").unwrap().to_f64();
        let d_values = _df.column("_target_d_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>)>> = k_values
            .iter()
            .zip(d_values.iter())
            .map(|(k, d)| Some((*k, *d)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut StochRelativeStrengthIndexIndicator::new(
                ctx.clone(),
                StochRelativeStrengthIndexIndicatorConfig {
                    length_rsi: 2,
                    length_stoch: 2,
                    smooth_d: 3,
                    smooth_k: 3,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }

    #[test]
    fn test_stoch_rsi_length_2_stoch_length_2_k_14_d_14_close_btc_1d() {
        let (_df, ctx) = Fixture::raw(
            "ta/relative_strength_index/tests/fixtures/stoch_rsi/indicator/btc_1d_rsi_length_2_stoch_length_2_k_14_d_14_close.csv",
        );
        let k_values = _df.column("_target_k_").unwrap().to_f64();
        let d_values = _df.column("_target_d_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>)>> = k_values
            .iter()
            .zip(d_values.iter())
            .map(|(k, d)| Some((*k, *d)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut StochRelativeStrengthIndexIndicator::new(
                ctx.clone(),
                StochRelativeStrengthIndexIndicatorConfig {
                    length_rsi: 2,
                    length_stoch: 2,
                    smooth_d: 14,
                    smooth_k: 14,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
