use pinocchio::pubkey::Pubkey;

use crate::utils::{DataLen, Initialized};

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VaultState {
    is_initialized: bool,
    pub user: Pubkey,
    pub expiration: i64,
    pub amount: u64,
    pub sol_price_at_initialization: u64,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl DataLen for VaultState {
    const LEN: usize = core::mem::size_of::<VaultState>();
}

impl Initialized for VaultState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl VaultState {
    pub fn initialize(
        &mut self,
        user: Pubkey,
        expiration: i64,
        amount: u64,
        sol_price_at_initialization: u64,
        vault_bump: u8,
        state_bump: u8,
    ) {
        self.is_initialized = true;
        self.user = user;
        self.expiration = expiration;
        self.amount = amount;
        self.sol_price_at_initialization = sol_price_at_initialization;
        self.vault_bump = vault_bump;
        self.state_bump = state_bump;
    }
}
