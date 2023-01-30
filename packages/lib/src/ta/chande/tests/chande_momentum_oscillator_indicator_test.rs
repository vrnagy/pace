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
            chande::{
                chande_kroll_stop_indicator::{
                    ChandeKrollStopIndicator, ChandeKrollStopIndicatorConfig,
                },
                chande_momentum_oscillator_indicator::{
                    ChandeMomentumOscillatorIndicator, ChandeMomentumOscillatorIndicatorConfig,
                },
            },
            moving_average::ma::MovingAverageKind,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut ChandeMomentumOscillatorIndicator,
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
    fn test_chande_momentum_oscillator_length_14_btc_1d() {
        let (_df, ctx, expected) =
            Fixture::load("ta/chande/tests/fixtures/cmo/indicator/btc_1d_length_14_close.csv");
        _test(
            &mut ctx.clone(),
            &mut ChandeMomentumOscillatorIndicator::new(
                ctx.clone(),
                ChandeMomentumOscillatorIndicatorConfig {
                    length: 14,
                    src: Source::from_kind(ctx, SourceKind::Close),
                },
            ),
            &expected,
        );
    }

    #[test]
    fn test_chande_momentum_oscillator_length_2_btc_1d() {
        let (_df, ctx, expected) =
            Fixture::load("ta/chande/tests/fixtures/cmo/indicator/btc_1d_length_2_close.csv");
        _test(
            &mut ctx.clone(),
            &mut ChandeMomentumOscillatorIndicator::new(
                ctx.clone(),
                ChandeMomentumOscillatorIndicatorConfig {
                    length: 2,
                    src: Source::from_kind(ctx, SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
