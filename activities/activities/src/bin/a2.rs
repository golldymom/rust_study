// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn sum(a:i32, b:i32) -> i32{
    a+b
}

fn dis_re(result:i32){
    println!("{:?}", result)
}

fn main() {
  let result = sum(2,2);
  dis_re(result);
}

//내가 한것, 함수를 활용하지 못함.

// fn main() {
//     let sum = 2+4;
//     println!("{:?}", sum)
// }
