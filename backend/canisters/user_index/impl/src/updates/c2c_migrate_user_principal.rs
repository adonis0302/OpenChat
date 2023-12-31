use crate::guards::caller_is_openchat_user;
use crate::{mutate_state, RuntimeState};
use canister_api_macros::update_msgpack;
use canister_tracing_macros::trace;
use user_index_canister::c2c_migrate_user_principal::{Response::*, *};

#[update_msgpack(guard = "caller_is_openchat_user")]
#[trace]
fn c2c_migrate_user_principal(args: Args) -> Response {
    mutate_state(|state| c2c_migrate_user_principal_impl(args, state))
}

fn c2c_migrate_user_principal_impl(args: Args, state: &mut RuntimeState) -> Response {
    let user_id = state.env.caller().into();

    if let Some(user) = state.data.users.get_by_user_id(&user_id) {
        if user.principal == args.new_principal {
            SuccessNoChange
        } else if state.data.users.get_by_principal(&args.new_principal).is_some() {
            PrincipalAlreadyInUse
        } else if state.data.user_principal_migration_queue.count_pending(&user_id) > 0 {
            MigrationAlreadyInProgress
        } else {
            let old_principal = user.principal;
            let now = state.env.now();

            let mut clone = user.clone();
            clone.principal = args.new_principal;
            state.data.users.update(clone, now);

            state.data.user_principal_migration_queue.push(
                user_id,
                old_principal,
                args.new_principal,
                args.groups,
                state.data.storage_index_canister_id,
                state.data.notifications_index_canister_id,
            );
            crate::jobs::notify_user_principal_migrations::start_job_if_required(state);

            Success
        }
    } else {
        UserNotFound
    }
}
