// struct add<T>{
//     a :T,
//     b :T
// }
// fn main(){
//     let a=add{a:10,b:20};
//     let b=add{a:10,b:20};
//     println!("{}",&a.a);
// }
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>{
    fn x(&self)->&T{
        &self.x
    }
}

fn main(){
    let x=Point{x:5,y:10};
    println!("{}",x.x());
}