#[cfg(test)]
mod tests {
    use crate::{
        base::{
            asset::source::{Source, SourceKind},
            components::{
                component_context::ComponentContext,
                testing::{ComponentTestSnapshot, Fixture},
            },
            ta::ma::MovingAverageKind,
        },
        content::{
            price_oscillator_indicator::{
                PriceOscillatorIndicator, PriceOscillatorIndicatorConfig,
            },
            williams_percent_range_indicator::{
                WilliamsPercentRangeIndicator, WilliamsPercentRangeIndicatorConfig,
            },
        },
    };

    fn format_path(path: &str) -> String {
        format!(
            "content/tests/fixtures/williams_percent_range/indicator/{}",
            path
        )
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut WilliamsPercentRangeIndicator,
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
    fn length_14_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut WilliamsPercentRangeIndicator::new(
                ctx.clone(),
                WilliamsPercentRangeIndicatorConfig {
                    length: 14,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_1_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_1_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut WilliamsPercentRangeIndicator::new(
                ctx.clone(),
                WilliamsPercentRangeIndicatorConfig {
                    length: 1,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }

    #[test]
    fn length_2_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut WilliamsPercentRangeIndicator::new(
                ctx.clone(),
                WilliamsPercentRangeIndicatorConfig {
                    length: 2,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
