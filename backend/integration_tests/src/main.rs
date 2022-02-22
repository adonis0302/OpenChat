use fondue::*;
use ic_fondue::ic_manager::IcManager;
use ic_fondue::internet_computer::InternetComputer;
use ic_registry_subnet_type::SubnetType;
use std::future::Future;
use tokio::runtime::Runtime as TRuntime;

mod create_group_tests;
mod change_role_tests;
mod get_updates_tests;
mod make_super_admin_tests;
mod mentions_tests;
mod online_users_aggregator_tests;
mod pinned_messages_tests;
mod send_cycles_tests;
mod send_message_tests;
mod verify_user_tests;

fn main() {
    let fondue_config = fondue::pot::execution::Config::default().random_pot_rng_seed();
    let pots = vec![tests_pot()];

    if let Some(res) = fondue::pot::execution::execute(&fondue_config, pots) {
        res.print_summary();
        print_rng_seed(&fondue_config);

        if !res.was_successful() {
            // propagate error in case of any failures
            std::process::exit(1);
        }
    } else {
        print_rng_seed(&fondue_config);
        std::process::exit(1);
    }
}

fn tests_pot() -> pot::Pot<IcManager> {
    composable!(
        "Tests",
        configure(),
        steps! {
            create_group_tests::create_group_tests,
            get_updates_tests::get_updates_tests,
            change_role_tests::change_role_tests,
            make_super_admin_tests::make_super_admin_tests,
            mentions_tests::mentions_tests,
            online_users_aggregator_tests::online_users_aggregator_tests,
            pinned_messages_tests::pinned_messages_tests,
            send_cycles_tests::send_cycles_tests,
            send_message_tests::send_message_tests,
            verify_user_tests::verify_user_tests
        }
    )
}

fn print_rng_seed<ManCfg>(fondue_config: &fondue::pot::execution::Config<ManCfg>) {
    println!(
        "(To reproduce this exact run, make sure to use '--seed {}')",
        fondue_config.pot_config.rng_seed
    );
}

pub fn configure() -> InternetComputer {
    InternetComputer::new().add_fast_single_node_subnet(SubnetType::System)
}

pub fn block_on<F: Future>(f: F) -> F::Output {
    let rt = TRuntime::new().unwrap_or_else(|err| panic!("Could not create tokio runtime: {}", err));
    rt.block_on(f)
}
