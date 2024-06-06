use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub apr : Uint128,
    pub token_stake : Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    Stake{amount : Uint128,},
    UnStake{},
    WithDraw{amount : Uint128,},
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
