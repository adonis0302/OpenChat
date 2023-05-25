use crate::guards::caller_is_local_user_index;
use crate::{mutate_state, openchat_bot, RuntimeState};
use canister_api_macros::update_msgpack;
use canister_tracing_macros::trace;
use user_canister::c2c_notify_user_events::{Response::*, *};
use user_canister::Event;

#[update_msgpack(guard = "caller_is_local_user_index")]
#[trace]
fn c2c_notify_events(args: Args) -> Response {
    mutate_state(|state| c2c_notify_events_impl(args, state))
}

fn c2c_notify_events_impl(args: Args, runtime_state: &mut RuntimeState) -> Response {
    for event in args.events {
        process_event(event, runtime_state);
    }
    Success
}

fn process_event(event: Event, runtime_state: &mut RuntimeState) {
    match event {
        Event::UsernameChanged(ev) => {
            runtime_state.data.username = ev.username;
        }
        Event::PhoneNumberConfirmed(ev) => {
            runtime_state.data.phone_is_verified = true;
            runtime_state.data.storage_limit = ev.new_storage_limit;
            openchat_bot::send_phone_number_confirmed_bot_message(&ev, runtime_state);
        }
        Event::StorageUpgraded(ev) => {
            runtime_state.data.storage_limit = ev.new_storage_limit;
            openchat_bot::send_storage_ugraded_bot_message(&ev, runtime_state);
        }
        Event::ReferredUserRegistered(ev) => {
            openchat_bot::send_referred_user_joined_message(&ev, runtime_state);
        }
        Event::UserSuspended(ev) => {
            openchat_bot::send_user_suspended_message(&ev, runtime_state);
        }
        Event::OpenChatBotMessage(content) => {
            openchat_bot::send_message(*content, false, runtime_state);
        }
        Event::UserJoinedGroup(ev) => {
            let now = runtime_state.env.now();
            runtime_state.data.group_chats.join(ev.chat_id, ev.latest_message_index, now);
            runtime_state.data.hot_group_exclusions.remove(&ev.chat_id, now);
        }
        Event::UserJoinedCommunity(ev) => {
            let now = runtime_state.env.now();
            runtime_state.data.communities.join(ev.community_id, now);
        }
        Event::DiamondMembershipPaymentReceived(ev) => {
            runtime_state.data.diamond_membership_expires_at = Some(ev.expires_at);

            if ev.send_bot_message {
                openchat_bot::send_text_message("Payment received for Diamond membership!".to_string(), false, runtime_state);
            }
        }
    }
}
