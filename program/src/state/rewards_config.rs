use pinocchio::pubkey::Pubkey;

use crate::utils::{DataLen, Initialized};

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RewardsConfig {
    is_initialized: bool,
    pub rewards_bump: u8,
    pub bump: u8,
}

impl DataLen for RewardsConfig {
    const LEN: usize = core::mem::size_of::<RewardsConfig>();
}

impl Initialized for Fundraiser {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl RewardsConfig {
    pub fn initialize(&mut self, 
        pub rewards_bump: u8,
        pub bump: u8
    ){
       self.rewards_bump = rewards_bump;
       self.bump = bump; 
    }      
}

