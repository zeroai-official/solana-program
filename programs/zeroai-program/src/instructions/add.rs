use crate::state::*;
use crate::ErrorCode;
use anchor_lang::prelude::*;

pub fn add_game(ctx: Context<AddGame>, symbol: Vec<String>, price: Vec<u64>) -> Result<()> {
    let current_timestamp = Clock::get()?.unix_timestamp;

    let feeds = vec![
        &ctx.accounts.pyth_btc,
        &ctx.accounts.pyth_eth,
        &ctx.accounts.pyth_sol,
        &ctx.accounts.pyth_bnb,
        &ctx.accounts.pyth_doge,
        &ctx.accounts.pyth_jup,
    ];

    let mut close_price: Vec<u64> = Vec::new();

    for feed in feeds {
        let price = feed.get_price_no_older_than(current_timestamp, 60);
        match price {
            Some(price) => {
                let get_price = u64::try_from(price.price).unwrap()
                    / 10u64.pow(u32::try_from(-price.expo - 4).unwrap());
                close_price.push(get_price);
            }
            None => {
                close_price.push(0);
            }
        }
    }

    let games = &mut ctx.accounts.game;

    games.author = ctx.accounts.admin.key();
    games.timestamp = current_timestamp;
    games.symbol = symbol;
    games.targetprice = price;
    games.closeprice = close_price;
    games.count += 1;

    emit!(GameEvent {
        count: games.count,
        symbol: games.symbol.clone(),
        targetprice: games.targetprice.clone(),
        closeprice: games.closeprice.clone(),
        timestamp: games.timestamp,
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(symbol:String)]
pub struct AddGame<'info> {
    #[account(
        has_one = admin,
     )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        seeds = [b"games", admin.key().as_ref()],
        bump,
    )]
    pub game: Account<'info, InstantWinGame>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        address = vault.btc_feed @ ErrorCode::InvalidArgument
    )]
    pub pyth_btc: Account<'info, PriceFeed>,
    #[account(
        address = vault.eth_feed @ ErrorCode::InvalidArgument
    )]
    pub pyth_eth: Account<'info, PriceFeed>,
    #[account(
        address = vault.sol_feed @ ErrorCode::InvalidArgument
    )]
    pub pyth_sol: Account<'info, PriceFeed>,
    #[account(
        address = vault.bnb_feed @ ErrorCode::InvalidArgument
    )]
    pub pyth_bnb: Account<'info, PriceFeed>,
    #[account(
        address = vault.doge_feed @ ErrorCode::InvalidArgument
    )]
    pub pyth_doge: Account<'info, PriceFeed>,
    #[account(
        address = vault.jup_feed @ ErrorCode::InvalidArgument
    )]
    pub pyth_jup: Account<'info, PriceFeed>,
}
