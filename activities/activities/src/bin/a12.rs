// Topic: Implementing functionality with the impl keyword
//impl 키워드를 이용한 기능구현
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//배송상자의 특성을 출력하라, 특성에는 dimensions, weight, and color이 들억야 한다.
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
// 참곳항 구조체를 사용해서 특성들을 캡슐화 하라
// color는 열거형을 이용해야 한다.
// 새로운 상자를 만들 수 있는 기능을 구현
// 상자 구좇에 상자의 특성을 출력하는 기능을 함수로 사용한다.
// 이 기능들은 구조체에 구현되야 한다.
// 강의
enum Color {
    Brown,
    Red,
}
impl Color{
    fn print(&self){
        match self {
            Color::Brown=> println!("brown"),
            Color::Red=> println!("red"),

        }
    }
}
struct Dimenstions {
    width:f64,
    height:f64,
    depth: f64,
}

impl Dimenstions{
    fn print(&self){
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox{
    color:Color,
    weight:f64,
    dimensions:Dimenstions,
}
impl ShippingBox{
    fn new(weight:f64, color: Color, dimensions: Dimenstions) -> Self{
        Self{
            weight,
            color,
            dimensions,
        }
    }
    //struct와 완전 같은 이름을 갖고 있어서 color:color(struct의 color:Color의 color지칭)같은 이게 어떤 필드를 의미하는지 표시할 필요 없다.
    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}
//rust에서 함수 이름을 짓는 규치에 따르면 새로운 구조체, 열거형을 만드는 기능을 구현할 때 
//new라는 함수 이름을 사용한다.
fn main(){
    let small_dimensions = Dimenstions{
        width:1.0,
        height:2.2,
        depth:4.1,
    };
    let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);
    small_box.print();
}


//나
// struct boxCharct{
//     dimensions:f64,
//     weight:f64,
//     color: Color,
// }
// enum Color{
//     orange,
//     yellow,
//     black,

// }

// impl boxCharct{
//     fn newBox(){
//         println!("")
//     }
// }
// fn main() {}

//impl 기능구현
/*
impl 키워드, 어떤 열거형이나 구조쳉 기능을 구현하게 함.
코드의 구성을 더 좋게 만들고 프로그램을 더 읽기 쉽게 해줌.
*/

// struct Temperature{
//     degress_f:f64,
// }

// fn s_temp(temp:Temperature){
//     println!("{:?} degress F", temp.degress_f);
// }
// fn main(){
//     let hot = Temperature{degress_f:99.9};
//     s_temp(hot);
// }
// /*
// 화씨 온도를 호출
// s_temp 함수는 temperature를 다룰 때만 의미가 있음.

// */

// //impl 구조체

// struct Temperature{
//     degress_f:f64,
// }
// impl Temperature{
//     fn s_temp(temp:Temperature){
//         println!("{:?} degress F", temp.degress_f);
//     }

// }
// fn main(){
//     let hot = Temperature{degress_f:99.9};
//     Temperature::s_temp(hot);
// }
// 같은 결과가 나옴
/*
impl을 이용해 구현 블록을 만들려면
impl 키워드를 쓰고 다음으로 구조체 이름인 Tempeature을 씀.
중괄호로 구현 블럭에 대한 스코프를 정의하기 위해 impl 키워드 뒤에 구조체나 열거형의 이름을 사용
구현은 자주 쓰이는 기능이라 함수를 fn으로 줄여쓰는 것과 같이 impl로 줄여 씀.
기존엔 함수()이렇게 썻다면 Temperature::함수()를 사용,
구현 블럭 안의 함수에 접근할 때는 구조체나 열거형의 이름을 쓰고 ::를 쓴뒤에
호추하고자 하는 함수의 이름을 씀.

Temperature와 관련된 함수를 Temperature에 대한 구현 블럭 안에 넣음.
Temperature를 다뤄야 하때 바로 이 구현 블럭의 안에 있는 코드를 이용할 수 있음.
*/

//좀더 업그레이드 버전
// struct Temperature{
//     degress_f:f64,
// }
// impl Temperature{
//     fn s_temp(&self){
//         println!("{:?} degress F", self.degress_f);
//     }

// }
// fn main(){
//     let hot = Temperature{degress_f:99.9};
//     hot.s_temp();
// }
/*
&self를 사용해 코드를 개선함.
함수의 매개변수를 temp:Temperature대신 &self로 변경
self를 참조하는 것. 
self는 Temperature과 같이 구현 블럭에서 사용하는 구조체나 열거형을 의미
println 매크로 안에도 temp를 self로 변경.
self는 Temerature를 의마하고 이 구조체 안에 degress_f가 있음.
메인 함수 안에서 변수 이름에 .를 써서 바로 함수를 호출 할 수 있게 됌
컴파일러가 hot 변수가 Temperature인 것을 알고 우리가 이 구조체에 이 기능을
구현했음을 알기 때문.
self를 참조하면 컴파일러에서는 이 함수를 Temperature 형의 변수로 부터 호출할 수 있음을 알게 됌.
그래서 변수명 뒤에 .을 쓰면 구현 블럭 안에 있는 모든 함수에 대해 접근 할 수 있음.
*/

// struct Temperature{
//     degress_f:f64,
// }
// impl Temperature{
//     fn freezing() -> Self{
//         Self{degress_f:32.0}
//     }
//     fn boiling()-> Self{
//         Self{degress_f:212.0}
//     }
//     fn s_temp(&self){
//         println!("{:?} degress F", self.degress_f);
//     }

// }
// fn main(){
//     let hot = Temperature{degress_f:99.9};
//     hot.s_temp();

//     let cold = Temperature::freezing();
//     cold.s_temp();

//     let boiling = Temperature::boiling();
//     boiling.s_temp();
// }
/*
Self와 self차이
self의 경우 프로그램의 어딘가에서 이미 만들어진 Temperature를 의미
Self는 아직 만들어 지지 않아 새로 만드는 경우나 Temperature를 이름으로 참조할때쓰임
즉 Self는 Temperature와 같다고 보면 됌.
이 예제는 Self를 이용해 새로운 Temperatue 구조체를 만든 것.
이 구조체의 degress_f값 설정, Self는 어떤 구조체나 여러형에서 구현을 하든 그것을 뜻하게 됨.
첫번째 예제처럼 하게 되면 메인 함수의 변수도 다 바꿔줘야 함.
그런데 Self를 사용하면 변경할 필요 없이 항상 그 구조체나 열거형을 참조하게 됨
impl의 이름을 다른걸로 변경해도 내부적으로는 아무것도 변경 안해도 됌.
impl블록 안에 새로운 Temperature만듦(freezing & boiling)
메인 함수 안에서 새로운 변수를 설정하고 변수 값으로 Temerature::freezing();을 구현
::을 이용해 구현 블록의 항목을 식별. 
.을 사용할 때 self가 매개변수를 의미하게 되어 매개변수가 self가 됌.
s_temp가 &로써 참조를 사용하기 때문에 여러번 호출해도 상관 없음
여기서는 self를 대여해서 원하는 만큼 사용할 수 있는 것.
freezing과 boiling이 있으므로 이 값들이 필요하면 프로그램에서 숫자를 입력하는대신
함수를 이요앟면 됌. 
구조체나 열거형에(struct & enum) 기능을 구현하면 여기서 Tempeature오 관련된 함수들을 한 곳에
두고 새로운 구조체를 만드는 표준화된 함수를 작성할 수 있게 돼 코드를 더 쉽게 관리할 수 있게 된다.
*/