mod store;

pub use store::store2;

pub fn init() {
    let result = store2::save();
    println!("Result is: {:?}", result);
}
