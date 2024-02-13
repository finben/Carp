mod scanner;
use scanner::tokenize;

fn main() {
    let src = "abc_123 = 3
                print(hi)";
    println!("tokens {:?}", tokenize(src));
}
