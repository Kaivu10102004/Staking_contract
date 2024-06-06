use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item,Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub apr : Uint128,
    pub token_stake : Addr,
}
#[cw_serde]
pub struct StakeQueue{
    pub sender : Addr,
    pub amount : Uint128,
}
#[cw_serde]
pub struct StakerInfo{
    //pub staker: Addr,
    pub stake_amount : Uint128,
    pub reward_amount : Uint128,
    pub last_update_time : Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const STAKE_QUEUE: Map<&Addr,StakerInfo> = Map::new("stake_queue");
