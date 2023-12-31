use crate::{read_state, RuntimeState};
use ic_cdk_macros::inspect_message;

#[inspect_message]
fn inspect_message() {
    read_state(accept_if_valid);
}

fn accept_if_valid(state: &RuntimeState) {
    let method_name = ic_cdk::api::call::method_name();

    let is_c2c_method = method_name.starts_with("c2c") || method_name == "wallet_receive";
    let is_frozen = state.data.frozen.value.is_some();

    // 'inspect_message' only applies to ingress messages so calls to c2c methods should be rejected
    if is_c2c_method || is_frozen {
        return;
    }

    let caller = state.env.caller();
    let permissions = &state.data.chat.permissions;
    let is_valid = if let Some(role) = state.data.get_member(caller).map(|p| p.role) {
        match method_name.as_str() {
            "add_reaction" | "remove_reaction" => role.can_react_to_messages(permissions),
            "block_user" => role.can_block_users(permissions),
            "convert_into_community" => role.is_owner(),
            "delete_group" => role.can_delete_group(),
            "enable_invite_code" | "disable_invite_code" | "reset_invite_code" => role.can_invite_users(permissions),
            "make_private" => role.can_change_group_visibility(),
            "pin_message" | "pin_message_v2" => role.can_pin_messages(permissions),
            "remove_participant" => role.can_remove_members(permissions),
            "send_message_v2" => role.can_send_messages(permissions) || role.can_reply_in_thread(permissions),
            "unblock_user" => role.can_unblock_users(permissions),
            "unpin_message" => role.can_pin_messages(permissions),
            "update_group_v2" => role.can_update_group(permissions),
            "update_permissions" => role.can_change_permissions(),
            "delete_messages"
            | "undelete_messages"
            | "change_role"
            | "claim_prize"
            | "edit_message_v2"
            | "put_chunk"
            | "register_poll_vote"
            | "register_proposal_vote"
            | "register_proposal_vote_v2"
            | "toggle_mute_notifications" => true,
            _ => false,
        }
    } else {
        state.data.get_invitation(caller).is_some() && method_name == "decline_invitation"
    };

    if is_valid {
        ic_cdk::api::call::accept_message();
    }
}
