use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

pub use crate::state::{Dao, Proposal, Vote};

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(
        init,
        payer = voter,
        space = 8 + Vote::INIT_SPACE,
        seeds = [
            b"vote",
            proposal.key().as_ref(),
            voter.key().as_ref()
        ],
        bump
    )]
    pub vote_account: Account<'info, Vote>,

    #[account(
        token::authority = voter,
    )]
    pub creator_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

impl CastVote<'_> {
    pub fn cast_vote(&mut self, vote_type: u8, bumps: &CastVoteBumps) -> Result<()> {
        let vote_account = &mut self.vote_account;
        // let proposal_account = &mut self.proposal;

        let vote_credits = (self.creator_token_account.amount as f64).sqrt() as u64;

        vote_account.set_inner(Vote {
            authority: self.voter.key(),
            vote_type,
            vote_credits,
            bump: bumps.vote_account,
        });
        Ok(())
    }
}
