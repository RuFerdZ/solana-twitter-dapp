use anchor_lang::prelude::*;

declare_id!("AdGdR4tVHcxa36vEvPGh9QHsAMLAD8861puJzr6mntJj");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
