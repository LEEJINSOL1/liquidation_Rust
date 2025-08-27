use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub liq_price: f64,
    pub size: f64,
    pub leverage: f64,
    pub wallet_balance_usd: f64,
}

pub fn calc_liq_risk(positions: &[Position], price: f64, bins: usize) -> (Vec<f64>, f64) {
    let start = price * 0.5;
    let end = price * 1.5;
    let step = (end - start) / bins as f64;
    let mut buckets = vec![0.0_f64; bins];
    for p in positions {
        let idx = ((p.liq_price - start) / step).floor() as isize;
        if idx >= 0 && (idx as usize) < bins {
            let weight = p.size.abs().min(p.wallet_balance_usd * p.leverage);
            buckets[idx as usize] += weight;
        }
    }
    let max = buckets.iter().cloned().fold(0.0, f64::max);
    let total: f64 = buckets.iter().sum();
    let index = if total == 0.0 { 0.0 } else { max / total * 100.0 };
    (buckets, index)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PnlAgg {
    pub win_rate: f64,
    pub sharpe: f64,
    pub funding_ratio: f64,
    pub avg_hold_secs: f64,
}

pub fn classify_trader(p: &PnlAgg) -> &'static str {
    if p.win_rate >= 0.55 && p.sharpe >= 0.8 {
        "Consistent"
    } else if p.funding_ratio >= 0.3 {
        "FundingFarmer"
    } else if p.avg_hold_secs < 600.0 && p.win_rate < 0.4 {
        "Gambler"
    } else {
        "Neutral"
    }
}

pub fn calc_spread(base: f64, reference: f64) -> f64 {
    (base - reference) / reference * 10_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liq_risk_sum_and_norm() {
        let positions = vec![
            Position { liq_price: 90.0, size: 10.0, leverage: 2.0, wallet_balance_usd: 100.0 },
            Position { liq_price: 110.0, size: 5.0, leverage: 3.0, wallet_balance_usd: 50.0 },
        ];
        let (buckets, idx) = calc_liq_risk(&positions, 100.0, 10);
        let sum: f64 = buckets.iter().sum();
        assert!(sum > 0.0);
        assert!(idx >= 0.0 && idx <= 100.0);
    }
}
