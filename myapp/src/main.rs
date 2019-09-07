
use mylib::util::split_response;

fn main() {
    println!("Hello, world!");
    let (s1, s2) = split_response("Hello,World");
    println!("{} {}", s1, s2);
}
