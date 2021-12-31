//use std::{ops::Range, collections::HashSet};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;
use std::slice::SliceIndex;

const  NUM_CHARS_PER_DIGIT_ROW: usize = 3;
const NUM_DIGITS_PER_ACC_NUM: usize = 9;

fn main() {
    println!("Welcome to Bank OCR kata");

    let line1 = "    _  _     _  _  _  _  _ " ;
    let line2 = "  | _| _||_||_ |_   ||_||_|";
    let line3 = "  ||_  _|  | _||_|  ||_| _|";

 //   let digit: HashMap<&str, i32> = HashMap::new();

    let digit_1: &str = &(   
        "   ".to_owned()    +  
        "  |"               + 
        "  |"
    );
        
    let digit_2: &str = &(   
        " _ ".to_owned() +
        " _|"            + 
        "|_ "
        );

    let digit_3: &str = &(   
        " _ ".to_owned() +
        " _|"            + 
        " _|"
        );

    let digit_4: &str = &(   
        "   ".to_owned() +
        "|_|"            + 
        "  |"
        );    
        
    let digit_5: &str = &(   
        " _ ".to_owned() +
        "|_ "            + 
        " _|"
        ); 
        
    let digit_6: &str = &(   
        " _ ".to_owned() +
        "|_ "            + 
        "|_|"
        ); 

    let digit_7: &str = &(   
        " _ ".to_owned() +
        "  |"            + 
        "  |"
        ); 

    let digit_8: &str = &(   
        " _ ".to_owned() +
        "|_|"            + 
        "|_|"
        );
        
    let digit_9: &str = &(   
        " _ ".to_owned() +
        "|_|"            + 
        " _|"
        );  

let mut digits = HashMap::new(); 

digits.insert(digit_1, 1);
digits.insert(digit_2, 2);
digits.insert(digit_3, 3);
digits.insert(digit_4, 4);
digits.insert(digit_5, 5);
digits.insert(digit_6, 6);
digits.insert(digit_7, 7);
digits.insert(digit_8, 8);
digits.insert(digit_9, 9);




 //   let digits = setup_digits();

        for i in 0..NUM_DIGITS_PER_ACC_NUM{
            let current_digit = get_digit(i, line1, line2, line3);
           // print_digit(&current_digit);
            print!("{}", detect_digit(&digits, &current_digit));

        }


}

//fn setup_digits() -> HashMap<&str, i32>{
//   
//        return digits
//}

fn detect_digit(digits: &HashMap<&str, i32>, digit: &str) -> i32{
    
    let mut value_outer = -1;
 
    match digits.get(digit){
        Some(value) => {
          //  println!("Digit found: {}", value);
            value_outer = *value;
        }
        
        _ => print!("") //println!("Not found"),
    }


    return value_outer

}

fn print_digit(digit: &str){

    for i in 0..3{
        let range = Range { start: i*NUM_CHARS_PER_DIGIT_ROW, end: i*NUM_CHARS_PER_DIGIT_ROW+NUM_CHARS_PER_DIGIT_ROW };
        println!("{}", &digit[range]);
    }
  
}

fn get_digit(index: usize, line1: &str, line2: &str, line3: &str) -> String{
 //   let temp1 = line1[1];
    
    let start: usize = index * 3;
    let stop: usize = start + 3;

    let range1 = Range { start: start, end: stop };
    let range2 = Range { start: start, end: stop };
    let range3 = Range { start: start, end: stop };

    let temp1 = &line1[range1];
    let temp2 = &line2[range2];
    let temp3 = &line3[range3];

    let temp4 = temp1.to_owned() + temp2 + temp3;

    return temp4
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_pass() {
        assert_eq!(2 + 2, 4);
    }
}