mod storage;

fn main() {
    let storage = storage::Storage::new();
    storage.save();
    // println!("{:#?}", storage);
    loop {
        println!("Hello, world!");
    }
}
