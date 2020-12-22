mod store;
mod actions;

pub use store::config;
pub use actions::action;

pub fn init() {
    config::init();
}

pub fn start() {
    action::start();
}
