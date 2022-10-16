use anchor_lang::{
    prelude::*,
    solana_program::{
        program::invoke,
        sysvar::rent::Rent,
        //instruction::{Instruction, AccountMeta},
        pubkey::Pubkey        
    },
};
use anchor_spl::{
    associated_token::{self, Create, AssociatedToken},
    token::{self, MintTo, Token},
};

use mpl_token_metadata::{
    instruction::{create_master_edition_v3, create_metadata_accounts_v2},
    state::{Collection, Uses, Creator},
};

declare_id!("H8frD7cBmWxwQzfyCXy53uvkjfUt1fgWKSkoH5Zvgrop");

#[program]
pub mod contract_nft {

    use super::*;

    pub fn create_metadata_token(
        ctx: Context<MetadataDataAccount>,
        name: String,
        symbol: String,
        uri: String,
        seller_fee_basis_points: u16,
        is_mutable: bool, 
    ) -> ProgramResult{
        
        if ctx.accounts.signer_token_account.data_is_empty() {
            let cpi_accounts = Create{
                payer: ctx.accounts.payer.clone(),
                associated_token: ctx.accounts.signer_token_account.clone(),
                authority: ctx.accounts.payer.clone(),
                rent: ctx.accounts.rent.to_account_info(),
                mint: ctx.accounts.mint.clone(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            };
            let cpi_program = ctx.accounts.associated_token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            associated_token::create(cpi_ctx)?;
        };

        let cpi_mint_to_account = MintTo{
            mint: ctx.accounts.mint.clone(),
            to: ctx.accounts.signer_token_account.clone(),
            authority: ctx.accounts.payer.clone(),
        };
        let cpi_mint_to_program = ctx.accounts.token_program.to_account_info();
        let cpi_mint_to_ctx = CpiContext::new(cpi_mint_to_program, cpi_mint_to_account);
        token::mint_to(cpi_mint_to_ctx, 1)?;

        let creators = vec![
            Creator{
                address: *ctx.accounts.payer.key,
                verified: true,
                share: 100,
            },
            Creator{
                address: *ctx.accounts.mint.key,
                verified: false,
                share: 0,
            }
        ];
         
        let my_collection = Collection{
            verified: false, 
            key: *ctx.accounts.payer.key,
        };

        let my_uses = Uses{
            use_method: mpl_token_metadata::state::UseMethod::Single,
            remaining: 1,
            total: 1,
        };

        let create_ix = create_metadata_accounts_v2(
            *ctx.accounts.mpl_program.key, 
            *ctx.accounts.metadata.key, 
            *ctx.accounts.mint.key,
            *ctx.accounts.payer.key, 
            *ctx.accounts.payer.key, 
            *ctx.accounts.payer.key,
            name,
            symbol, 
            uri, 
            Some(creators), 
            seller_fee_basis_points, 
            true, 
            is_mutable,
            Some(my_collection),
            Some(my_uses),
        );

        invoke(
            &create_ix, 
            &[
                ctx.accounts.metadata.clone(),
                ctx.accounts.mint.clone(),
                ctx.accounts.signer_token_account.clone(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.mpl_program.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
                ctx.accounts.associated_token_program.to_account_info(),    
            ]
        )?;
         
        let master_ix = create_master_edition_v3(
            *ctx.accounts.mpl_program.key, 
            *ctx.accounts.master_edition.key, 
            *ctx.accounts.mint.key, 
            *ctx.accounts.payer.key, 
            *ctx.accounts.payer.key, 
            *ctx.accounts.metadata.key, 
            *ctx.accounts.payer.key, 
            Some(0)
        );

        invoke(
            &master_ix, 
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mpl_program.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ]
        )?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct MetadataDataAccount<'info>{
    #[account(address = mpl_token_metadata::id())]
    pub mpl_program: AccountInfo<'info>,
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub signer_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    pub master_edition: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}