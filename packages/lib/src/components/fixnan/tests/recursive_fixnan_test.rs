#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        asset::in_memory_asset_data_provider::InMemoryAssetDataProvider,
        components::{
            batch_validator::recursive_batch_validator::RecursiveBatchValidator,
            component_context::ComponentContext, execution_context::ExecutionContext,
            fixnan::recursive_fixnan::RecursiveFixNan, testing::ComponentTestSnapshot,
        },
    };

    fn _test(cctx: &mut ComponentContext, target: &mut RecursiveFixNan, expected: &[Option<f64>]) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let output = target.next(ctx.close());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_fixnan_all_non_nan() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Rc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut RecursiveFixNan::new(ctx.clone()),
            &[
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
            ],
        );
    }

    #[test]
    fn test_fixnan_all_nan() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Rc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                None, None, None, None, None, None, None, None,
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut RecursiveFixNan::new(ctx.clone()),
            &[None, None, None, None, None, None, None, None],
        );
    }

    #[test]
    fn test_fixnan_mixed() {
        let ctx = ComponentContext::build(ExecutionContext::from_asset(Rc::from(
            InMemoryAssetDataProvider::from_values(Vec::from([
                None,
                None,
                None,
                None,
                Some(1.0),
                Some(2.0),
                None,
                None,
                None,
                Some(3.0),
                None,
                Some(4.0),
                None,
                None,
                None,
                None,
                Some(5.0),
                Some(6.0),
                Some(7.0),
                None,
            ])),
        )));

        _test(
            &mut ctx.clone(),
            &mut RecursiveFixNan::new(ctx.clone()),
            &[
                None,
                None,
                None,
                None,
                Some(1.0),
                Some(2.0),
                Some(2.0),
                Some(2.0),
                Some(2.0),
                Some(3.0),
                Some(3.0),
                Some(4.0),
                Some(4.0),
                Some(4.0),
                Some(4.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(7.0),
            ],
        );
    }
}
