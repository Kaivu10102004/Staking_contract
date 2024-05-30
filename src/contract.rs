#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
//use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, Uint256, WasmMsg,
};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, APR, TOKEN_STAKE,TOKEN};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // save config to storage
    CONFIG.save(deps.storage, &Config { owner: msg.owner })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::SetToken {token_address} => Execute_Set_Token(_deps, _env, _info, token_address),
        ExecuteMsg::SetAPR{amount} => Execute_Set_APR(_deps, _env, _info, amount),
        ExecuteMsg::Stake{amount} => Execute_Stake(_deps, _env, _info, amount),
        ExecuteMsg::WithDraw{amount} => Execute_WithDraw(_deps, _env, _info, amount),
    }
}

fn Execute_Set_Token(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    token_address: Addr,
) -> Result<Response, ContractError> {
    // load config from storage
    let config = CONFIG.load(_deps.storage)?;

    // check owner: only contract owner can execute
    if config.owner != _info.sender {
        return Err(ContractError::Unauthorized {});
    }

    //save token address to storage
    TOKEN.save(_deps.storage, &token_address)?;
    Ok(Response::default())
}

fn Execute_Set_APR(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    amount: u64,
) -> Result<Response, ContractError> {
    // load config from storage
    let config = CONFIG.load(_deps.storage)?;

    // check owner: only contract owner can execute
    if config.owner != _info.sender {
        return Err(ContractError::Unauthorized {});
    }

    //save token address to storage
    APR.save(_deps.storage, &amount)?;
    Ok(Response::default())
}

fn Stake(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    amount: u64,
) -> Result<Response, ContractError> {
    // load config from storage
    let config = CONFIG.load(_deps.storage)?;

    // check owner: only contract owner can execute
    if config.owner != _info.sender {
        return Err(ContractError::Unauthorized {});
    }

    
    
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // read SUM from storage
        QueryMsg::Example {} => to_json_binary(&SUM.load(deps.storage)?),
    }
}


// Define a transfer function that sends ERC-20 tokens from Contract A to Receiver B
fn transfer_tokens(
    info: MessageInfo,
    token_contract_address: Addr, // Address of the ERC-20 token contract
    receiver_address: Addr,       // Address of the receiver
    amount: Uint128,              // Amount of tokens to transfer
) -> StdResult<Response> {
    let transfer_msg = WasmMsg::Execute {
        contract_addr: token_contract_address.clone().to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: receiver_address.clone().to_string(),
            amount: amount.clone(),
        })?,
        funds: vec![],
    };

    // Send the transfer message to the token contract
    let res = Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "transfer")
        .add_attribute("sender", info.sender.clone().as_str())
        .add_attribute("recipient", receiver_address.clone().as_str())
        .add_attribute("amount", amount.clone().to_string())
        .add_attribute("token_contract", token_contract_address.clone().as_str());

    Ok(res)
}

#[cfg(test)]
mod tests {}