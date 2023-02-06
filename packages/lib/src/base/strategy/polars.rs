use std::{path::Path, time::Duration};

use polars::{
    prelude::{CsvReader, DataFrame, DataType, IsFloat, ParquetReader, SerReader, TimeUnit},
    series::Series,
};

use crate::utils::polars::SeriesCastUtils;

use super::trade::{trade_direction_from_f64, TradeDirection};

pub trait SeriesCastUtilsForStrategy {
    fn to_trade(&self) -> Vec<Option<TradeDirection>>;
}

impl SeriesCastUtilsForStrategy for Series {
    fn to_trade(&self) -> Vec<Option<TradeDirection>> {
        return self
            .to_f64()
            .into_iter()
            .map(|value| {
                value?;
                return trade_direction_from_f64(value);
            })
            .collect::<Vec<_>>();
    }
}
