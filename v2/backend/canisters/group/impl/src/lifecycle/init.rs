use crate::lifecycle::{init_logger, init_state};
use crate::Data;
use group_canister::init::Args;
use ic_cdk_macros::init;
use slog::info;
use slog_scope::with_logger;
use utils::env::canister::CanisterEnv;
use utils::env::Environment;

#[init]
fn init(args: Args) {
    init_logger();
    ic_cdk::setup();

    let env = Box::new(CanisterEnv::new(false));
    let group_index_canister_id = env.caller();

    let data = Data::new(
        env.canister_id().into(),
        args.is_public,
        args.name,
        args.description,
        args.avatar,
        args.history_visible_to_new_joiners,
        args.created_by_principal,
        args.created_by_user_id,
        env.now(),
        args.mark_active_duration,
        group_index_canister_id,
        args.wasm_version,
    );

    init_state(env, data);

    with_logger(|l| info!(l, "Initialization complete"));
}
