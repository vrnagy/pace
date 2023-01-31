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
            relative_volatility_index::rvi_indicator::{
                RelativeVolatilityIndexIndicator, RelativeVolatilityIndexIndicatorConfig,
            },
        },
        testing::fixture::Fixture,
    };

    fn _test(
        cctx: &mut ComponentContext,
        target: &mut RelativeVolatilityIndexIndicator,
        expected: &[Option<f64>],
    ) {
        let mut snapshot = ComponentTestSnapshot::<f64>::new();
        for cctx in cctx {
            let ctx = cctx.get();
            let tick = ctx.current_tick;
            let output = target.next();
            // We need to omit first 250 bars, because of ta.change and NaNs
            if tick < 250 {
                snapshot.push(expected[tick]);
            } else {
                snapshot.push(output.value);
            }
        }
        snapshot.assert(expected);
    }

    #[test]
    fn test_relative_volatility_index_btc_1d_length_14_ma_14_ema_close() {
        let (_df, ctx, expected) = Fixture::load_with_target(
            "ta/relative_volatility_index/tests/fixtures/rvi/indicator/btc_1d_length_14_ma_14_ema_close.csv",
            "_target_",
        );
        _test(
            &mut ctx.clone(),
            &mut RelativeVolatilityIndexIndicator::new(
                ctx.clone(),
                RelativeVolatilityIndexIndicatorConfig {
                    length: 14,
                    ma_length: 14,
                    src: Source::from_kind(ctx.clone(), SourceKind::Close),
                },
            ),
            &expected,
        );
    }
}
