use crate::guards::caller_is_owner;
use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_canister::events_by_index::{Response::*, *};

#[query(guard = "caller_is_owner")]
fn events_by_index(args: Args) -> Response {
    read_state(|state| events_by_index_impl(args, state))
}

fn events_by_index_impl(args: Args, runtime_state: &RuntimeState) -> Response {
    if let Some(chat) = runtime_state.data.direct_chats.get(&args.user_id.into()) {
        let events = chat.events.get_by_index(args.events);
        let affected_events = chat.events.affected_events(&events);

        Success(SuccessResult { events, affected_events })
    } else {
        ChatNotFound
    }
}
