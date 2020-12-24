pub use super::store::orm;
use chrono::Utc;

pub mod action {
    pub fn start() {
        let now = super::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        super::orm::create(&now);
    }
}

//==========================Tests==========================

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_start() {
//         assert_eq!(action::start(), 1);
//     }
// }
