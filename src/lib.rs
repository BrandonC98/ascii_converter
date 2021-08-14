//! This project is a library for converting between different Ascii representations in the Rust language.
//! This is made for Rust programs that need to convert an ascii value.
//! This library has methods for converting any of the supported representations to any other.
//! 
//! ---
//! 
//! each supported ascii representation has it's type listed below:
//! 
//! - Binrary = `Vec<u32>`.
//! 
//! - Decimals = `Vec<u8>`.
//! 
//! - characters = `String`.
//! 
//! ---
//! 
//! Each method uses the `Result` enum for the return type so this will need to be unwrapped to get the actual value.
//! An example of how best to deal with the Results enum is below:
//! ```
//! use ascii_converter::*;
//! 
//! let input = vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];
//!
//! match decimals_to_string(&input){
//!     Ok(num) => println!("* Output: {}", num),
//!     Err(e) => println!("* Error: {}", e),
//! };
//! ```


pub fn decimal_to_hexadecimal(decimal: Vec<u8>) -> Result<Vec<String>, String>{

    let mut vec = Vec::new();

    for i in decimal.iter() {


        if i > &126 {
            return Err("this function doesn't support values over 126".to_string());
        }
        
        vec.push(dec_to_hex(*i));

    };

    Ok(vec)
    
}

 pub fn hexadecimal_to_decimal(hexadecimals: Vec<String>) -> Result<Vec<u8>, String>{

    let mut decimal: Vec<u8> = Vec::new();

    for i in hexadecimals.iter() {

        match hex_to_dec(i) {
            Ok(d) => (decimal.push(d)),
            Err(e) => return Err(e.to_string()),
        };
        
    };

    return Ok(decimal);
    
 }

 pub fn hexadecimal_to_binary(hexadecimals: Vec<String>) -> Result<Vec<u32>, String>{
    let mut binary = Vec::new();

    for i in hexadecimals.iter() {

        match hex_to_dec(i) {
            Ok(d) =>  binary.push(dec_to_bit(d)),
            Err(e) => return Err(e.to_string()),
        };

    };

    Ok(binary)

 }

 pub fn binary_to_hexadecimal(bin: &Vec<u32>) -> Result<Vec<String>, String> {
    let mut hexadecimal = Vec::new();

    for i in bin.iter() {
        match bits_to_dec(i) {
            Ok(d) => hexadecimal.push(dec_to_hex(d )),
            Err(e) => return Err(e),
            
        }
    };

    Ok(hexadecimal)

}

pub fn string_to_hexadecimal(text: String)  -> Result<Vec<String>, String> {
    let mut hexadecimal = Vec::new();

    for c in text.chars(){

        if !c.is_ascii() {
            return Err("A character in the string isn't apart of the ascii table".to_string());
        }

        hexadecimal.push(dec_to_hex(c as u8));
    }

    Ok(hexadecimal)

}

/// This function returns a string's decimal values
/// 
/// takes a string and finds the decimal assciated with that character according to the Ascii table.
/// each decimal is pushed into a `Vec<u8>` .
/// 
/// charcters inside the string should only be in the ascii range  of `32 - 126` . any other
/// character will cause an error.
/// 
/// example of unsupported characters: `£` , `☢️` , `Æ`
/// 
/// # Example
/// ```
/// use ascii_converter::*;
/// 
/// let  expected = vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];
///    
/// assert_eq!(string_to_decimals("Hello world!").unwrap(), expected);
/// ```
pub fn string_to_decimals(text: &str) -> Result<Vec<u8>, String>{   

    let mut vec = Vec::new();

    for c in text.chars(){

        if !c.is_ascii() {
            return Err("A character in the string isn't apart of the ascii table".to_string());
        }

        vec.push(c as u8);
    }

    Ok(vec)
}

/// This function returns a binary representations of decimal numbers
/// 
/// this function takes a `Vec<u8>` , it's elements should represent ascii characters. 
/// An error will be thrown if the a value is above `126` as this is the end of the ascii range. 
/// 
/// # Example
/// ```
/// use ascii_converter::*;
/// 
/// let  hello_world = vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];
/// let expected = vec![1001000, 1100101, 1101100, 1101100, 1101111, 100000, 1110111, 1101111, 1110010, 1101100, 1100100, 100001];
///    
/// assert_eq!(decimals_to_binary(&hello_world).unwrap(), expected);
/// ```
pub fn decimals_to_binary(dec_vec: &Vec<u8>) ->  Result<Vec<u32>, String>{
    
    let mut binary = Vec::new();

    for i in dec_vec.iter() {
        if i > &126 {
            return Err("this function doesn't support values over 126".to_string());
        }

        binary.push(dec_to_bit(*i));            
    }

    Ok(binary)
}

