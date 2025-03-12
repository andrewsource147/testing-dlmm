use anchor_lang::prelude::*;

declare_id!("3m7k42SNzdf3iQfk3hMDvmiB7UmtJFbFFi23vYTU7ncn");

#[program]
pub mod testing_dlmm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // call lb_clmm instance here
        // lb_clmm::add_liquidity(ctx, liquidity_parameter)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
