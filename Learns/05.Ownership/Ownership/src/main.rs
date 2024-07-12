fn main() {
    //对于简单的赋值操作，比如
    let n = 5;
    let n2 = n;
    //此时默认执行的是Copy操作，只Copy数据给新的变量

    //其他类似Struct，String等，默认执行Move操作
    //也就是移交所有权
    //个人理解所有权是指针外加一个使用权，每个变量尽管有指针但没使用权，则无法修改变量
    //当执行Move操作时，会Copy指针外加移动使用权给新的变量
    
    let str1 = String::from("Hello world");
    let str2 = str1;//移交所有权
    // println!("{}",str1)//error 没有所有权
}
