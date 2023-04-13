// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// enum Drink{
//     Coke,
//     Sprit,
//     Fanta,
//     Beer,
// }
// // * Use a struct to store drink flavor and fluid ounce information
// struct Drink{ 
//     flavor:i32,
//     fluid:i32,

// }
// // * Use a function to print out the drink flavor and ounces
// fn presco(my_d:Drink){
// match my_d{
//     Drink::Coke => println!("coke"),
//     Drink::Sprit => println!("sprit"),
//     Drink::Fanta => println!("fanta"),
//     Drink::Beer => println!("beer"),
// }
// }
// // * Use a match expression to print the drink flavor

// fn main() {
//     presco(Drink::Coke);
// }
//강의풀이
enum Flavor{
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink{
    flavor: Flavor,
    fluid_oz:f64,
}

fn print_d(drink: Drink){
    match drink.flavor{
        Flavor::Sparkling =>println!("flavor: sparkling"),
        Flavor::Sweet =>println!("flavor: Sweet"),
        Flavor::Fruity =>println!("flavor: Fruity"),

    }
    println!("oz:{:?}",drink.fluid_oz);
}

fn main(){
    let sweet = Drink{
        flavor:Flavor::Sweet,
        fluid_oz:6.0,
    };
    print_d(sweet);
    let spark = Drink{
        flavor:Flavor::Sparkling,
        fluid_oz:10.1,
    };
    print_d(spark);
}




//struct는 structure의 줄임말, 여러 데이터 조각으 표함하는 데이터 타입
/* 
구조체는 있거나 아예 없거나 둘 중 하나.
구조체에 포함된 각 데이터 조각은 반드시 채워져 있어야 함
구조체의 일부만 갖거나 일부만 갖지 않을 수는 없다
구조체에 촴된 데이터 조각을 필드라고 함.

구조체는 데이터로 작업하는 것을 수월하게 만드어 줌, 
프로그램에서 유사한 데이터 끼리 묶을 수 있고 코드의 다른 부분으로 함께
이동시킬 수 있기 때문
*/
// fn main(){

//     struct ShippingBox {
//         depth: i32,
//         width:i32,
//         height:i32,
//     }
//     //구조체를 사용하는 방법, struct에서 정의된게 모두 나열 됌. 빠지면 컴ㅍ일러가 오류를 알리며 프로그램 미실행
//     let my_box = ShippingBox{
//         depth:3,
//         width:2,
//         height:5,
//     };
//     //.을 이용해 구조체의 개별 필드에 접근 할 수 있음.
//     let tall = my_box.height;
//     println!("theh box is {:?} units tall", tall);
// }

//let 의 끝나는 곳엔 무조건 ;을 붙여야 하나보다, my_box 끝에 ;를 안하니깐 실행안됌. 확실한 에러, 워닝이 아닌 에러 뜸
/*
구조체는 여러 데이터 조각을 처리함.
구조체 안에 있는 모든 필드는 구조체를 생성하기 위해 반드시 있어야 함.
점을 이용해 구조체의 모든 필드에 접근 할 수 있다.
*/

//상점을 나열해봄.
// struct GroceryItem {
//     stock: i32,
//     price: f64,
//     //f64는 소수점 쓸때 사용
// }
// fn main(){
//     let cereal = GroceryItem{
//         stock:10,
//         price: 2.99,
//     };
//     println!("cereal stock : {:?}", cereal.stock);
//     println!("cereal price : {:?}", cereal.price);
// }