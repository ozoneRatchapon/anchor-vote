use crate::state::Dao;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitDao<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = 8 + Dao::INIT_SPACE,
        seeds = [b"dao", creator.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub dao_account: Account<'info, Dao>,
    pub system_program: Program<'info, System>,
}

impl InitDao<'_> {
    pub fn init_dao(&mut self, name: String, bumps: &InitDaoBumps) -> Result<()> {
        self.dao_account.set_inner(Dao {
            name,
            authority: self.creator.key(),
            proposal_count: 0,
            bump: bumps.dao_account,
        });
        Ok(())
    }
}