/// This function takes in binary numbers and will return the decimal version.
/// 
/// This function takes a `Vec<u8>`, it's elements should represent binary values.  
/// if a value passed in contains a digit that  isn't `1` or `0` an error will be thrown.
/// 
/// # Example
/// ```
/// use ascii_converter::*;
/// 
/// let hello_world = vec![1001000, 1100101, 1101100, 1101100, 1101111, 100000, 1110111, 1101111, 1110010, 1101100, 1100100, 100001];
/// let  expected = vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];
///    
/// assert_eq!(binary_to_decimal(&hello_world).unwrap(), expected);
/// ```
pub fn binary_to_decimal(bin_vec: &Vec<u32>) -> Result<Vec<u8>, String>{

    let mut decimals = Vec::new();

    for b in bin_vec.iter(){

        for c in b.to_string().chars() {
            if c != '0' && c != '1' {
                return Err("number passed in isn't binary".to_string());
            }
        }

        match bits_to_dec(b) {
            Ok(t) => (decimals.push(t)),
            Err(_) => return Err("FAIL".to_string()),
        };
    }

    Ok(decimals)
}

/// This function takes in Decimal numbers and will return the string they represent 
/// 
/// this function will takes a `Vec<u8>` and changes each element to a `char` then pushs it into a string.
/// if a element of the vec passed in is below 32 or above 126 it will cause an error to be thrown.
/// 
/// # Example
/// ```
/// use ascii_converter::*;
/// 
/// let hello_world = vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];
///    
/// assert_eq!(decimals_to_string(&hello_world).unwrap(), "Hello world!".to_string());
/// ```
pub fn decimals_to_string(dec_vec: &Vec<u8>) -> Result<String, String>{

    let mut text = String::new();

    for d in dec_vec.iter(){

        if !(d >=  &32 && d <= &126) {
            return Err("the number is outside the ascii range".to_string());
        }

        text.push(*d as char);
    }

    Ok(text)
}

/// This function returns the binary numbers of each letter passed in. 
/// 
/// This function takes each `char` of the `&str` passed in and converts it to a binary number which is represented
/// as a `u32`. These are then pushed into a `Vec<u32>`. 
/// 
/// # Example
/// ```
/// use ascii_converter::*;
/// 
/// let expected = vec![1001000, 1100101, 1101100, 1101100, 1101111, 100000, 1110111, 1101111, 1110010, 1101100, 1100100, 100001];
/// 
/// assert_eq!(string_to_binary("Hello world!").unwrap(), expected);
/// ```
pub fn string_to_binary(text: &str) -> Result<Vec<u32>, String>{

    let dec = string_to_decimals(text);

    let dec = match dec {
        Ok(new_dec_vec) => new_dec_vec,
        Err(error) => return Err(error.to_string()), 
    };

    let bin = decimals_to_binary(&dec);

    match bin {
        Ok(ref bin_vec) => bin_vec,
        Err(error) => return Err(error),
    };

    return bin;

}

/// This function returns a string made from the binary values passed to it.
/// 
/// This function will take a `Vec<u32>`, each `u32` element will be converted to
/// its a `char` value and then return as a `string`.
/// 
/// # Example
/// ```
/// use ascii_converter::*;
/// 
/// let input = vec! [1001000, 1100101, 1101100, 1101100, 1101111, 100000, 1110111, 1101111, 1110010, 1101100, 1100100, 100001];
/// 
/// assert_eq!(binary_to_string(&input).unwrap(), "Hello world!".to_string());
/// ```
 pub fn binary_to_string(bin: &Vec<u32>) -> Result<String, String>{
    
     let dec = binary_to_decimal(&bin);

     let dec = match dec {
         Ok(new_dec_vec) => new_dec_vec, 
         Err(error) => return Err(error.to_string()),
     };

     let string = decimals_to_string(&dec);

      match string {
          Ok(ref dec_string) => dec_string,
          Err(error) => return Err(error.to_string()),
      };

      return string;
 }

 fn dec_to_hex(dec: u8) -> String{
    format!("{:X}", dec)

 }


 fn dec_to_bit(dec: u8) -> u32 {
    let n = &format!("{:b}", dec);
    let n: u32  = n.trim().parse().unwrap();
    n
}

 fn bits_to_dec(bit: &u32) -> Result<u8, String> {

    match u8::from_str_radix(&bit.to_string(), 2) {
        Ok(dec) => Ok(dec),
        Err(_) => Err("error".to_string()),
    }

}

