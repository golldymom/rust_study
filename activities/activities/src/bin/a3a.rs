// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    // let my_bool = true;
    let my_bool = false;
    if my_bool == true{
        println!("hello")
    } else{
        println!("goodbye")
    }

}
// 내가 한것, 참거짓을 나누는 값을 어떻게 줘야 하는지 까먹음

// fn main() {
//     if a > 5{
//         println!("hello")
//     }else{
//         println!("goodbye")
//     }
// }
