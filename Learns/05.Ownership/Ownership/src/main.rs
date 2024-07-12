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
    let str3 = str2.clone();//深拷贝
    println!("{str2}");
    get_Length(str2);//传入函数后，在函数结束时就销毁了
    // println!("{str2}");//Error
    println!("{}",first_word("Hello World"));
}
fn get_Length(str:String)->usize
{
    // return str.len();
    str.len()//不输入;时，自动return最后一个函数的值
}
fn first_word(s:&str)->&str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if(item == b' ')
        {
            return &s[0..i];
        }
    }
    &s[..]
}