use bitbadges_cosmwasm::{create_register_addresses_msg, BitBadgesMsg};

use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::msg::{ExecuteMsg, InstantiateMsg};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<BitBadgesMsg>, StdError> {
    match msg {
        ExecuteMsg::RegisterAddressesMsg {
            addresses_to_register,
        } => execute_msg_register_addresses(addresses_to_register),

        // Add other messages here as needed
    }
}

pub fn execute_msg_register_addresses(
    addresses_to_register: Vec<String>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = create_register_addresses_msg(
        addresses_to_register,
    );

    Ok(Response::new().add_message(msg))
}