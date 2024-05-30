use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

pub const CONFIG: Item<Config> = Item::new("config");
pub const APR: Item<u64> = Item::new("apr");
pub const TOKEN_STAKE: Item<u64> = Item::new("token_stake");
pub const TOKEN : Item<Addr> = Item::new("token");
#[cw_serde]
pub struct Config {
    pub owner: Addr,
}
pub struct StakeQueue{
    pub sender : String,
    pub time : String,
    pub amount : u64,
    pub token_address : String,
}