mod actions;
mod store;

pub use actions::action;
pub use store::config;

pub fn init() {
    config::init();
}

pub fn start() {
    action::start();
}
