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
            donchian::donchian_channels_indicator::{
                DonchianChannelsIndicator, DonchianChannelsIndicatorConfig,
            },
            moving_average::ma::MovingAverageKind,
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut DonchianChannelsIndicator,
        expected: &[Option<(Option<f64>, Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Option<f64>, Option<f64>, Option<f64>)>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(Some((output.upper, output.basis, output.lower)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_donchian_channels_14_length_btc_1d() {
        let (_df, ctx) = Fixture::raw(
            "ta/donchian/tests/fixtures/donchian_channels/indicator/btc_1d_14_length.csv",
        );
        let upper_values = _df.column("_target_upper_").unwrap().to_f64();
        let basis_values = _df.column("_target_basis_").unwrap().to_f64();
        let lower_values = _df.column("_target_lower_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>, Option<f64>)>> = upper_values
            .iter()
            .zip(basis_values.iter())
            .zip(lower_values.iter())
            .map(|((upper, basis), down)| Some((*upper, *basis, *down)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut DonchianChannelsIndicator::new(
                ctx.clone(),
                DonchianChannelsIndicatorConfig { length: 14 },
            ),
            &expected,
        );
    }
}
