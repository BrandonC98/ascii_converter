use ascii_converter::*;

fn main() {

    let input = vec! [1001000, 1100101, 1101100, 1101100, 1101111, 100000, 1110111, 1101111, 1110010, 1101100, 1100100, 100001];

    println!("{}", binary_to_string(&input).unwrap());
}