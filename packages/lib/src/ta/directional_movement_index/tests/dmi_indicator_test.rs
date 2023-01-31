#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component_context::ComponentContext,
            source::{Source, SourceKind},
            testing::ComponentTestSnapshot,
        },
        data::polars::SeriesCastUtils,
        ta::{
            aroon::aroon_indicator::{AroonIndicator, AroonIndicatorConfig},
            awesome_oscillator::awesome_oscillator_indicator::{
                AwesomeOscillatorIndicator, AwesomeOscillatorIndicatorConfig,
            },
            directional_movement_index::dmi_indicator::{
                DirectionalMovementIndexIndicator, DirectionalMovementIndexIndicatorConfig,
            },
            moving_average::ma::MovingAverageKind,
            price_oscillator::price_oscillator_indicator::{
                PriceOscillatorIndicator, PriceOscillatorIndicatorConfig,
            },
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut DirectionalMovementIndexIndicator,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Option<f64>, Option<f64>, Option<f64>)>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(Some((output.plus, output.minus, output.adx)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_directional_movement_index_btc_1d_14_length_lensig_14() {
        let (_df, ctx) = Fixture::raw(
            "ta/directional_movement_index/tests/fixtures/indicator/btc_1d_14_length_lensig_14.csv",
        );
        let plus_values = _df.column("_target_plus_").unwrap().to_f64();
        let minus_values = _df.column("_target_minus_").unwrap().to_f64();
        let adx_values = _df.column("_target_adx_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>, Option<f64>)>> = plus_values
            .iter()
            .zip(minus_values.iter())
            .zip(adx_values.iter())
            .map(|(((plus, minus), adx))| Some((*plus, *minus, *adx)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut DirectionalMovementIndexIndicator::new(
                ctx.clone(),
                DirectionalMovementIndexIndicatorConfig {
                    length: 14,
                    lensig: 14,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn test_directional_movement_index_btc_1d_3_length_lensig_3() {
        let (_df, ctx) = Fixture::raw(
            "ta/directional_movement_index/tests/fixtures/indicator/btc_1d_3_length_lensig_3.csv",
        );
        let plus_values = _df.column("_target_plus_").unwrap().to_f64();
        let minus_values = _df.column("_target_minus_").unwrap().to_f64();
        let adx_values = _df.column("_target_adx_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>, Option<f64>)>> = plus_values
            .iter()
            .zip(minus_values.iter())
            .zip(adx_values.iter())
            .map(|(((plus, minus), adx))| Some((*plus, *minus, *adx)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut DirectionalMovementIndexIndicator::new(
                ctx.clone(),
                DirectionalMovementIndexIndicatorConfig {
                    length: 3,
                    lensig: 3,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn test_directional_movement_index_btc_1d_14_length_lensig_3() {
        let (_df, ctx) = Fixture::raw(
            "ta/directional_movement_index/tests/fixtures/indicator/btc_1d_14_length_lensig_3.csv",
        );
        let plus_values = _df.column("_target_plus_").unwrap().to_f64();
        let minus_values = _df.column("_target_minus_").unwrap().to_f64();
        let adx_values = _df.column("_target_adx_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>, Option<f64>)>> = plus_values
            .iter()
            .zip(minus_values.iter())
            .zip(adx_values.iter())
            .map(|(((plus, minus), adx))| Some((*plus, *minus, *adx)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut DirectionalMovementIndexIndicator::new(
                ctx.clone(),
                DirectionalMovementIndexIndicatorConfig {
                    length: 14,
                    lensig: 3,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn test_directional_movement_index_btc_1d_3_length_lensig_14() {
        let (_df, ctx) = Fixture::raw(
            "ta/directional_movement_index/tests/fixtures/indicator/btc_1d_3_length_lensig_14.csv",
        );
        let plus_values = _df.column("_target_plus_").unwrap().to_f64();
        let minus_values = _df.column("_target_minus_").unwrap().to_f64();
        let adx_values = _df.column("_target_adx_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>, Option<f64>)>> = plus_values
            .iter()
            .zip(minus_values.iter())
            .zip(adx_values.iter())
            .map(|(((plus, minus), adx))| Some((*plus, *minus, *adx)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut DirectionalMovementIndexIndicator::new(
                ctx.clone(),
                DirectionalMovementIndexIndicatorConfig {
                    length: 3,
                    lensig: 14,
                },
            ),
            &expected,
        );
    }
}
