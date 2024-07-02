#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_json,to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg
};
use cw20::{self, Cw20ExecuteMsg,Cw20ReceiveMsg};
use crate::error::ContractError;
use crate::msg::{Cw20HookMsg,ExecuteMsg, InstantiateMsg, QueryMsg, GetStakeamountResponses, GetRewardamountResponses};
use crate::state::{Config, CONFIG, StakerInfo, STAKE_QUEUE};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // save config to storage
    CONFIG.save(deps.storage, &Config { owner: msg.owner, apr : msg.apr, token_stake : msg.token_stake})?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        //ExecuteMsg::Stake{amount} => execute_stake(deps, env, info, amount),
        ExecuteMsg::UnStake{amount} => execute_unstake(deps, env, info, amount),
        ExecuteMsg::WithDraw{} => execute_withdraw(deps, env, info),
    }
}

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_json(&cw20_msg.msg)?{
        Cw20HookMsg::Stake {} => {
            let config = CONFIG.load(deps.storage)?;
            if info.sender != config.token_stake {
                return Err(ContractError::InvalidStakingToken {});
            }
            let mut stake_amount_now : Uint128 = cw20_msg.amount.clone();
            let mut reward_amount_now : Uint128 = Uint128::new(0);
            let now = Uint128 :: new(env.block.time.seconds() as u128);
            let staker_addr = deps.api.addr_validate(&cw20_msg.sender)?;
            match STAKE_QUEUE.may_load(deps.storage, &staker_addr.clone())? {
                Some(value) => {
                    reward_amount_now = reward_amount_now.clone() + value.reward_amount.clone() + value.stake_amount.clone() * (now.clone() - value.last_update_time.clone()) * config.apr.clone() / Uint128::new(100) / Uint128::new(31536000);
                    stake_amount_now = value.stake_amount.clone() + stake_amount_now.clone();
                },
                None => {}
                ,
            }
            STAKE_QUEUE.save(
                deps.storage,
                &staker_addr.clone(), 
                &StakerInfo{
                stake_amount : stake_amount_now.clone(),
                reward_amount: reward_amount_now.clone(),
                last_update_time : now.clone(),
                },
            )?;
            return Ok(Response::default())
        }
    }
}

pub fn execute_unstake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount : Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let mut stake_amount_now : Uint128 = Uint128::new(0);
    let mut reward_amount_now : Uint128 = Uint128::new(0);
    let now = Uint128 :: new(env.block.time.seconds() as u128);
    match STAKE_QUEUE.may_load(deps.storage, &info.sender.clone())? {
        Some(value) => {
            stake_amount_now = stake_amount_now.clone() + value.stake_amount.clone();
            reward_amount_now = reward_amount_now.clone() + value.reward_amount.clone() + value.stake_amount.clone() * (now.clone() - value.last_update_time.clone()) * config.apr.clone() / Uint128::new(100) / Uint128::new(31536000);
            if stake_amount_now < amount {
                return Err(ContractError::NotEnBalance{})
            }
            else {
                //reward_amount_now = reward_amount_now.clone() + value.reward_amount.clone() + value.stake_amount.clone() * (now.clone() - value.last_update_time.clone()) * config.apr.clone() / Uint128::new(100) / Uint128::new(31536000);
                stake_amount_now = value.stake_amount.clone() - amount.clone();
                let transfer_msg = WasmMsg::Execute {
                    contract_addr: config.token_stake.clone().to_string(),
                    msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                        recipient: info.sender.clone().to_string(),
                        amount: Uint128::from(amount.clone()),
                    })?,
                    funds: vec![],
                }; 
                STAKE_QUEUE.save(
                    deps.storage,
                    &info.sender.clone(), 
                    &StakerInfo{
                    stake_amount : stake_amount_now.clone(),
                    reward_amount: reward_amount_now.clone(),
                    last_update_time : now.clone(),
                    },
                )?;
                return Ok(Response::default().add_message(transfer_msg))
                } 
            },
            None => {
                return Err(ContractError::Unauthorized {});
            }
            ,     

    }
    
}

pub fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let mut stake_amount_now : Uint128 = Uint128::new(0);
    let mut reward_amount_now : Uint128 = Uint128::new(0);
    let now = Uint128 :: new(env.block.time.seconds() as u128);
    match STAKE_QUEUE.may_load(deps.storage, &info.sender.clone())? {
        Some(value) => {
            stake_amount_now = stake_amount_now.clone() + value.stake_amount.clone();
            reward_amount_now = reward_amount_now.clone() + value.reward_amount.clone() + value.stake_amount.clone() * (now.clone() - value.last_update_time.clone()) * config.apr.clone() / Uint128::new(100) / Uint128::new(31536000);     
            let transfer_msg = WasmMsg::Execute {
                contract_addr: config.token_stake.clone().to_string(),
                msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: info.sender.clone().to_string(),
                    amount: Uint128::from(reward_amount_now.clone()),
                })?,
                funds: vec![],
            };
            // save tran to stake_queue
            STAKE_QUEUE.save(
                deps.storage,
                &info.sender.clone(), 
                &StakerInfo{
                stake_amount : stake_amount_now.clone(),
                reward_amount: Uint128::new(0),
                last_update_time : now.clone(),
                },
            )?;
            return Ok(Response::default().add_message(transfer_msg))
        },
        None => {
            return Err(ContractError::Unauthorized {})
        }
        ,
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps, 
    env: Env,
    msg: QueryMsg,
    //sender : Addr,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStakeamount {staker} => to_json_binary(&get_stakeamount(deps,staker)?),
        QueryMsg::GetRewardamount {staker} => to_json_binary(&get_rewardamount(deps,env,staker)?),
    }
}

pub fn get_stakeamount(
    deps : Deps,
    staker : Addr,
) -> StdResult<GetStakeamountResponses> {
    let res = GetStakeamountResponses {
        amount : (STAKE_QUEUE.load(deps.storage, &staker)).unwrap().stake_amount.clone(),
    };
    return Ok(res)
}


pub fn get_rewardamount(
    deps : Deps,
    env: Env,
    staker : Addr,
)-> StdResult<GetRewardamountResponses> {
    let config = CONFIG.load(deps.storage)?;
    let mut reward_amount_now : Uint128 = Uint128::new(0);
    let now = Uint128 :: new(env.block.time.seconds() as u128);
    match STAKE_QUEUE.may_load(deps.storage, &staker.clone())? {
        Some(value) => {
            reward_amount_now = reward_amount_now.clone() + value.reward_amount.clone() + value.stake_amount.clone() * (now.clone() - value.last_update_time.clone()) * config.apr.clone() / Uint128::new(100) / Uint128::new(31536000);
        },
        None => {}
        ,
    }

    let res = GetRewardamountResponses {
        amount : reward_amount_now,
    };
    return Ok(res)
}

#[cfg(test)]
mod tests {}