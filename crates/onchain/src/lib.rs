use anyhow::Result;
use ethers::prelude::*;

pub const STABLE_WHITELIST: [(&str, &str); 2] = [
    ("USDT", "0xdAC17F958D2ee523a2206206994597C13D831ec7"),
    ("USDC", "0xA0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
];

pub async fn get_erc20_balance(
    provider: &Provider<Http>,
    token_addr: Address,
    owner: Address,
) -> Result<U256> {
    let erc20 = ethers::contract::abigen!(ERC20, "[function balanceOf(address) view returns (uint256)]");
    let contract = ERC20::new(token_addr, provider.clone());
    let balance = contract.balance_of(owner).call().await?;
    Ok(balance)
}

/// TODO: 실제 Transfer 이벤트 스캔 구현
pub async fn get_recent_transfers(_provider: &Provider<Http>, _token: Address) -> Result<Vec<()>> {
    Ok(vec![])
}
