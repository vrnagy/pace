#[cfg(test)]
mod tests {
    use crate::base::{
        components::{
            component_context::ComponentContext,
            testing::{ComponentTestSnapshot, Fixture},
        },
        ta::{
            atr_component::AverageTrueRangeComponent, sma_component::SimpleMovingAverageComponent,
            sum_component::SumComponent,
        },
    };

    fn format_path(path: &str) -> String {
        format!("base/ta/tests/fixtures/sum/{}", path)
    }

    fn _test(cctx: &mut ComponentContext, target: &mut SumComponent, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let output = target.next(cctx.get().close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    fn _test_with_atr(
        cctx: &mut ComponentContext,
        target: &mut SumComponent,
        target_atr: &mut AverageTrueRangeComponent,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let atr = target_atr.next();
            let output = target.next(atr);
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn length_1_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_1_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut SumComponent::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn length_2_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_2_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut SumComponent::new(ctx.clone(), 2),
            &expected,
        );
    }

    #[test]
    fn length_3_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_3_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut SumComponent::new(ctx.clone(), 3),
            &expected,
        );
    }

    #[test]
    fn length_14_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_14_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut SumComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn length_365_close() {
        let (_df, ctx, expected) = Fixture::load(&format_path("length_365_close.csv"));
        _test(
            &mut ctx.clone(),
            &mut SumComponent::new(ctx.clone(), 365),
            &expected,
        );
    }

    #[test]
    fn length_14_with_atr_length_1() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("atr/length_14_with_atr_length_1.csv"));
        _test_with_atr(
            &mut ctx.clone(),
            &mut SumComponent::new(ctx.clone(), 14),
            &mut AverageTrueRangeComponent::new(ctx.clone(), 1),
            &expected,
        );
    }

    #[test]
    fn length_1_with_atr_length_14() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("atr/length_1_with_atr_length_14.csv"));
        _test_with_atr(
            &mut ctx.clone(),
            &mut SumComponent::new(ctx.clone(), 1),
            &mut AverageTrueRangeComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn length_14_with_atr_length_14() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("atr/length_14_with_atr_length_14.csv"));
        _test_with_atr(
            &mut ctx.clone(),
            &mut SumComponent::new(ctx.clone(), 14),
            &mut AverageTrueRangeComponent::new(ctx.clone(), 14),
            &expected,
        );
    }

    #[test]
    fn length_1_with_atr_length_1() {
        let (_df, ctx, expected) =
            Fixture::load(&format_path("atr/length_1_with_atr_length_1.csv"));
        _test_with_atr(
            &mut ctx.clone(),
            &mut SumComponent::new(ctx.clone(), 1),
            &mut AverageTrueRangeComponent::new(ctx.clone(), 1),
            &expected,
        );
    }
}
