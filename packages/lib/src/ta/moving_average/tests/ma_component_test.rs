#[cfg(test)]
mod tests {
    use crate::{
        components::{
            component_context::ComponentContext,
            source::{Source, SourceKind},
            testing::ComponentTestSnapshot,
        },
        ta::moving_average::{
            ma::MovingAverageKind, ma_component::MovingAverageComponent,
            rma_component::RunningMovingAverageComponent,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut MovingAverageComponent,
        source: Source,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next(source.get());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_btc_1d_length_34_hl2() {
        let (_df, ctx, expected) =
            Fixture::load("ta/moving_average/tests/fixtures/sma/btc_1d_length_34_hl2.csv");
        _test(
            &mut ctx.clone(),
            &mut MovingAverageComponent::new(ctx.clone(), 34, MovingAverageKind::SMA),
            Source::from_kind(ctx.clone(), SourceKind::HL2),
            &expected,
        );
    }
}
