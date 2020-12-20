use tt;

// Initial point
// There we parse command line arguments and call modiles.
fn main() {
    let command = std::env::args().nth(1).expect("No arguments given");
    
    tt::init();

    if command == "start" {
        println!("Timer started!");
    }
}
