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
            coppock_curve::coppock_curve_indicator::{
                CoppockCurveIndicator, CoppockCurveIndicatorConfig,
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
        target: &mut CoppockCurveIndicator,
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
    fn test_coppock_curve_indicator() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/coppock_curve/tests/fixtures/cc/indicator/btc_1d_wma_length_14_long_roc_length_14_short_roc_length_11_close.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut CoppockCurveIndicator::new(
                ctx.clone(),
                CoppockCurveIndicatorConfig {
                    ma_length: 10,
                    long_roc_length: 14,
                    short_roc_length: 11,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
