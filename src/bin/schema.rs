use cosmwasm_schema::write_api;

use project_name::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        //stake : ExecuteMsg::Stake,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
