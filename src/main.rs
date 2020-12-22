use tt;

const MESSAGE_NO_ARGUMENTS: &str = "No arguments given";

// Initial point
// There we parse command line arguments and call modiles.
fn main() {
    let command = std::env::args().nth(1).expect(MESSAGE_NO_ARGUMENTS);

    tt::init();

    if command == "start" {
        tt::start();
    }
}

//==========================Tests==========================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "No arguments given")]
    fn test_main() {
        main();
    }
}
