use anchor_lang::prelude::*;

declare_id!("6sntG2TygfQFj1s9Sh4MpqPf4SWFGiuipE867tzt4wez");

#[program]
pub mod sol_stable {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
