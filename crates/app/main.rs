mod app;
mod config;
mod error;

use app::launch_spc;
use app::setup;
use spc_utils::report::ok_or_critical;

fn main() {
  let config = ok_or_critical(setup());
  launch_spc(config);
}
