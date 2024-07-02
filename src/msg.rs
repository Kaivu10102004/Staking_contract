use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;
#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub apr : Uint128,
    pub token_stake : Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    //Stake{amount : Uint128,},
    UnStake{amount : Uint128,},
    WithDraw{},
    
}

#[cw_serde]
pub enum Cw20HookMsg {
    Stake {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Uint128)]
    GetStakeamount {staker : Addr,},
    #[returns(Uint128)]
    GetRewardamount{staker : Addr,},
}

#[cw_serde]
pub struct GetStakeamountResponses{
    pub amount : Uint128,
}

#[cw_serde]
pub struct GetRewardamountResponses{
    pub amount : Uint128,
}