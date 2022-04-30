
use crate::*;

struct Converter {
    pub binary: Vec<u32>,
    hexadecimal: Vec<String>,
    decimal: Vec<u8>,
    charcters: String,
}

impl Converter {
    
}

impl From<Vec<u32>> for Converter {
    fn from(value: Vec<u32>) -> Self {
        let bin = value;

        Self {
            binary: bin.clone(),
            hexadecimal: binary_to_hexadecimal(&bin).unwrap(),
            decimal: binary_to_decimal(&bin).unwrap(),
            charcters: binary_to_string(&bin).unwrap(),
        }
    }
}

impl From<Vec<String>> for Converter {

        fn from(value: Vec<String>) -> Self {
        let hex = value;

        Self {
            hexadecimal: hex.clone(),
            binary: hexadecimal_to_binary(&hex).unwrap(),
            decimal: hexadecimal_to_decimal(&hex).unwrap(),
            charcters: hexadecimal_to_string(&hex).unwrap(),
        }
    }
}

impl From<Vec<u8>> for Converter {

    fn from(value: Vec<u8>) -> Self {
    let dec = value;

        Self {
            decimal: dec.clone(),
            binary: decimals_to_binary(&dec).unwrap(),
            hexadecimal: decimal_to_hexadecimal(&dec).unwrap(),
            charcters: decimals_to_string(&dec).unwrap(),
        }
    }
}

impl From<String> for Converter {
    
    fn from(value: String) -> Self {
        let chars = value;

        Self {
            charcters: chars.clone(),
            binary: string_to_binary(&chars).unwrap(),
            hexadecimal: string_to_hexadecimal(&chars).unwrap(),
            decimal: string_to_decimals(&chars).unwrap(),
        }
    }
}

#[test]
fn converter_new_with_characters() {
    let expected_binary = vec![1101000, 1100101, 1101100, 1101100, 1101111];
    let converter = Converter::from("hello".to_string());
    
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
    assert_eq!(converter.binary, expected_binary);

}

#[test]
fn converter_new_with_binary() {
    let input = vec![1101000, 1100101, 1101100, 1101100, 1101111];
    let converter = Converter::from(input.clone());
    
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

#[test]
fn converter_new_with_hexadecimal() {

    let input = vec![
        "68".to_string(),
        "65".to_string(),
        "6C".to_string(),
        "6C".to_string(),
        "6F".to_string(),
    ];
    let expected_binary = vec![1101000, 1100101, 1101100, 1101100, 1101111];
    let converter = Converter::from(input.clone());
    
    let expected_decimal = vec![104, 101, 108, 108, 111];


    assert_eq!(converter.charcters, "hello".to_string());
    assert_eq!(converter.decimal, expected_decimal);
    assert_eq!(converter.hexadecimal, input);
    assert_eq!(converter.binary, expected_binary);

}

#[test]
fn converter_new_with_decimal() {

    let input = vec![104, 101, 108, 108, 111];
    let expected_binary = vec![1101000, 1100101, 1101100, 1101100, 1101111];
    let converter = Converter::from(input.clone());
    
    
    let expected_hexadecimal = vec![
        "68".to_string(),
        "65".to_string(),
        "6C".to_string(),
        "6C".to_string(),
        "6F".to_string(),
    ];
    
    assert_eq!(converter.charcters, "hello".to_string());
    assert_eq!(converter.decimal, input);
    assert_eq!(converter.hexadecimal, expected_hexadecimal);
    assert_eq!(converter.binary, expected_binary);

}
