// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
struct Grocery{
    quantity:i32,
    id:i32,
}

fn d_quantity(g_item:&Grocery){
    println!("quantity: {:?}", g_item.quantity);
}
fn d_id(g_item:&Grocery){
    println!("id: {:?}", g_item.id);
}
fn main() {
    let g_item = Grocery{
        quantity:4,
        id:6,
    };
    d_quantity(&g_item);
    d_id(&g_item);

}

//소유권 예제
// enum Light {
//     Bright,
//     Dull,
// }

// fn d_light(light: Light){
//     match light{
//         Light::Bright => println!("bringht"),
//         Light::Dull => println!("dull"),

//     }
// }
// fn main(){
//     let dull = Light::Dull;
//     d_light(dull);
//     d_light(dull);
    
// }
//에러 발생 let dull이 한번만 실행됨. dull 값이 첫번째 함수로 이동해서 그 데이터에 대한 값을 첫번째 함수만 갖게 됌. 그 한번 실행되고 사라지기 때문에 에러 발생함.
/* 
Light가 dull에 할당할때 소유자는 메인 함수, 메인함수의 중골호 안에 있는 어떤것이든 
main함수가 소유하는 것.
d_light(dull)을 호출하면 dull의 d_light로 이동해 d_light함수에 위치함.
데이터의 소유자는 d_light가 됌.
데이터를 소유한 모든 함수는 함수가 끝날 때 그 데이터를 삭제하도록 되어 있음.
그래서 첫번째 d_light(dull)이 끝나면 dull도 같이 삭제된다. 
두번째 d_light에선 이미 (dull)이 삭제 됐으므로 데이터가 존재하지 않아 사용할 수 없음.
중요한 점은 변수가 만들어 진 곳이 최초의 소유자가 된다.
메인에서 변수 만들어서 처음 소유권은 메인, d_light가 호출해서 두번째 소유권자가 됌.
d_light함수를 두번사용할 수 있게 하기 위해서는 "대여"해야 함.
*/

// enum Light {
//     Bright,
//     Dull,
// }

// fn d_light(light: &Light){
//     match light{
//         Light::Bright => println!("bringht"),
//         Light::Dull => println!("dull"),

//     }
// }
// fn main(){
//     let dull = Light::Dull;
//     d_light(&dull);
//     d_light(&dull);
    
// }

/*
rust에서 &는 데이터를 대여한다는 의미, 참조라고도 불림.
dull변수가 만들어 지고 main함수가 즉시 소유자가 되서 변수를 삭제할 수 있음.
처음 d_light 함수를 만들때 &로 함으로써 데이터를 대여해서 사용한다는 것을 표현함.
main 함수에서 d_light 함수를 호출할때도 ()안에 &를 씀으로써 데이터를 대여해서 한다는 것을 
알 수 있음.
데이터를 대여해서 씀으로 d_light를 2번 호출해도 아직 메인함수의 식이 다 안끝났기 때문에
두번째 d_light함수 까지 프린트를 하고 변수의 데이터가 사라짐. 변수 데이터의 소유는 main에 있기 때문에.
대여를 사용하면 d_light함수를 원하는 만큼 호출 할 수 있음.
*/

/*
정리
메모리는 수루 방지를 위해 관리되어야 함. rust는 소유권 모형을 이용해 메모리를 관리함
데이터의 소유자가 반드시 메모리를 정리해야 하며 
중골호가 닫기는 스코프의 끝에서 자동적으로 일어남.
함수를 호출할 때의 기본 행동양식은 메모리를 새로운 소유자로 옮기는 것.
데이터를 대여만 하고 싶다면 &를 써서 함수가 메모리를 대여하도록 함.
*/


//데모
/* 소유권은 rust의 메모리 관리 모형으로 여러분의 프로그램을 빠르게 하며
메모리 누수를 방지함. */

// struct Book{
//     pages : i32,
//     rating: i32,
// }

// fn d_p_count(book:Book){
//     println!("pages = {:?}", book.pages);
// }

// fn d_p_rating(book:Book){
//     println!("rating={:?}", book.rating);
// }

// fn main(){
//     let book = Book{
//         pages:5,
//         rating:9,
//     };
//     d_p_count(book);
//     d_p_rating(book);

// }
//let book = Book의 {}중괄호 사이의 코드를 스코프 혹은 블록이라고 부름
//d_p_count와 d_p_rating을 동시에 사용한 지금은 에러가 난다.
/*
Book이라는 pages와 rating을 포함하는 구조체가 있음.
첫번째 함수는 pages 필드를 참조해 페이지 수를 출려해 보여주는 함수
두번째 함수는 rating 필드에 접근해 점수를 출력해 보여주는 함수
main 함수에서 book이라는 변수를 만들어 pages, rating 숫자 부여.
프로그램의 모든 데이터는 프로그램의 다른 부분에서 소유됌.
book 변수는 main 함수에서 만들어 졌기 때문에 소유자가 main함수.
소유자인 main 함수는 메모리를 정리할 책임이 있음. 
메모리를 정맇는 행위가 헤제. 블럭의 끝에서 자동적으로 해제 됨.
블록(스코프)이 한번 실행되면 book은 자동적으로 해제 됌,
그런데 d_p_count에 book 변수오 함께 호출하면 소유권이 옮겨지게 됌.
d_p_count가 실행된 후 book 을 메모리 해젷게 됨, main에서 다시 쓸수 없음.
d_p_rating을 호출하면 데이터를 d_p_count가 써서 컴파일러가 에러 표시를 띄움.

*/

// struct Book{
//     pages : i32,
//     rating: i32,
// }

// fn d_p_count(book:&Book){
//     println!("pages = {:?}", book.pages);
// }

// fn d_p_rating(book:&Book){
//     println!("rating={:?}", book.rating);
// }

// fn main(){
//     let book = Book{
//         pages:5,
//         rating:9,
//     };
//     d_p_count(&book);
//     d_p_rating(&book);

// }
//작동함.
//이렇게 하는 이유는 효율성과 메모리 관리를 위해.
/* 
데이터 구조체가 몇 mb에 달할 정도로 컸다면 이 구조체의 소유권을 다른 함수로 옮기면
함수를 사용할 때마다 이 데이터 전체를 복사해야 함.
데이터 대여를 사용하면 데이터가 한 곳에 그대로 머물러 있고 단순히 대여만 해준뒤에
돌려받는 것이므로 훨씬 빠르게 작동함.
*/