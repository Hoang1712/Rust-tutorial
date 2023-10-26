fn main() {
    let  x  = 4;
    //x = "Hello WW"
    println!("x is : {}", x);
    {
        let x = x -2;
        println!("x is : {}", x)
    }
    let x  = x + 1;
    println!("x is : {}", x);
}
