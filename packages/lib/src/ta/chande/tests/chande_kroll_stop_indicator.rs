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
            chande::chande_kroll_stop_indicator::{
                ChandeKrollStopIndicator, ChandeKrollStopIndicatorConfig,
            },
            moving_average::ma::MovingAverageKind,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut ChandeKrollStopIndicator,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot =
            ComponentTestSnapshot::<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(Some((
                output.first_high_stop,
                output.first_low_stop,
                output.stop_short,
                output.stop_long,
            )));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_chande_kroll_stop_10_length_btc_1d() {
        let (_df, ctx) =
            Fixture::raw("ta/chande/tests/fixtures/cks/indicator/btc_1d_p10_x1_q9.csv");
        let first_high_stop_values = _df.column("_target_first_high_stop_").unwrap().to_f64();
        let first_low_stop = _df.column("_target_first_low_stop_").unwrap().to_f64();
        let stop_short = _df.column("_target_stop_short_").unwrap().to_f64();
        let stop_long = _df.column("_target_stop_long_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>, Option<f64>, Option<f64>)>> =
            first_high_stop_values
                .iter()
                .zip(first_low_stop.iter())
                .zip(stop_short.iter())
                .zip(stop_long.iter())
                .map(
                    |(((first_high_stop, first_low_stop), stop_short), stop_long)| {
                        Some((*first_high_stop, *first_low_stop, *stop_short, *stop_long))
                    },
                )
                .collect();
        _test(
            &mut ctx.clone(),
            &mut ChandeKrollStopIndicator::new(
                ctx.clone(),
                ChandeKrollStopIndicatorConfig {
                    p: 10,
                    x: 1.0,
                    q: 9,
                },
            ),
            &expected,
        );
    }
}
