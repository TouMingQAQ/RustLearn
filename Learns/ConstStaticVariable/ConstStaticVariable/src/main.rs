//静态变量 static
//静态变量可使用unsafe修改
static STATIC_NUM : i32 = 10;
static mut STATIC_MUT_NUM : i32 = 12;
fn main() {

    //常量 const
    //常量是块级作用域
    //常量需要指定类型
    const SECOND_HOUR : usize = 3_600;
    const SECOND_DAY : usize = 24 * SECOND_HOUR;//在编译的时候就确定了
    {
        const SE : usize = 10_10;
        println!("{SE}");
    }
    //println!("{SE}"); 无法使用SE，SE只在块作用域内
    
    println!("Static:{STATIC_NUM}");
    //STATIC_NUM = 12; 无法直接修改,且因为是声明不可变
    unsafe {
        //STATIC_NUM = 12; 
        STATIC_MUT_NUM = 14;
        println!("Static mut num:{STATIC_MUT_NUM}");
    }
    // println!("{STATIC_MUT_NUM}"); //在unsafe修改后无法在safe中使用
 
}
