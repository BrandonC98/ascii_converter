
use crate::*;

struct Converter {
    pub binary: Vec<u32>,
    hexadecimal: Vec<String>,
    decimal: Vec<u8>,
    charcters: String,
}

impl Converter {
    fn new(value: &Vec<u32>) -> Self {
        let bin = value;

        Self {
            binary: bin.clone(),
            hexadecimal: binary_to_hexadecimal(&bin).unwrap(),
            decimal: binary_to_decimal(&bin).unwrap(),
            charcters: binary_to_string(&bin).unwrap(),
        }
    }
}
#[test]
fn converter_new_with_binary() {
    let input = vec![1101000, 1100101, 1101100, 1101100, 1101111];
    let converter = Converter::new(&input);
    
    let expected_decimal = vec![104, 101, 108, 108, 111];

    let expected_hexadecimal = vec![
        "68".to_string(),
        "65".to_string(),
        "6C".to_string(),
        "6C".to_string(),
        "6F".to_string(),
    ];

    assert_eq!(converter.charcters, "hello".to_string());
    assert_eq!(converter.decimal, expected_decimal);
    assert_eq!(converter.hexadecimal, expected_hexadecimal);
    assert_eq!(converter.binary, input);

}
