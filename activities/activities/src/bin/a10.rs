// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print
//로직의 캡슐화

fn p_some(gt_100:bool){
    match gt_100{
        true => println!("its big"),
        false => println!("its small"),

    }
    
}

fn main() {
    let value = 100;
    let is_gt_100 = value>100;
    p_some(is_gt_100);
}


//예제

// let my_num= 3;
// let is_lt_5= if my_num <5{
//     true
// } else{
//     false
// };
// //위의 if문과 ㅇ래의 식문이 같다.
// //let my_num= 3;
// let ls_lt_5 = my_num <5;

// //match 식문
// //let my_num= 3;
// let message = match my_num{
//     1 => "hello",
//     _ => "goodbye"
// };

// //여러개 식 중첩 예제
// enum Menu{
//     Burger,
//     Fries,
//     Drink,
// }

// let paid = true;
// let item = Menu::Drink;
// let drink_type = "water";
// //중첩 식
// let order_placed = match item{
//     Menu::Drink =>{
//         if drink_type == "water"{
//             true
//         } else{
//             false
//         }
//     }
//     _ => true,
// };
// 중첩식 사용 예제
// enum Access{
//     Admin,
//     Manager,
//     User,
//     Guest,
// }

// fn main(){
//     //secret file: admins only
//     let access_level = Access::Guest;
//     let can_access_file = match access_level{
//         Access::Admin => ture,
//         _ => false,
//     }
// };