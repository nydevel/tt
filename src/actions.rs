use chrono::{Utc};
pub use super::store::orm;

pub mod action{
    pub fn start (){
        let now = super::Utc::now();
        super::orm::create(&now.format("%Y-%m-%d %H:%M:%S").to_string());
    }
}

//==========================Tests==========================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start() {
        assert_eq!(action::start(), 1);
    }
}