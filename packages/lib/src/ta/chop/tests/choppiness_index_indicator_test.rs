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
            balance_of_power::balance_of_power_indicator::BalanceOfPowerIndicator,
            chaikin::chaikin_money_flow_indicator::{
                ChaikinMoneyFlowIndicator, ChaikinMoneyFlowIndicatorConfig,
            },
            chop::choppiness_index_indicator::{
                ChoppinessIndexIndicator, ChoppinessIndexIndicatorConfig,
            },
            moving_average::ma::MovingAverageKind,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut ChoppinessIndexIndicator,
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
    fn test_choppiness_index_14_length_btc_1d() {
        let (_df, ctx, expected) =
            Fixture::load("ta/chop/tests/fixtures/choppiness_index/indicator/btc_1d_14_length.csv");
        _test(
            &mut ctx.clone(),
            &mut ChoppinessIndexIndicator::new(
                ctx.clone(),
                ChoppinessIndexIndicatorConfig { length: 14 },
            ),
            &expected,
        );
    }

    #[test]
    fn test_choppiness_index_2_length_btc_1d() {
        let (_df, ctx, expected) =
            Fixture::load("ta/chop/tests/fixtures/choppiness_index/indicator/btc_1d_2_length.csv");
        _test(
            &mut ctx.clone(),
            &mut ChoppinessIndexIndicator::new(
                ctx.clone(),
                ChoppinessIndexIndicatorConfig { length: 2 },
            ),
            &expected,
        );
    }
}
