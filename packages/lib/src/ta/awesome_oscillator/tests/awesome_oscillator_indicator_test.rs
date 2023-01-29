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
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut AwesomeOscillatorIndicator,
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
    fn test_awesome_oscillator_btc_1d_short_length_5_long_length_34_hl2() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/awesome_oscillator/tests/fixtures/indicator/btc_1d_short_length_5_long_length_34_hl2.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut AwesomeOscillatorIndicator::new(
                ctx.clone(),
                AwesomeOscillatorIndicatorConfig {
                    long_length: 34,
                    short_length: 5,
                    long_ma_type: MovingAverageKind::SMA,
                    short_ma_type: MovingAverageKind::SMA,
                    long_source: Source::from_kind(ctx.clone(), SourceKind::HL2),
                    short_source: Source::from_kind(ctx.clone(), SourceKind::HL2),
                },
            ),
            &expected,
        );
    }
}