pub fn hex_to_dec(hex: &String) ->  Result<u8, String> {

    match u8::from_str_radix(&hex, 16){
        Ok(dec) => Ok(dec),
        Err(e) => Err(e.to_string()),
    }
    

}

#[cfg(test)]
mod tests{

    mod string_to_hexadecimal_tests {
        use super::super::*;

        #[test]
        fn string_to_hexadecimal_test_happy_path() {
            let input = "hello".to_string();
            let expected = vec!["68".to_string(), "65".to_string(), "6C".to_string(), "6C".to_string() , "6F".to_string()];

            assert_eq!(string_to_hexadecimal(input), Ok(expected));

        }

        #[test]
        fn string_to_hexadecimal_test_unhappy_path() {
            let input = "☢️".to_string();

            assert_eq!(string_to_hexadecimal(input), Err("A character in the string isn't apart of the ascii table".to_string()));

        }

    }

    mod binary_to_hexadecimal_tests{
        use super::super::*;

        #[test]
        fn binary_to_hexadecimal_test_happy_path() {

            let input = vec![1101000, 1100101, 1101100, 1101100, 1101111];
            let expected = vec!["68".to_string(), "65".to_string(), "6C".to_string(), "6C".to_string() , "6F".to_string()];

            assert_eq!(binary_to_hexadecimal(&input), Ok(expected));

        }

        #[test]
        fn binary_to_hexadecimal_test_unhappy_path() {

            let input = vec![86, 1100101, 1101100, 1101100, 1101111];

            assert_eq!(binary_to_hexadecimal(&input), Err("error".to_string()));

        }

    }

    mod hexadecimal_to_binary_tests{
        use super::super::*;

        #[test]
        fn hexadecimal_to_binary_test_happy_path() {

            let input = vec!["68".to_string(), "65".to_string(), "6C".to_string(), "6C".to_string() , "6F".to_string()];
            let expected  = vec![1101000, 1100101, 1101100, 1101100, 1101111];

            assert_eq!(hexadecimal_to_binary(input), Ok(expected));

        }

        #[test]
        fn hexadecimal_to_binary_test_unhappy_path() {

            let input = vec!["68".to_string(), "65z".to_string(), "6C".to_string(), "6C".to_string() , "6F".to_string()];

            assert_eq!(hexadecimal_to_binary(input), Err("invalid digit found in string".to_string()));

        }
    }

    mod hexadecimal_to_decimal_tests{

        use super::super::*;

        #[test]
        fn hexadecimal_to_decimal_test_happy_path(){

            let input = vec!["68".to_string(), "65".to_string(), "6C".to_string(), "6C".to_string() , "6F".to_string()];
            let  expected = vec![104, 101, 108, 108, 111];

            assert_eq!(hexadecimal_to_decimal(input), Ok(expected));

        }

        #[test]
        fn hexadecimal_to_decimal_test_unhappy_path(){

            let input = vec!["68".to_string(), "65sdf".to_string(), "6C".to_string(), "6C".to_string() , "6F".to_string()];

            assert_eq!(hexadecimal_to_decimal(input), Err("invalid digit found in string".to_string()));

        }

    }

    mod decimal_to_hexadecimal_tests{

        use super::super::*;

        #[test]
        fn decimal_to_hexadecimal_test_happy_path(){

            let  input = vec![104, 101, 108, 108, 111];
            let expected = vec!["68".to_string(), "65".to_string(), "6C".to_string(), "6C".to_string() , "6F".to_string()];

            assert_eq!(decimal_to_hexadecimal(input), Ok(expected));


        }

        #[test]
        fn decimal_to_hexadecimal_test_unhappy_path(){
            let  input = vec![127];
            
            assert_eq!(decimal_to_hexadecimal(input), Err("this function doesn't support values over 126".to_string()));
        }

        #[test]
        fn dec_to_hex_test_happy_path(){
            let expected = "A";

            assert_eq!(dec_to_hex(10), expected);
        }

    }
    
    mod string_to_decimal_tests{

        use super::super::*;

        
        #[test]
        fn string_to_decimals_test_happy_path(){
    
            let  expected = vec![104, 101, 108, 108, 111];
    
            assert_eq!(string_to_decimals("hello"), Ok(expected));
        }

