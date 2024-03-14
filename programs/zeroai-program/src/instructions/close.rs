use anchor_lang::prelude::*;
use crate::state::*;

pub fn close_game(_ctx: Context<CloseGame>) -> Result<()> {
    msg!("Game has been closed.");
    emit!(GameEvent {
        count: 0,
        symbol: ["Hasta la vista, baby!".to_string()].to_vec(),
        targetprice: vec![0;6],
        closeprice: vec![0;6],
        timestamp: 0,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct CloseGame<'info> {
    #[account(
        has_one = admin,
     )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        seeds = [b"games", admin.key().as_ref()],
        bump,
        close=admin
    )]
    pub game: Account<'info, InstantWinGame>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>
}