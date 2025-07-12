use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("8JqeoGQZprqWk69oeqQYfUSNdgv8WgKZSx5MUdLLnNoP");

#[program]
pub mod  solana_property_registry {
    use super::*;
    pub fn initialize_admin (ctx: Context<InitializeAdmin>) -> Result<()> {
        let admin = &mut ctx.accounts.admin;
         admin.owner = *ctx.accounts.initializer.key;
        Ok(())
    }

    pub fn register_property (ctx: Context<RegisterProperty>, property_id:String, location:String, area:u64,) -> Result<()> {
        let property = &mut ctx.accounts.property;
         property.owner = *ctx.accounts.owner.key;
         property.property_id = property_id;
         property.location = location;
         property.area = area;
         let current_owner = property.owner;
         property.history.push(current_owner);
        Ok(())
    }


    pub fn transfer_property (ctx: Context<TransferProperty>,new_owner:Pubkey   ) -> Result<()> {
        let property = &mut ctx.accounts.property;
          require!(property.freeze_status == false, CustomError::PropertyFrozen);
          if property.history.len()== 5{
            property.history.remove(0);
            property.owner = new_owner;
             
            property.history.push(new_owner);
          }
        Ok(())
    }

    pub fn update_dispute_status (ctx: Context<UpdateDisputeStatus>,status:bool   ) -> Result<()> {
        let property = &mut ctx.accounts.property;
        let admin = &mut ctx.accounts.admin;
        require!(admin.owner == *ctx.accounts.authority.key, CustomError::Unauthorized);
         property.freeze_status = status;
          
        Ok(())
    }

    pub fn add_nominee (ctx: Context<AddNominee>,nominee:Pubkey, share_percentage: u8,   ) -> Result<()> {
        let property = &mut ctx.accounts.property;
         
        require!( property.nominees.len()<10, CustomError::MaxNomineesReached);
        let mut total_percent: u8 = property.nominees.iter().map(|n| n.share).sum();
        total_percent += share_percentage;
        require!(total_percent <=100, CustomError::ShareExceeded);

        property.nominees.push(NomineeInfo {
            nominee,
            share: share_percentage,
            claimed : false,
        });
          
        Ok(())
    }

     pub fn claim_property (ctx: Context<ClaimNominee>  ) -> Result<()> {
        let property = &mut ctx.accounts.property;
        let claimant = *ctx.accounts.claimant.key;
        let nominee =  property.nominees.iter_mut().find(|n| n.nominee == claimant);
         
        require!(nominee.is_some(),CustomError::NotANominee);
        let nominee = nominee.unwrap();
        require!(nominee.claimed == false, CustomError::AlreadyClaimed);
        nominee.claimed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {    
    #[account(init, payer = initializer, space = 8 + 32)]
    pub admin: Account<'info, Admin>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterProperty<'info> {    
    #[account(init, payer = owner, space = 8 + 512)]
    pub property: Account<'info, Property>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferProperty<'info> {    
    #[account(mut,  has_one = owner)]
    pub property: Account<'info, Property>,     
    pub owner: Signer<'info>, 

 }   

 #[derive(Accounts)]
pub struct UpdateDisputeStatus<'info> {    
    #[account(mut)]
    pub property: Account<'info, Property>,    
    pub admin: Account<'info,Admin>, 
    pub authority: Signer<'info>, 

 }    

 #[derive(Accounts)]
pub struct AddNominee<'info> {    
    #[account(mut, has_one = owner)]
    pub property: Account<'info, Property>,    
    pub owner: Signer<'info>, 

 }   


 #[derive(Accounts)]
pub struct ClaimNominee<'info> {    
    #[account(mut)]
    pub property: Account<'info, Property>,       
    pub claimant: Signer<'info>, 

 }              

#[account]
pub struct Admin {
    pub owner : Pubkey,
}

#[account]
pub struct Property {
    pub property_id:String,
    pub location: String,
    pub area : u64,
    pub owner : Pubkey,
    pub history : Vec<Pubkey>,
    pub freeze_status : bool,
    pub nominees : Vec<NomineeInfo>,
}

#[derive(AnchorSerialize,AnchorDeserialize, Clone)]
pub struct NomineeInfo {
    pub nominee : Pubkey,
    pub share:u8,
    pub claimed : bool,
}

#[error_code]
pub enum CustomError {
    #[msg("You are not authorized to perform this Action")]
    Unauthorized,
    #[msg("Property is currently frozen/disputed")]
    PropertyFrozen,
    #[msg("Cannot add more than 10 nominees")]
    MaxNomineesReached,
    #[msg("Total nominee share cannot exceed 100%")]
    ShareExceeded,
    #[msg("Nominee has already claimed their share")]
    AlreadyClaimed,
    #[msg("You are not listed as a nominee")]
    NotANominee,


}