        #[test]
        fn string_to_decimals_test_happy_none_alphabetical(){
    
            let  expected = vec![49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 32, 33, 36, 37, 94, 38, 42,
             40, 41, 45, 95, 61, 43, 123, 125, 91, 93, 59, 58, 64, 39, 126, 35, 60, 44, 46, 62, 47, 63];
         
            
            assert_eq!(string_to_decimals("1234567890 !$%^&*()-_=+{}[];:@'~#<,.>/?"), Ok(expected));
        }
        
        #[test]
        fn string_to_decimals_test_happy_alphabetical(){
    
            let  expected = vec![113, 119, 101, 114, 116, 121, 117, 105, 111, 112, 97, 115, 100, 102, 103,
             104, 106, 107, 108, 122, 120, 99, 118, 98, 110, 109, 81, 87, 69, 82, 84, 89, 85, 73, 79, 80,
              65, 83, 68, 70, 71, 72, 74, 75, 76, 90, 88, 67, 86, 66, 78, 77];
         
            
            assert_eq!(string_to_decimals("qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM"), Ok(expected));
        }
    
        #[test]
        fn string_to_decimals_test_unhappy_path(){
            
            assert_eq!(string_to_decimals("☢️"), Err("A character in the string isn't apart of the ascii table".to_string()));
        }

    }

    mod decimals_to_binary_tests{

        use super::super::*;

        #[test]
        fn decimals_to_binary_test_happy_path(){
    
            let  input = vec![104, 101, 108, 108, 111];
            let  expected = vec![1101000, 1100101, 1101100, 1101100, 1101111];
            
            assert_eq!(decimals_to_binary(&input), Ok(expected));
        }

        #[test]
        fn decimals_to_binary_test_unhappy_path(){
    
            let  input = vec![127];
            
            assert_eq!(decimals_to_binary(&input), Err("this function doesn't support values over 126".to_string()));
        }

        
        #[test]
        fn decimals_to_binary_test_max_num(){
    
            let  input = vec![126];
            let  expected = vec![01111110];
            
            assert_eq!(decimals_to_binary(&input), Ok(expected));
        }

        #[test]
        fn decimals_to_binary_test_min_num(){
    
            let  input = vec![0];
            let  expected = vec![0];
            
            assert_eq!(decimals_to_binary(&input), Ok(expected));
        }

    }

    mod binary_to_decimal_tests{

        use super::super::*;

        #[test]
        fn binary_to_decimal_test_happy_path(){
    
            let  input = vec![1101000, 1100101, 1101100, 1101100, 1101111];
            let  expected = vec![104, 101, 108, 108, 111];
            
            assert_eq!(binary_to_decimal(&input), Ok(expected));
        }

        #[test]
        fn binary_to_decimal_test_unhappy_path(){
    
            let  input = vec![30340];
            
            assert_eq!(binary_to_decimal(&input), Err("number passed in isn't binary".to_string()));
        }
        
    }

    mod decimals_to_string_tests{

        use super::super::*;

        #[test]
        fn decimals_to_string_test_happy_path(){
            
            let  input = vec![104, 101, 108, 108, 111];
            
            assert_eq!(decimals_to_string(&input), Ok("hello".to_string()));
        }
        
        #[test]
        fn decimals_to_string_test_unhappy_path(){
            
            let  input = vec![168];
            
            assert_eq!(decimals_to_string(&input), Err("the number is outside the ascii range".to_string()));
        }

    }
    
    mod binary_to_string_tests{
        
        use super::super::*;
        
        #[test]
        fn binary_to_string_test_happy_path(){
            
            let  input = vec![1101000, 1100101, 1101100, 1101100, 1101111];
            
            assert_eq!(binary_to_string(&input), Ok("hello".to_string()));
        }
        
        #[test]
        fn binary_to_string_test_unhappy_path_none_binary(){
            
            let  input = vec![104];
            
            assert_eq!(binary_to_string(&input), Err("number passed in isn't binary".to_string()));
        }
        
        #[test]
        fn binary_to_string_test_unhappy_path_out_of_range(){
            
            let  input = vec![1];
            
            assert_eq!(binary_to_string(&input), Err("the number is outside the ascii range".to_string()));
        }
    }

    mod string_to_binary_tests{
        
        use super::super::*;
        
        #[test]
        fn string_to_binary_test_happy_path(){
            
            let  expected = vec![1101000, 1100101, 1101100, 1101100, 1101111];
            
            assert_eq!(string_to_binary("hello"), Ok(expected));
        }

        #[test]
        fn string_to_binary_test_unhappy_path(){
                        
            assert_eq!(string_to_binary("☢️"), Err("A character in the string isn't apart of the ascii table".to_string()));
        }

    }

}
