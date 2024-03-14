use crate::state::*;
use anchor_lang::prelude::*;

pub fn init_vault(ctx: Context<InitVault>, config: Vault) -> Result<()> {
    ctx.accounts.vault.set_inner(config);
    Ok(())
}

pub fn init_games(ctx: Context<InitGames>) -> Result<()> {
    let games = &mut ctx.accounts.games;

    let clock: Clock = Clock::get().unwrap();

    games.author = ctx.accounts.admin.key();
    games.timestamp = clock.unix_timestamp;
    games.symbol = ["ZEROAI Instant Win Games".to_string()].to_vec();
    games.closeprice = vec![0; 6];
    games.targetprice = vec![0; 6];
    games.count = 0;

    emit!(GameEvent {
        count: games.count,
        symbol: games.symbol.clone(),
        targetprice: games.targetprice.clone(),
        closeprice: games.closeprice.clone(),
        timestamp: games.timestamp,
    });

    msg!(
        "{:?} has been successfully initialized, created by {} on {}.",
        games.symbol,
        games.author,
        games.timestamp
    );
    Ok(())
}

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + Vault::INIT_SPACE,
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitGames<'info> {
    #[account(
        has_one = admin,
     )]
    pub vault: Account<'info, Vault>,
    #[account(
        init,
        seeds = [b"games", admin.key().as_ref()],
        bump,
        payer = admin,
        space = 8 + 32 + 8 + std::mem::size_of::<Vec<String>>() + 6 * std::mem::size_of::<String>() + 6 * 4 + std::mem::size_of::<Vec<u64>>() + 6 * 8 + std::mem::size_of::<Vec<u64>>()+ 6 * 8 + 8
    )]
    pub games: Account<'info, InstantWinGame>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
