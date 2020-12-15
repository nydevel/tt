mod store;

pub use store::config;

pub fn init() {
    config::init();
}
