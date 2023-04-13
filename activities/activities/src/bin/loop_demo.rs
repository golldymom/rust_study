// fn main(){
//     let mut i = 3;
//     loop {
//         println!("{:?}", i);
//         i = i - 1;
//         if i == 0{
//             println!("done!");
//             break;
//             }
//     }
// }
// 위 아래 두개가 결과는 같다.
// fn main(){
//     let mut i = 3;
//     loop {
//         println!("{:?}", i);
//         i = i - 1;
//         if i == 0{
//             break;
//             }
//     }
//     println!("done!");
// }

// fn main(){
//     let mut i = 3;
//     loop{
//         if i == 0{
//             break;
//         }
//         println!("{:?}", i);
//         i = i -1;
//     }
// }

// fn main(){
//     let mut i - 1;
//     while i <= 3{
//         println!("{:?}", i);
//         i = i +1;
//     }
// }

//enum
//이넘 키워드 뒤 원하는 이름 부여
// enum Direction{
//     //입력값, 열거값(variant), 웒는 이름 작성
//     Up,
//     Down,
//     Left,
//     Right,
//     Forward,
    
//     //마지막의 열거값에서 ,는 선택형
// }
// //direcion 열거형이 네 개의 서로 다른 가능한 데이터 조각 중
// //하나일 수 있다.
// fn wh_way(go: Direction){
//     //go가 변수, 함수의 매개ㅐ변수로 들어가 있고, 이 변수에 direction을 더한 것
//     match go{
//         //매치문을 사용중이기 때문에 디렉션과 매치문을 연결시켜야 함.
//         Direction::Up => "up",
//         Direction::Down => "down",
//         Direction::Left => "left",
//         Direction::Right => "right",
//         //왼쪽이 디렉션, => 이후의 값이 매치값
//         //만약 디렉션에 forward가 있는데 매치에 적용 안하면 에럭 뜬다. 
//         //매치 문은 모든 가능한 값을 확인해야 한다는 값을 충족 못시킴.
//         //코드의 안정성을 높이는데 도움이 된다.

//     }

// }
/*
열거형은 한 번에 하나의 열것 값만 될 수 있습니다.
프로그램을 더 안정적으로 만들 수 있다. 코드읽기도 쉽게 만들어 준다.
숫자와 문장ㄹ 대신 열거형을 전달해 줄 수 있다.
*/