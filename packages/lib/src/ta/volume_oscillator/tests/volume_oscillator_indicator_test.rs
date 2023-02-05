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
            moving_average::ma::MovingAverageKind,
            price_oscillator::price_oscillator_indicator::{
                PriceOscillatorIndicator, PriceOscillatorIndicatorConfig,
            },
            volume_oscillator::volume_oscillator_indicator::{
                VolumeOscillatorIndicator, VolumeOscillatorIndicatorConfig,
            },
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut VolumeOscillatorIndicator,
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
    fn test_volume_oscillator_oscillator_short_length_5_long_length_10() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/volume_oscillator/tests/fixtures/indicator/btc_1d_short_length_5_long_length_10.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut VolumeOscillatorIndicator::new(
                ctx.clone(),
                VolumeOscillatorIndicatorConfig {
                    short_length: 5,
                    long_length: 10,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn test_volume_oscillator_oscillator_short_length_1_long_length_1() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/volume_oscillator/tests/fixtures/indicator/btc_1d_short_length_1_long_length_1.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut VolumeOscillatorIndicator::new(
                ctx.clone(),
                VolumeOscillatorIndicatorConfig {
                    short_length: 1,
                    long_length: 1,
                },
            ),
            &expected,
        );
    }
}
