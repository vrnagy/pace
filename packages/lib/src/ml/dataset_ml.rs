use std::path::{Path, PathBuf};

use polars::prelude::*;

use crate::{
    base::{
        asset::{
            asset_feature_builder::AssetFeatureBuilder,
            source::{Source, SourceKind},
        },
        components::{component_context::ComponentContext, component_default::ComponentDefault},
        features::{
            feature::{CombinedFeatures, Feature, FeatureNamespace, RawFeature},
            feature_composer::FeatureComposer,
        },
    },
    content::{
        aroon_feature_builder::AroonFeatureBuilder,
        aroon_indicator::{AroonIndicator, AroonIndicatorConfig},
        aroon_strategy::AroonStrategy,
        awesome_oscillator_feature_builder::{
            AwesomeOscillatorFeature, AwesomeOscillatorFeatureBuilder,
        },
        awesome_oscillator_indicator::{
            AwesomeOscillatorIndicator, AwesomeOscillatorIndicatorConfig,
        },
        awesome_oscillator_strategy::{AwesomeOscillatorStrategy, AwesomeOscillatorStrategyConfig},
        balance_of_power_feature_builder::BalanceOfPowerFeatureBuilder,
        balance_of_power_indicator::BalanceOfPowerIndicator,
        balance_of_power_strategy::{BalanceOfPowerStrategy, BalanceOfPowerStrategyConfig},
        bollinger_bands_pb_feature_builder::BollingerBandsPercentBFeatureBuilder,
        bollinger_bands_pb_indicator::{
            BollingerBandsPercentBIndicator, BollingerBandsPercentBIndicatorConfig,
        },
        bollinger_bands_pb_strategy::{
            BollingerBandsPercentBStrategy, BollingerBandsPercentBStrategyConfig,
        },
        bollinger_bands_width_feature_builder::BollingerBandsWidthFeatureBuilder,
        bollinger_bands_width_indicator::{
            BollingerBandsWidthIndicator, BollingerBandsWidthIndicatorConfig,
        },
        chaikin_money_flow_feature_builder::ChaikinMoneyFlowFeatureBuilder,
        chaikin_money_flow_indicator::{
            ChaikinMoneyFlowIndicator, ChaikinMoneyFlowIndicatorConfig,
        },
        chaikin_money_flow_strategy::{ChaikinMoneyFlowStrategy, ChaikinMoneyFlowStrategyConfig},
        chande_momentum_oscillator_feature_builder::ChandeMomentumOscillatorFeatureBuilder,
        chande_momentum_oscillator_indicator::{
            ChandeMomentumOscillatorIndicator, ChandeMomentumOscillatorIndicatorConfig,
        },
        chande_momentum_oscillator_strategy::{
            ChandeMomentumOscillatorStrategy, ChandeMomentumOscillatorStrategyConfig,
        },
        choppiness_index_feature_builder::ChoppinessIndexFeatureBuilder,
        choppiness_index_indicator::{ChoppinessIndexIndicator, ChoppinessIndexIndicatorConfig},
        relative_strength_index_feature_builder::RelativeStrengthIndexFeatureBuilder,
        relative_strength_index_indicator::{
            RelativeStrengthIndexIndicator, RelativeStrengthIndexIndicatorConfig,
        },
        relative_strength_index_strategy::{
            RelativeStrengthIndexStrategy, RelativeStrengthIndexStrategyConfig,
        },
    },
};

