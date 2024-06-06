#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg
};
use cw20::{self, Cw20ExecuteMsg};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetStakeamountResponses, GetRewardamountResponses};
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
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::Stake{amount} => execute_stake(_deps, env, _info, amount),
        ExecuteMsg::UnStake{} => execute_unstake(_deps, env, _info),
        ExecuteMsg::WithDraw{amount} => execute_withdraw(_deps, env, _info, amount),
    }
}

//#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute_stake(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // get config 
    let config = CONFIG.load(_deps.storage)?;
    let mut stake_amount_now : Uint128 = amount.clone();
    let mut reward_amount_now : Uint128 = Uint128::new(0);
    let now = Uint128 :: new(env.block.time.seconds() as u128);
    //STAKE_QUEUE.load(deps.storage, &staker)?
    match STAKE_QUEUE.may_load(_deps.storage, &_info.sender.clone())? {
        Some(value) => {
            reward_amount_now = reward_amount_now.clone() + value.reward_amount.clone() + value.stake_amount.clone() * (now.clone() - value.last_update_time.clone()) * config.apr.clone() / Uint128::new(100) / Uint128::new(31536000);
            stake_amount_now = value.stake_amount.clone() + stake_amount_now.clone();
        },
        None => {}
        ,
    }
    //approve
    //tran
    let transfer_msg = WasmMsg::Execute {
        contract_addr: env.contract.address.clone().to_string(),
        msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
            owner: _info.sender.clone().to_string(),
            recipient: env.contract.address.to_string(),
            amount : amount.clone(),
        })?,
        funds: vec![],
    };

    // save tran to stake_queue
    STAKE_QUEUE.save(
        _deps.storage,
        &_info.sender.clone(), 
        &StakerInfo{
        stake_amount : stake_amount_now.clone(),
        reward_amount: reward_amount_now.clone(),
        last_update_time : now.clone(),
        },
    )?;
    return Ok(Response::default().add_message(transfer_msg))
}

//#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute_unstake(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    // get config 
    let config = CONFIG.load(_deps.storage)?;
    let mut reward_amount_now : Uint128 = Uint128::new(0);
    let now = Uint128 :: new(env.block.time.seconds() as u128);
    match STAKE_QUEUE.may_load(_deps.storage, &_info.sender.clone())? {
        Some(value) => {
            reward_amount_now = reward_amount_now.clone() + value.reward_amount.clone() + value.stake_amount.clone() * (now.clone() - value.last_update_time.clone()) * config.apr.clone() / Uint128::new(100) / Uint128::new(31536000);
            // tran
            let transfer_msg = WasmMsg::Execute {
                contract_addr: env.contract.address.clone().to_string(),
                msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
                    owner : _info.sender.clone().to_string(),
                    recipient: env.contract.address.clone().to_string(),
                    amount: Uint128::from(value.stake_amount.clone()),
                })?,
                funds: vec![],
            }; 
            // save tran to stake_queue
            STAKE_QUEUE.save(
                _deps.storage,
                &_info.sender.clone(), 
                &StakerInfo{
                stake_amount : Uint128::new(0),
                reward_amount: reward_amount_now.clone(),
                last_update_time : now.clone(),
                },
            )?;
            return Ok(Response::default().add_message(transfer_msg))
        },
        None => {
            return Err(ContractError::Unauthorized {});
        }
        ,
    }
    
}

//#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute_withdraw(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(_deps.storage)?;
    let mut reward_amount_now : Uint128 = Uint128::new(0);
    let now = Uint128 :: new(env.block.time.seconds() as u128);
    match STAKE_QUEUE.may_load(_deps.storage, &_info.sender.clone())? {
        Some(value) => {
            reward_amount_now = reward_amount_now.clone() + value.reward_amount.clone() + value.stake_amount.clone() * (now.clone() - value.last_update_time.clone()) * config.apr.clone() / Uint128::new(100) / Uint128::new(31536000);
            if reward_amount_now < amount {
                return Err(ContractError::NotEnBalance{})
            }
            else {
                reward_amount_now = reward_amount_now.clone() - amount.clone();
                let transfer_msg = WasmMsg::Execute {
                    contract_addr: env.contract.address.clone().to_string(),
                    msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                        recipient: _info.sender.clone().to_string(),
                        amount: Uint128::from(amount.clone()),
                    })?,
                    funds: vec![],
                };
                // save tran to stake_queue
                STAKE_QUEUE.save(
                    _deps.storage,
                    &_info.sender.clone(), 
                    &StakerInfo{
                    stake_amount : value.reward_amount.clone(),
                    reward_amount: reward_amount_now.clone(),
                    last_update_time : now.clone(),
                    },
                )?;
                return Ok(Response::default().add_message(transfer_msg))
            }
        },
        None => {
            return Err(ContractError::Unauthorized {})
        }
        ,
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    _deps: Deps, 
    env: Env,
    _msg: QueryMsg,
    //sender : Addr,
) -> StdResult<Binary> {
    match _msg {
        QueryMsg::GetStakeamount {staker} => to_json_binary(&get_stakeamount(_deps,staker)?),
        QueryMsg::GetRewardamount {staker} => to_json_binary(&get_rewardamount(_deps,env,staker)?),
    }
}

pub fn get_stakeamount(
    _deps : Deps,
    staker : Addr,
) -> StdResult<GetStakeamountResponses> {
    let res = GetStakeamountResponses {
        amount : (STAKE_QUEUE.load(_deps.storage, &staker)).unwrap().stake_amount.clone(),
    };
    return Ok(res)
}


pub fn get_rewardamount(
    _deps : Deps,
    env: Env,
    staker : Addr,
)-> StdResult<GetRewardamountResponses> {
    let config = CONFIG.load(_deps.storage)?;
    let mut reward_amount_now : Uint128 = Uint128::new(0);
    let now = Uint128 :: new(env.block.time.seconds() as u128);
    match STAKE_QUEUE.may_load(_deps.storage, &staker.clone())? {
        Some(value) => {
            reward_amount_now = reward_amount_now.clone() + value.reward_amount.clone() + value.stake_amount.clone() * (now.clone() - value.last_update_time.clone()) * config.apr.clone() / Uint128::new(100);
        },
        None => {}
        ,
    }
    let res = GetRewardamountResponses {
        amount : (STAKE_QUEUE.load(_deps.storage, &staker)).unwrap().reward_amount.clone(),
    };
    return Ok(res)
}

#[cfg(test)]
mod tests {}