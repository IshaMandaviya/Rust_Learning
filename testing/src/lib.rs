// // #[cfg(test)]
// // mod tests {
// //     #[test]
// //     fn it_works() {
// //         let result = 2 + 2;
// //         assert_eq!(result, 4);
// //     }
// // }
// #[cfg(test)]
// mod tests{
//     use rectangle::can_hold;
//     #[test]
//     fn larger_can_hold_smaller(){
//         let larger=Rectangle{
//             width:10,
//             height:10
//         };
//         let smaller= Rectangle{
//             width:7,
//             height:7
//         };
//         assert!(larger.can_hold(&smaller));
//     }

// }
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));

}
