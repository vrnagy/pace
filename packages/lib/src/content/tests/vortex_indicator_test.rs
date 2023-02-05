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
            volume_oscillator_indicator::{
                VolumeOscillatorIndicator, VolumeOscillatorIndicatorConfig,
            },
            vortex_indicator::{VortexIndicator, VortexIndicatorConfig},
        },
        utils::polars::DataFrameUtils,
    };

    fn format_path(path: &str) -> String {
        format!("content/tests/fixtures/vortex/indicator/{}", path)
    }

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut VortexIndicator,
        expected: &[Option<(Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Option<f64>, Option<f64>)>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(Some((output.plus, output.minus)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_14() {
        let (_df, ctx) = Fixture::raw(&format_path("length_14.csv"));
        let expected = _df.merge_two_columns("_target_plus_", "_target_minus_");
        _test(
            &mut ctx.clone(),
            &mut VortexIndicator::new(ctx.clone(), VortexIndicatorConfig { length: 14 }),
            &expected,
        );
    }

    #[test]
    fn length_2() {
        let (_df, ctx) = Fixture::raw(&format_path("length_2.csv"));
        let expected = _df.merge_two_columns("_target_plus_", "_target_minus_");
        _test(
            &mut ctx.clone(),
            &mut VortexIndicator::new(ctx.clone(), VortexIndicatorConfig { length: 2 }),
            &expected,
        );
    }
}
