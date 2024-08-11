use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};


use crate::state::Escrow;

/// Remember to add first the instruction parameters exposed in the structs to avoid issues with anchor
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    /// The user started the escrow process
    #[account(mut)]
    pub maker: Signer<'info>,

    /// Using token:interface::Mint will support SPL tokens and SPL2022 tokes
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    /// property to define the mint address for the receiver token
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    /// the token associated account who will send the tokens stored in the vault to the user
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    /// The rule seetings of the escrow 
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [
            b"escrow", 
            maker.key().as_ref(), 
            seed.to_le_bytes().as_ref()
            ],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// Account to store the maker tokens (it's an ata). 
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// WE will need the program to create and use associated token accounts
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// We will also need the program to mint, transfer tokens
    pub token_program: Interface<'info, TokenInterface>,

    /// And we will need the system program to create escrow state account
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    /// Method to start the escrow process. The maker will initialize the Escrow account state 
    pub fn save_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive,
            bump: bumps.escrow,
        });
        Ok(())
    }


    /// From the maker ATA where the tokers are hosted, will be sent the vault  
    pub fn deposit_to_vault(&mut self, amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        // as we are using a different program (token2022) we will have to create a CPI
        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)
    }
}
