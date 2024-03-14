use anchor_lang::prelude::*;
use pyth_sdk_solana::state::load_price_account;
use pyth_sdk_solana::state::SolanaPriceAccount;
use std::ops::Deref;
use std::str::FromStr;
use crate::ErrorCode;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub admin: Pubkey,
    pub btc_feed: Pubkey,
    pub eth_feed: Pubkey,
    pub sol_feed: Pubkey,
    pub bnb_feed: Pubkey,
    pub doge_feed: Pubkey,
    pub jup_feed: Pubkey,
}

#[account]
pub struct InstantWinGame{
    pub author: Pubkey,
    pub count: u64,
    pub symbol: Vec<String>,
    pub targetprice: Vec<u64>,
    pub closeprice: Vec<u64>,
    pub timestamp: i64,
}

#[event]
pub struct GameEvent{
    pub count: u64,
    pub symbol: Vec<String>,
    pub targetprice: Vec<u64>,
    pub closeprice: Vec<u64>,
    pub timestamp: i64,
}

#[derive(Clone)]
pub struct PriceFeed(pyth_sdk::PriceFeed);

impl anchor_lang::Owner for PriceFeed {
    fn owner() -> Pubkey {
        // Make sure the owner is the pyth oracle account on solana devnet
        let oracle_addr = "gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s";
        Pubkey::from_str(&oracle_addr).unwrap()
    }
}

impl anchor_lang::AccountDeserialize for PriceFeed {
    fn try_deserialize_unchecked(data: &mut &[u8]) -> Result<Self> {

        let account: &SolanaPriceAccount =
            load_price_account(data).map_err(|_x| error!(ErrorCode::PythError))?;

        // Use a dummy key since the key field will be removed from the SDK
        let zeros: [u8; 32] = [0; 32];
        let dummy_key = Pubkey::from(*&zeros);
        let feed = account.to_price_feed(&dummy_key);
        Ok(PriceFeed(feed))
    }
}

impl anchor_lang::AccountSerialize for PriceFeed {
    fn try_serialize<W: std::io::Write>(&self, _writer: &mut W) -> std::result::Result<(), Error> {
        Err(error!(ErrorCode::TryToSerializePriceAccount))
    }
}

impl Deref for PriceFeed {
    type Target = pyth_sdk::PriceFeed;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}