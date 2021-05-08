use std::io;

struct Point{
    x: i32,
    y: i32,
}

impl Point{
    fn print(&self){
        println!("x:{},y:{}",self.x,self.y);
    }
}

fn main() {

    let a = vec![1,3,5,7,9];
    let n = 5;
    let p = Point{x:1,y:2};

    p.print();
    match getvalue(&a,n){
        Some(result) => println!("value {}",result),
        None => println!("no value {}",n),
    }
}

fn getvalue(a:&Vec<i32>, n: i32) -> Option<i32>{
    for x in a.iter(){
        if x==&n{
            return Some(n);
        }
    }
    return None;
}
