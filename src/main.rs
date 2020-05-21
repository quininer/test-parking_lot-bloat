pub mod bar {
    include!(concat!(env!("OUT_DIR"), "/bar.rs"));
}

fn main() {
    let count = bar::run();

    println!("{}", count);
}
