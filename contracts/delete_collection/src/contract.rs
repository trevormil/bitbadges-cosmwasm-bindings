use bitbadges_cosmwasm::{delete_collection_msg, BitBadgesMsg};

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
        ExecuteMsg::DeleteCollectionMsg { collection_id } => {
          execute_msg_delete_collection(collection_id)
        }

        // Add other messages here as needed
    }
}

pub fn execute_msg_delete_collection(
    collection_id: String,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = delete_collection_msg(
        collection_id,
    );

    Ok(Response::new().add_message(msg))
}