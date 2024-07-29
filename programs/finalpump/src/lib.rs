use anchor_lang::prelude::*;

pub mod utils;
pub mod instructs;
pub mod consts;
pub mod state;
pub mod errors;


use crate::instructs::*;

declare_id!("9yFGjEq3TEdgtQSzQaZQBg1xuCbj5hDshPxo1xy3E4d8");



#[program]
pub mod finalpump {
    use super::*;

    pub fn initialize(ctx: Context<InitialCurveConfiguration>, fee: f64) -> Result<()> {
        instructs::initialize(ctx, fee)
    }

    pub fn create_pool(ctx: Context<CreateLiquidityPool>) -> Result<()> {
         instructs::create_pool(ctx)
      }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
    ) -> Result<()> {
        instructs::add_liquidity(ctx)
    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, bump: u8) -> Result<()> {
        instructs::remove_liquidity(ctx, bump)
    }

    pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
        instructs::buy(ctx, amount)
    }

    pub fn sell(ctx: Context<Sell>, amount: u64, bump: u8) -> Result<()> {
        instructs::sell(ctx, amount, bump)
    }
}