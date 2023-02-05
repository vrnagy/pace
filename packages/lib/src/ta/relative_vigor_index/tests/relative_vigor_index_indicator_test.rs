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
            commodity::commodity_channel_index_indicator::{
                CommodityChannelIndexIndicator, CommodityChannelIndexIndicatorConfig,
            },
            donchian::donchian_channels_indicator::{
                DonchianChannelsIndicator, DonchianChannelsIndicatorConfig,
            },
            moving_average::ma::MovingAverageKind,
            relative_vigor_index::relative_vigor_index_indicator::{
                RelativeVigorIndexIndicator, RelativeVigorIndexIndicatorConfig,
            },
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RelativeVigorIndexIndicator,
        expected: &[Option<(Option<f64>, Option<f64>)>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<(Option<f64>, Option<f64>)>::new();
        for cctx in cctx {
            let output = target.next();
            snapshot.push(Some((output.rvi, output.sig)));
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_relative_vigor_index_14_length() {
        let (_df, ctx) = Fixture::raw(
            "ta/relative_vigor_index/tests/fixtures/rvi/indicator/btc_1d_length_14.csv",
        );
        let rvi_values = _df.column("_target_rvi_").unwrap().to_f64();
        let sig_values = _df.column("_target_sig_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>)>> = rvi_values
            .iter()
            .zip(sig_values.iter())
            .map(|(rvi, sig)| Some((*rvi, *sig)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut RelativeVigorIndexIndicator::new(
                ctx.clone(),
                RelativeVigorIndexIndicatorConfig { length: 14 },
            ),
            &expected,
        );
    }

    #[test]
    fn test_relative_vigor_index_1_length() {
        let (_df, ctx) = Fixture::raw(
            "ta/relative_vigor_index/tests/fixtures/rvi/indicator/btc_1d_length_1.csv",
        );
        let rvi_values = _df.column("_target_rvi_").unwrap().to_f64();
        let sig_values = _df.column("_target_sig_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>)>> = rvi_values
            .iter()
            .zip(sig_values.iter())
            .map(|(rvi, sig)| Some((*rvi, *sig)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut RelativeVigorIndexIndicator::new(
                ctx.clone(),
                RelativeVigorIndexIndicatorConfig { length: 1 },
            ),
            &expected,
        );
    }

    #[test]
    fn test_relative_vigor_index_2_length() {
        let (_df, ctx) = Fixture::raw(
            "ta/relative_vigor_index/tests/fixtures/rvi/indicator/btc_1d_length_2.csv",
        );
        let rvi_values = _df.column("_target_rvi_").unwrap().to_f64();
        let sig_values = _df.column("_target_sig_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>)>> = rvi_values
            .iter()
            .zip(sig_values.iter())
            .map(|(rvi, sig)| Some((*rvi, *sig)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut RelativeVigorIndexIndicator::new(
                ctx.clone(),
                RelativeVigorIndexIndicatorConfig { length: 2 },
            ),
            &expected,
        );
    }

    #[test]
    fn test_relative_vigor_index_3_length() {
        let (_df, ctx) = Fixture::raw(
            "ta/relative_vigor_index/tests/fixtures/rvi/indicator/btc_1d_length_3.csv",
        );
        let rvi_values = _df.column("_target_rvi_").unwrap().to_f64();
        let sig_values = _df.column("_target_sig_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>)>> = rvi_values
            .iter()
            .zip(sig_values.iter())
            .map(|(rvi, sig)| Some((*rvi, *sig)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut RelativeVigorIndexIndicator::new(
                ctx.clone(),
                RelativeVigorIndexIndicatorConfig { length: 3 },
            ),
            &expected,
        );
    }

    #[test]
    fn test_relative_vigor_index_365_length() {
        let (_df, ctx) = Fixture::raw(
            "ta/relative_vigor_index/tests/fixtures/rvi/indicator/btc_1d_length_365.csv",
        );
        let rvi_values = _df.column("_target_rvi_").unwrap().to_f64();
        let sig_values = _df.column("_target_sig_").unwrap().to_f64();
        let expected: Vec<Option<(Option<f64>, Option<f64>)>> = rvi_values
            .iter()
            .zip(sig_values.iter())
            .map(|(rvi, sig)| Some((*rvi, *sig)))
            .collect();
        _test(
            &mut ctx.clone(),
            &mut RelativeVigorIndexIndicator::new(
                ctx.clone(),
                RelativeVigorIndexIndicatorConfig { length: 365 },
            ),
            &expected,
        );
    }
}
