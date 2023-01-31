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
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut PriceOscillatorIndicator,
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
    fn test_price_oscillator_oscillator_short_length_10_long_length_21_sma_close() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/price_oscillator/tests/fixtures/indicator/btc_1d_short_length_10_long_length_21_sma_close.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut PriceOscillatorIndicator::new(
                ctx.clone(),
                PriceOscillatorIndicatorConfig {
                    long_length: 21,
                    short_length: 10,
                    long_ma_type: MovingAverageKind::SMA,
                    short_ma_type: MovingAverageKind::SMA,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