pub fn generate_ml_dataset(ctx: ComponentContext, path: &Path) {
    let mut composer = FeatureComposer::new();
    let mut asset_fb = AssetFeatureBuilder::new(ctx.clone());

    let mut rsi_indicator = RelativeStrengthIndexIndicator::new(
        ctx.clone(),
        RelativeStrengthIndexIndicatorConfig::default(ctx.clone()),
    );
    let mut rsi_strategy = RelativeStrengthIndexStrategy::new(
        ctx.clone(),
        RelativeStrengthIndexStrategyConfig::default(ctx.clone()),
    );
    let mut rsi_fb = RelativeStrengthIndexFeatureBuilder::new(ctx.clone());

    let mut aroon_indicator =
        AroonIndicator::new(ctx.clone(), AroonIndicatorConfig::default(ctx.clone()));
    let mut aroon_strategy = AroonStrategy::new(ctx.clone());
    let mut aroon_fb = AroonFeatureBuilder::new(ctx.clone());

    let mut ao_indicator = AwesomeOscillatorIndicator::new(
        ctx.clone(),
        AwesomeOscillatorIndicatorConfig::default(ctx.clone()),
    );
    let mut ao_strategy = AwesomeOscillatorStrategy::new(
        ctx.clone(),
        AwesomeOscillatorStrategyConfig::default(ctx.clone()),
    );
    let mut ao_fb = AwesomeOscillatorFeatureBuilder::new(ctx.clone());

    let mut bp_indicator = BalanceOfPowerIndicator::new(ctx.clone());
    let mut bp_strategy = BalanceOfPowerStrategy::new(
        ctx.clone(),
        BalanceOfPowerStrategyConfig::default(ctx.clone()),
    );
    let mut bp_fb = BalanceOfPowerFeatureBuilder::new(ctx.clone());

    let mut bbpb_indicator = BollingerBandsPercentBIndicator::new(
        ctx.clone(),
        BollingerBandsPercentBIndicatorConfig::default(ctx.clone()),
    );
    let mut bbpb_strategy = BollingerBandsPercentBStrategy::new(
        ctx.clone(),
        BollingerBandsPercentBStrategyConfig::default(ctx.clone()),
    );
    let mut bbpb_fb = BollingerBandsPercentBFeatureBuilder::new(ctx.clone());

    let mut bbw_indicator = BollingerBandsWidthIndicator::new(
        ctx.clone(),
        BollingerBandsWidthIndicatorConfig::default(ctx.clone()),
    );
    let mut bbw_fb = BollingerBandsWidthFeatureBuilder::new(ctx.clone());

    let mut cmf_indicator = ChaikinMoneyFlowIndicator::new(
        ctx.clone(),
        ChaikinMoneyFlowIndicatorConfig::default(ctx.clone()),
    );
    let mut cmf_strategy = ChaikinMoneyFlowStrategy::new(
        ctx.clone(),
        ChaikinMoneyFlowStrategyConfig::default(ctx.clone()),
    );
    let mut cmf_fb = ChaikinMoneyFlowFeatureBuilder::new(ctx.clone());

    let mut cmo_indicator = ChandeMomentumOscillatorIndicator::new(
        ctx.clone(),
        ChandeMomentumOscillatorIndicatorConfig::default(ctx.clone()),
    );
    let mut cmo_strategy = ChandeMomentumOscillatorStrategy::new(
        ctx.clone(),
        ChandeMomentumOscillatorStrategyConfig::default(ctx.clone()),
    );
    let mut cmo_fb = ChandeMomentumOscillatorFeatureBuilder::new(ctx.clone());

    let mut ci_indicator = ChoppinessIndexIndicator::new(
        ctx.clone(),
        ChoppinessIndexIndicatorConfig::default(ctx.clone()),
    );
    let mut ci_fb = ChoppinessIndexFeatureBuilder::new(ctx.clone());

    for cctx in ctx {
        let ctx = cctx.get();

        let mut combined = CombinedFeatures::new();

        // let rsi = rsi_indicator.next();
        // let rsi_trade = rsi_strategy.next(rsi);
        // let rsi_feat = FeatureNamespace::new(
        //     "rsi",
        //     rsi_fb
        //         .next(
        //             rsi,
        //             rsi_indicator.metadata(),
        //             rsi_trade,
        //             &rsi_strategy.config,
        //         )
        //         .to_box(),
        // );
        // combined.push(rsi_feat.to_box());

        // let aroon = aroon_indicator.next();
        // let aroon_trade = aroon_strategy.next(&aroon);
        // let aroon_feat = FeatureNamespace::new(
        //     "aroon",
        //     aroon_fb
        //         .next(&aroon, aroon_strategy.metadata(), aroon_trade)
        //         .to_box(),
        // );
        // combined.push(aroon_feat.to_box());

        // let ao = ao_indicator.next();
        // let ao_trade = ao_strategy.next(ao);
        // let ao_feat =
        //     FeatureNamespace::new("ao", ao_fb.next(ao, ao_trade, &ao_strategy.config).to_box());
        // combined.push(ao_feat.to_box());

        // let bp = bp_indicator.next();
        // let bp_trade = bp_strategy.next(bp);
        // let bp_feat =
        //     FeatureNamespace::new("bp", bp_fb.next(bp, bp_trade, &bp_strategy.config).to_box());
        // combined.push(bp_feat.to_box());

        // let bbpb = bbpb_indicator.next();
        // let bbpb_trade = bbpb_strategy.next(bbpb);
        // let bbpb_feat = FeatureNamespace::new(
        //     "bbpb",
        //     bbpb_fb
        //         .next(bbpb, bbpb_trade, &bbpb_strategy.config)
        //         .to_box(),
        // );
        // combined.push(bbpb_feat.to_box());

        // let bbw = bbw_indicator.next();
        // let bbw_feat = FeatureNamespace::new("bbw", bbw_fb.next(bbw).to_box());
        // combined.push(bbw_feat.to_box());

        // let cmf = cmf_indicator.next();
        // let cmf_trade = cmf_strategy.next(cmf);
        // let cmf_feat = FeatureNamespace::new(
        //     "cmf",
        //     cmf_fb.next(cmf, cmf_trade, &cmf_strategy.config).to_box(),
        // );
        // combined.push(cmf_feat.to_box());

        // let cmo = cmo_indicator.next();
        // let cmo_trade = cmo_strategy.next(cmo);
        // let cmo_feat = FeatureNamespace::new(
        //     "cmo",
        //     cmo_fb.next(cmo, cmo_trade, &cmo_strategy.config).to_box(),
        // );
        // combined.push(cmo_feat.to_box());

        let ci = ci_indicator.next();
        let ci_feat = FeatureNamespace::new("ci", ci_fb.next(ci).to_box());
        combined.push(ci_feat.to_box());

        combined.push(asset_fb.next().to_box());
        composer.push(combined.to_box());
    }

    let mut df = composer.to_df();
    let mut file = std::fs::File::create(path).unwrap();

    CsvWriter::new(&mut file).finish(&mut df).unwrap();
}
