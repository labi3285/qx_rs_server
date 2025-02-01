
pub mod err;
pub mod def;
pub mod util;
pub mod env;
pub mod time;
pub mod log;

pub mod http_server;
pub mod router;

pub mod template;
pub mod req;
pub mod json;
pub mod middleware;

pub fn setup() {
    env::setup();
    log::setup();
}