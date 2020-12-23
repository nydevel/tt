pub use super::store::orm;

pub mod action{
    pub fn start (){
        super::orm::create("test");
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