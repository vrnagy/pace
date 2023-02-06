#[cfg(test)]
mod tests {
    use crate::{
        base::{
            components::{
                component_context::ComponentContext,
                testing::{ComponentTestSnapshot, Fixture},
            },
            ta::ma::MovingAverageKind,
        },
        content::{
            ultimate_oscillator_indicator::{
                UltimateOscillatorIndicator, UltimateOscillatorIndicatorConfig,
            },
            volume_oscillator_indicator::{
                VolumeOscillatorIndicator, VolumeOscillatorIndicatorConfig,
            },
            vortex_indicator::{VortexIndicator, VortexIndicatorConfig},
        },
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/ultimate_oscillator/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut UltimateOscillatorIndicator,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn short_length_7_mid_length_14_long_length_28() {
        let (_df, ctx, expected) = Fixture::load(&format_path(
            "short_length_7_mid_length_14_long_length_28.csv",
        ));
        _test(
            &mut ctx.clone(),
            &mut UltimateOscillatorIndicator::new(
                ctx.clone(),
                UltimateOscillatorIndicatorConfig {
                    short_length: 7,
                    mid_length: 14,
                    long_length: 28,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn short_length_1_mid_length_1_long_length_1() {
        let (_df, ctx, expected) = Fixture::load(&format_path(
            "short_length_1_mid_length_1_long_length_1.csv",
        ));
        _test(
            &mut ctx.clone(),
            &mut UltimateOscillatorIndicator::new(
                ctx.clone(),
                UltimateOscillatorIndicatorConfig {
                    short_length: 1,
                    mid_length: 1,
                    long_length: 1,
                },
            ),
            &expected,
        );
    }

    #[test]
    fn short_length_30_mid_length_15_long_length_7() {
        let (_df, ctx, expected) = Fixture::load(&format_path(
            "short_length_30_mid_length_15_long_length_7.csv",
        ));
        _test(
            &mut ctx.clone(),
            &mut UltimateOscillatorIndicator::new(
                ctx.clone(),
                UltimateOscillatorIndicatorConfig {
                    short_length: 30,
                    mid_length: 15,
                    long_length: 7,
                },
            ),
            &expected,
        );
    }
}
