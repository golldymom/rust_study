// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple 튜플을 리턴는 함수를 만들어라
// * Destructure the return value into two variables 해체하는 함수를 만들고 2개의 변수를 사용하라
// * Use an if..else if..else block to determine what to print 

fn coordinate() -> (i32, i32){
    (1,7)
}
fn main() {
    let (x, y)= coordinate();

    if y > 5{
        println!(">5");
    } else if y<5{
        println!("<5");
    } else{
        println!("=5");
    }
}

//튜플, tuples
//튜플 사례를 보여주기 위해 만들어진 여러 데이터 조각
//열거형
// enum Access{
//     Full,
// }
//  //튜플 소괄호에 싸여있다.
// fn one_two_three() -> (i32, i32, i32){
//     // 아래도 ()소괄호로 싸여있어서 튜플임
//     (1,2,3)
// }
// let numbers = one_two_three();
// // 해체 destrucuring
// // 함수에서 세 조각의 정보를 얻어 각 값을 개별 변수에 넣는 것.
// let (x, y, z) = one_two_three();
// println!("{:?},{:?}",x, numbers.0);
// println!("{:?},{:?}",y, numbers.1);
// println!("{:?},{:?}",z, numbers.2);

// //numbrs.ㅁ에서 .다음의 숫자는 튜플의 인덱스값. 0부터 시작함.

// let (employee, access) = ("Jake", Access::Full);
// //튜플로 작업하면 서로 다른 타입의 데이터를 섞을 수 있다.
// // 튜플은 익명의 데이터 접근을 허용하며, 여러 변수로 정보를 해체할 때 유용하고, 필드를 원하는 만큼 퐇ㅁ할 수 있다.
// //하지만 두세 개 이상의 필드로 작업을 할 때는 구조체를 사용하는 게 좋음. 왜냐 코드를 조금 더 정리된 상태로 유지하는 데 도움이 됨.

// //예제
// fn main(){
//     let coord = (2,3);
//     println!("{:?}, {:?}", coord.0, coord.1);
// // coord와 xy의 값은 같다. 위의 방식은 점방식, 아래의 방식은 하나하나 대칭되서 그냥 그 이름만 부르면 값을 알 수 있음.
//     let (x,y) = (2,3);
//     println!("{:?},{:?}", x, y);

//     let (name, age) = ("Emma", 20);
// // 안좋은 예. 변수 안에 값이 너무 많다.
//     let favorites = ("red", 14, "TX", "pizza", "TV SHOW", "home");
// // 원하는 걸 알려면 변수 설정해서 가져오려는 변수의 몇번째 인덱스인지 알아야 하는데 찾기 쉽지 않다.
//     let state = favorites.2;
//     let place = favorites.5;
// }