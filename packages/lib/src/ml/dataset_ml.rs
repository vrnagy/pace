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

        let bp = bp_indicator.next();
        let bp_trade = bp_strategy.next(bp);
        let bp_feat =
            FeatureNamespace::new("bp", bp_fb.next(bp, bp_trade, &bp_strategy.config).to_box());
        combined.push(bp_feat.to_box());

        combined.push(asset_fb.next().to_box());
        composer.push(combined.to_box());
    }

    let mut df = composer.to_df();
    let mut file = std::fs::File::create(path).unwrap();

    CsvWriter::new(&mut file).finish(&mut df).unwrap();
}
