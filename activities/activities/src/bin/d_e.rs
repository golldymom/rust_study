//주 함수 바깥에 enum을 만듦
enum Direction{
    Left,
    Right,
    Up
}

fn main(){
    let go = Direction::Left;
    match go{
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
        Direction::Up => println!("go up"),

    }
}

//