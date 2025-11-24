mod crates_io;
use crate::crates_io::*;

fn main() {
    // get a client and set search-type
    let client = init_client().unwrap();
    let res = search(&client, String::from("rand"), 1);
    println!("len -> {}", res.len());
    for crat in &res {
        println!("{} ", crat.name);
    }
}


