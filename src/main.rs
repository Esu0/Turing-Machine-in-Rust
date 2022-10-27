mod analyzer;
mod machine;

use machine::TuringMachine;
fn main() {
    let tm = TuringMachine::from_file("./code.txt");
    println!("{:?}", tm);
    println!("{}", tm.is_accepted("0010000".to_string()));
}
