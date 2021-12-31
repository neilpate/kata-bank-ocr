use std::ops::Range;

const  NUM_CHARS_PER_DIGIT_ROW: usize = 3;

fn main() {
    println!("Welcome to Bank OCR kata");

    let line1 = "    _  _     _  _  _  _  _ " ;
    let line2 = "  | _| _||_||_ |_   ||_||_|";
    let line3 = "  ||_  _|  | _||_|  ||_| _|";

    let digit_1: &str = &(   
        "   ".to_owned()  +  
        &"  |" + 
        &"  |"
    );
        
    let digit_2: &str = &(   
    " _ ".to_owned() +
    " _|"            + 
    "|_ "
    );




let _some_string = "hello".to_owned() + &"world";

let _some_str: &str = &("hello".to_owned() + "world" + "again");


    let dig = get_digit(4, line1, line2, line3);
    print_digit(&dig);

    let strings_match = dig == digit_2;

    if strings_match{
        println!("Digit is a 2")
    }
    else {
        println!("Digit is not a 2");
    }

}

fn print_digit(digit: &String){

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