use std::env::{args, Args};

pub fn _calculator(){
     // Argument understanding
     let mut args : Args = args();
     let first_number = args.nth(1).unwrap();
     let operator = args.nth(0).unwrap().chars().next().unwrap();
     let second_number= args.nth(0).unwrap();
 
     let first_operator = first_number.parse::<f32>().unwrap();
     let second_operator = second_number.parse::<f32>().unwrap();
 
     println!("{:?} {:?} {:?}", first_operator,operator,second_operator);
 
     let result = _operate(operator,first_operator,second_operator);
     println!("{:?}", _output(first_operator, operator, second_operator, result));
}

fn _operate(operator:char, first_operator:f32, second_operator:f32) -> f32 {
    /*  if operator == '+' {
           first_operator + second_operator
      }else if operator == '-' {
           first_operator - second_operator
      }else if operator == '/' {
           first_operator / second_operator
      }else if operator == '*'{
           first_operator * second_operator
      }else {
           0.0
      }*/
  
      match operator {
          '+' => first_operator + second_operator,
          '-' => first_operator - second_operator,
          '/' => first_operator / second_operator,
          '*' | 'X' | 'x' => first_operator * second_operator,
          _ => panic!("Invalid operator used")
      }
  }
  
  fn _output(first_operator:f32,operator:char, second_operator:f32, result:f32) -> String {
      format!("{} {} {} = {}", first_operator, operator, second_operator, result)
  }
  