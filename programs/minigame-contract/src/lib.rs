use anchor_lang::prelude::*;

declare_id!("AhGizHCqVHXgmPtV7QTskda3EpP1XgCw2ECSKuQYRQkP");

#[program]
pub mod minigame_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
