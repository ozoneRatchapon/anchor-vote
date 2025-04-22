use crate::state::{Dao, Proposal};
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct InitProposal<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    pub dao_account: Account<'info, Dao>,
    #[account(
        init,
        payer = creator,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [b"proposal", dao_account.key().as_ref(), dao_account.proposal_count.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    pub creator_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

impl InitProposal<'_> {
    pub fn init_proposal(&mut self, name: String, bumps: &InitProposalBumps) -> Result<()> {
        let proposal = &mut self.proposal;
        let dao_account = &mut self.dao_account;

        dao_account.proposal_count += 1;
        // Token transfer to collect 0.1 SOL
        proposal.set_inner(Proposal {
            authority: self.creator.key(),
            metadata: name,
            yes_vote_count: 0u64,
            no_vote_count: 0u64,
            bump: bumps.proposal,
        });
        Ok(())
    }
}
