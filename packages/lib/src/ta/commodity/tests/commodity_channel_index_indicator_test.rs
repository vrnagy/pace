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
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut CommodityChannelIndexIndicator,
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
    fn test_donchian_channels_14_length_btc_1d() {
        let (_df, ctx, expected) = Fixture::load(
            "ta/commodity/tests/fixtures/cci/indicator/btc_1d_length_14_hlc3_sma.csv",
        );
        _test(
            &mut ctx.clone(),
            &mut CommodityChannelIndexIndicator::new(
                ctx.clone(),
                CommodityChannelIndexIndicatorConfig {
                    length: 14,
                    src: Source::from_kind(ctx.clone(), SourceKind::HLC3),
                    ma_kind: MovingAverageKind::SMA,
                },
            ),
            &expected,
        );
    }
}
