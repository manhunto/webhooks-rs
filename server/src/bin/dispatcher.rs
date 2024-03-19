use log::debug;
use server::logs::init_log;

fn main() {
    init_log();

    debug!("Dispatcher is running and ready for messages!");
}
