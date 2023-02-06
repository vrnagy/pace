use crate::base::ta::cross::CrossMode;

#[derive(Debug, PartialEq, Clone)]
pub enum TradeDirection {
    Long,
    Short,
}

pub fn trade_direction_to_f64(direction: Option<TradeDirection>) -> f64 {
    return match direction {
        Some(TradeDirection::Long) => 1.0,
        Some(TradeDirection::Short) => -1.0,
        None => 0.0,
    };
}

pub fn trade_direction_from_f64(value: Option<f64>) -> Option<TradeDirection> {
    return match value {
        Some(value) => {
            if value == 1.0 {
                return Some(TradeDirection::Long);
            }
            if value == -1.0 {
                return Some(TradeDirection::Short);
            }
            return None;
        }
        None => None,
    };
}

// impl StrategyActionKind {
//     pub fn to_f64(&self) -> f64 {
//         return match self {
//             StrategyActionKind::None => 0.0,
//             StrategyActionKind::Long => 1.0,
//             StrategyActionKind::Short => -1.0,
//         };
//     }
// }
