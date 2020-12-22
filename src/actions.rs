pub mod action{
    pub fn start () -> i32{
        return 1;
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