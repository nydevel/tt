mod actions;
mod store;

pub use actions::action;
pub use store::config;

/**
 * Create config dir on program start etc.
 */
pub fn init() {
    config::init();
}

/**
 * Start time tracking
 */
pub fn start() {
    action::start();
}
