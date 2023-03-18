mod apply;
mod new;
mod create_block;
mod delay_difficulty;
mod transition;
mod validator;
use std::collections::BTreeMap;
use neutrondb::Store;
use crate::account::Account;
use crate::block::Block;
use crate::address::Address;

#[derive(Debug)]
pub struct State {
    pub accounts: BTreeMap<Address, [u8;32]>,
    pub accounts_store: Store<Address, Account>,
    pub latest_block: Block,
}