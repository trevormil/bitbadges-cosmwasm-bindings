use bitbadges_cosmwasm::{
  delete_collection_msg, BitBadgesMsg, AddressMapping, address_mappings_msg, Transfer, transfer_badges_msg,
  Balance, CollectionPermissions, ManagerTimeline, CollectionMetadataTimeline, BadgeMetadataTimeline, OffChainBalancesMetadataTimeline, CustomDataTimeline, CollectionApproval, StandardsTimeline, IsArchivedTimeline, UserPermissions, UserOutgoingApproval, UserIncomingApproval,
  create_collection_msg, update_collection_msg, universal_update_collection_msg,
};

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
        ExecuteMsg::CreateAddressMappingsMsg { address_mappings } => {
          execute_msg_create_address_mappings(address_mappings)
        }
        ExecuteMsg::TransferBadgeMsg { collection_id, transfers } => {
          execute_msg_transfer_badges(collection_id, transfers)
        }
        ExecuteMsg::CreateCollectionMsg { balances_type, default_outgoing_approvals, default_incoming_approvals, badges_to_create, collection_permissions, manager_timeline, collection_metadata_timeline, badge_metadata_timeline, off_chain_balances_metadata_timeline, custom_data_timeline, collection_approvals, standards_timeline, is_archived_timeline, default_auto_approve_self_initiated_outgoing_transfers, default_auto_approve_self_initiated_incoming_transfers, default_user_permissions } => {
          execute_msg_create_collection(balances_type, default_outgoing_approvals, default_incoming_approvals, badges_to_create, collection_permissions, manager_timeline, collection_metadata_timeline, badge_metadata_timeline, off_chain_balances_metadata_timeline, custom_data_timeline, collection_approvals, standards_timeline, is_archived_timeline, default_auto_approve_self_initiated_outgoing_transfers, default_auto_approve_self_initiated_incoming_transfers, default_user_permissions)
        }
        ExecuteMsg::UpdateCollectionMsg { collection_id, badges_to_create, update_collection_permissions, collection_permissions, update_manager_timeline, manager_timeline, update_collection_metadata_timeline, collection_metadata_timeline, update_badge_metadata_timeline, badge_metadata_timeline, update_off_chain_balances_metadata_timeline, off_chain_balances_metadata_timeline, update_custom_data_timeline, custom_data_timeline, update_collection_approvals, collection_approvals, update_standards_timeline, standards_timeline, update_is_archived_timeline, is_archived_timeline } => {
          execute_msg_update_collection(collection_id, badges_to_create, update_collection_permissions, collection_permissions, update_manager_timeline, manager_timeline, update_collection_metadata_timeline, collection_metadata_timeline, update_badge_metadata_timeline, badge_metadata_timeline, update_off_chain_balances_metadata_timeline, off_chain_balances_metadata_timeline, update_custom_data_timeline, custom_data_timeline, update_collection_approvals, collection_approvals, update_standards_timeline, standards_timeline, update_is_archived_timeline, is_archived_timeline)
        }
        ExecuteMsg::UniversalUpdateCollectionMsg { collection_id, balances_type, default_outgoing_approvals, default_incoming_approvals, badges_to_create, update_collection_permissions, collection_permissions, update_manager_timeline, manager_timeline, update_collection_metadata_timeline, collection_metadata_timeline, update_badge_metadata_timeline, badge_metadata_timeline, update_off_chain_balances_metadata_timeline, off_chain_balances_metadata_timeline, update_custom_data_timeline, custom_data_timeline, update_collection_approvals, collection_approvals, update_standards_timeline, standards_timeline, update_is_archived_timeline, is_archived_timeline, default_auto_approve_self_initiated_outgoing_transfers, default_auto_approve_self_initiated_incoming_transfers, default_user_permissions } => {
          execute_msg_universal_update_collection(collection_id, balances_type, default_outgoing_approvals, default_incoming_approvals, badges_to_create, update_collection_permissions, collection_permissions, update_manager_timeline, manager_timeline, update_collection_metadata_timeline, collection_metadata_timeline, update_badge_metadata_timeline, badge_metadata_timeline, update_off_chain_balances_metadata_timeline, off_chain_balances_metadata_timeline, update_custom_data_timeline, custom_data_timeline, update_collection_approvals, collection_approvals, update_standards_timeline, standards_timeline, update_is_archived_timeline, is_archived_timeline, default_auto_approve_self_initiated_outgoing_transfers, default_auto_approve_self_initiated_incoming_transfers, default_user_permissions)
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

pub fn execute_msg_create_address_mappings(
    address_mappings: Vec<AddressMapping>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = address_mappings_msg(
        address_mappings,
    );

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_transfer_badges(
    collection_id: String,
    transfers: Vec<Transfer>,
) -> StdResult<Response<BitBadgesMsg>> {
    let msg = transfer_badges_msg(
        collection_id,
        transfers,
    );

    Ok(Response::new().add_message(msg))
}

pub fn execute_msg_create_collection(
  balances_type: String,
  default_outgoing_approvals: Vec<UserOutgoingApproval>,
  default_incoming_approvals: Vec<UserIncomingApproval>,
  badges_to_create: Vec<Balance>,
  collection_permissions: CollectionPermissions,
  manager_timeline: Vec<ManagerTimeline>,
  collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
  badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
  off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
  custom_data_timeline: Vec<CustomDataTimeline>,
  collection_approvals: Vec<CollectionApproval>,
  standards_timeline: Vec<StandardsTimeline>,
  is_archived_timeline: Vec<IsArchivedTimeline>,
  default_auto_approve_self_initiated_outgoing_transfers: bool,
  default_auto_approve_self_initiated_incoming_transfers: bool,
  default_user_permissions: UserPermissions,
) -> StdResult<Response<BitBadgesMsg>> {
  let msg = create_collection_msg(
    balances_type,
    default_outgoing_approvals,
    default_incoming_approvals,
    badges_to_create,
    collection_permissions,
    manager_timeline,
    collection_metadata_timeline,
    badge_metadata_timeline,
    off_chain_balances_metadata_timeline,
    custom_data_timeline,
    collection_approvals,
    standards_timeline,
    is_archived_timeline,
    default_auto_approve_self_initiated_outgoing_transfers,
    default_auto_approve_self_initiated_incoming_transfers,
    default_user_permissions,
  );

  Ok(Response::new().add_message(msg))
}

pub fn execute_msg_update_collection(
  collection_id: String,
  badges_to_create: Vec<Balance>,
  update_collection_permissions: bool,
  collection_permissions: CollectionPermissions,
  update_manager_timeline: bool,
  manager_timeline: Vec<ManagerTimeline>,
  update_collection_metadata_timeline: bool,
  collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
  update_badge_metadata_timeline: bool,
  badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
  update_off_chain_balances_metadata_timeline: bool,
  off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
  update_custom_data_timeline: bool,
  custom_data_timeline: Vec<CustomDataTimeline>,
  update_collection_approvals: bool,
  collection_approvals: Vec<CollectionApproval>,
  update_standards_timeline: bool,
  standards_timeline: Vec<StandardsTimeline>,
  update_is_archived_timeline: bool,
  is_archived_timeline: Vec<IsArchivedTimeline>,
) -> StdResult<Response<BitBadgesMsg>> {
  let msg = update_collection_msg(
    collection_id,
    badges_to_create,
    update_collection_permissions,
    collection_permissions,
    update_manager_timeline,
    manager_timeline,
    update_collection_metadata_timeline,
    collection_metadata_timeline,
    update_badge_metadata_timeline,
    badge_metadata_timeline,
    update_off_chain_balances_metadata_timeline,
    off_chain_balances_metadata_timeline,
    update_custom_data_timeline,
    custom_data_timeline,
    update_collection_approvals,
    collection_approvals,
    update_standards_timeline,
    standards_timeline,
    update_is_archived_timeline,
    is_archived_timeline,
  );

  Ok(Response::new().add_message(msg))
}

pub fn execute_msg_universal_update_collection(
  collection_id: String,
  balances_type: String,
  default_outgoing_approvals: Vec<UserOutgoingApproval>,
  default_incoming_approvals: Vec<UserIncomingApproval>,
  badges_to_create: Vec<Balance>,
  update_collection_permissions: bool,
  collection_permissions: CollectionPermissions,
  update_manager_timeline: bool,
  manager_timeline: Vec<ManagerTimeline>,
  update_collection_metadata_timeline: bool,
  collection_metadata_timeline: Vec<CollectionMetadataTimeline>,
  update_badge_metadata_timeline: bool,
  badge_metadata_timeline: Vec<BadgeMetadataTimeline>,
  update_off_chain_balances_metadata_timeline: bool,
  off_chain_balances_metadata_timeline: Vec<OffChainBalancesMetadataTimeline>,
  update_custom_data_timeline: bool,
  custom_data_timeline: Vec<CustomDataTimeline>,
  update_collection_approvals: bool,
  collection_approvals: Vec<CollectionApproval>,
  update_standards_timeline: bool,
  standards_timeline: Vec<StandardsTimeline>,
  update_is_archived_timeline: bool,
  is_archived_timeline: Vec<IsArchivedTimeline>,
  default_auto_approve_self_initiated_outgoing_transfers: bool,
  default_auto_approve_self_initiated_incoming_transfers: bool,
  default_user_permissions: UserPermissions,
) -> StdResult<Response<BitBadgesMsg>> {
  let msg = universal_update_collection_msg(
    collection_id,
      balances_type,
      default_outgoing_approvals,
      default_incoming_approvals,
      badges_to_create,
      update_collection_permissions,
      collection_permissions,
      update_manager_timeline,
      manager_timeline,
      update_collection_metadata_timeline,
      collection_metadata_timeline,
      update_badge_metadata_timeline,
      badge_metadata_timeline,
      update_off_chain_balances_metadata_timeline,
      off_chain_balances_metadata_timeline,
      update_custom_data_timeline,
      custom_data_timeline,
      update_collection_approvals,
      collection_approvals,
      update_standards_timeline,
      standards_timeline,
      update_is_archived_timeline,
      is_archived_timeline,
      default_auto_approve_self_initiated_outgoing_transfers,
      default_auto_approve_self_initiated_incoming_transfers,
      default_user_permissions,
  );

  Ok(Response::new().add_message(msg))
}