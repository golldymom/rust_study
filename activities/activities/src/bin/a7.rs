// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Color{
    Red,
    Azul,
}
// fn main() {
//     let go = Color::Red;
//     match go{
//         Color::Red => println!("it's red"),
//         Color::Azul => println!("it's blue"),

//     }
// }
fn p_c(my_c:Color){
    match my_c{
        Color::Red =>println!("red"),
        Color::Azul =>println!("blue"),

    }
}
fn main(){
    p_c(Color::Red);
}