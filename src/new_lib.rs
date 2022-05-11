use std::{convert::TryInto, sync::mpsc::channel};

use crate::binary_to_decimal;

fn binary_to_string(binary: &Vec<u32>) -> String{

    let mut res = String::new();

    let decimal = binary_to_decimal2(&binary);
    for i in binary.iter(){

        if *i == 1 {
            res.push('1');
        } else if *i == 0 {
            res.push('0');
        }else {
            res.push(char::from_u32(*i).unwrap());
        }
    }

   res

}

fn binary_to_decimal2(decimal: &Vec<u32>) -> Vec<u32>{

    let mut res = Vec::new();

    for i in decimal.iter() {

        let mut element = i.to_string().chars().rev().collect::<String>();
        
        let mut iter = 0;
        let mut sum = 0;
        while iter < element.len() {
            let mut num = char::to_digit(element.chars().nth(iter).unwrap(), 10).unwrap() * 2_u32.pow(iter as u32);

            sum += num;
            iter += 1;
            
        }

        res.push(sum);

    }

    res

}

fn decimal_to_binary(binary: &Vec<u32>) -> Vec<u32>{

    let mut res = Vec::new();

    for i in binary.iter() {

        let mut num = i.clone();
        let mut element = String::new();
        while num > 0{
    
            let sum = num / 2;
            let remainder = num % 2;
            num = sum;
            println!("remainder: {:?}", remainder);
            element.push(char::from_digit(remainder, 2).unwrap());

        }
        res.push(element.parse::<u32>().unwrap());

    }

    res

}

#[test]
fn test1(){

    let input = vec![15];
    let expected = vec![1111];
    assert_eq!(decimal_to_binary(&input), expected);

}

#[test]
fn test2(){

    let input = vec![1010];
    let expected = vec![10];

    assert_eq!(binary_to_decimal2(&input), expected);

}

#[test]
fn test3(){

    let input = vec![61, 1];
    let expected = String::from("=1");

    assert_eq!(binary_to_string(&input), expected);

}



