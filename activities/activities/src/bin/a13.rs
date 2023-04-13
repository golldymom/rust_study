//ex
// let my_nums = vec![1,2,3];

// let mut my_nums = Vec::new();
// my_nums.push(1);
// my_nums.push(2);
// my_nums.push(3);
// my_nums.pop();
// my_nums.len();

// let two = my_nums[1];

/*
vec는 백터 메크로를 사용할때 쓰는 것.
벡터 메크로는 벡터에 추가하려는 항목을 []대괄호 안에 넣는다.
데이터는 모두 같은 자료형이여야 하기 때문에 예제에서는 모두 숫자형이다.
대안으로 mut(가변) 변수를 생성할 수 있다.
이때 let my_nums와 let mut my_nums가 같은가? => 비슷하게 할 수 있다.
Vec를 사용할때 ::를 사용해 호출할 함수를 결정함.
Vec안에 new 함수가 구현되어 있음.
빈 Vec안에 데이터를 넣을때는 push를 사용.
이렇게 1,2,3을 넣어주면 vec!와 같아짐.
pop 매서드를 사용하면 맨 뒤에 있는 데이터를 꺼냄. 제거하는 것.
len 함수는 lenght의 줄임, Vec 속 항목들의 수를 반환
push(3)까지 에서 len을 사용하면 3, pop을 사용한 뒤 len을 사용하면 2

let two는 슬라이스 표기법을 사용한 것.
슬라이스 표기법인 인덱스 번호를 대괄호로 감싸면 Vec항목에 접근할 수 있음.
Vec의 인덱스도 0에서 시작함.
*/

// let my_nums = vec![1,2,3];

// for num in my_nums {
//     println!("{:?}", num);
// }
/*
for in 구문
for 변수 num에 대해 in colections인 my_nums에서 중괄호 사이의 몇가지 액션을 한다.
코드 블럭은 벡터의 모든 단일 요소에 대해 실행됨.
*/
/*
요약
벡터는 여러개의 유사한 데이터를 포함 할 수 있다.
vec!는 매크로이다. 매크로를 사용하면 코드에서 벡터를 쉽게 생성할 수 있다.
for in을 ㅅ용하면 벡터의 항목을 반복(iterate)할 수 있다.
*/

//demo
//벡터를 사용해 코드 내에서 유사한 데이터를 구성하는 방법 시연(테스트 점수 추적 프로그램)

// struct Test {
//     score: i32,
// }

// fn main(){
//     let my_sco = vec![
//         Test { score:90},
//         Test { score:80},
//         Test { score:70},
//         Test { score:99},
//     ];
//     for test in my_sco{
//         println!("score = {:?}", test.score);
//     }
// }
/*
for test에서 test부분은 사용자 마음대로 지정할 수 있음.
in my_sco에서 my_sco는 항상 collection이어야 함.
test는 my_sco의 vec!이기 때문에 Test{score:...를 모두 돔. 
와중에 test.score이기 때문에 struct Test의 score인 숫자 부분만 가지고 옴.
순서를 보장하기 때문에 90,80,70,99순서로 터미널에 출력 됌.
*/

// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let my_nums = vec![10,20,330,40];

    for num in &my_nums {
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
    }
    println!("number of element = {:?}", &my_nums.len());
}

/*

*/
