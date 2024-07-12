fn main() {
    let n1 = 1;
    //n1 = 2; error
    let mut n1: i16 = 2;//变为可变
    n1 = 3;//Success

    
    {
        //Rust变量定义相同名称
        //此时新定义所在的作用域内，前面的同名变量被隐藏了
        let n1 = 32;
        println!("In:{n1}");
    }
    //出了作用域后又显示了出来
    println!("Out:{n1}");

    //类型转换
    let f1 : f32 = 1.3;
    n1 = f1 as i16;

}
