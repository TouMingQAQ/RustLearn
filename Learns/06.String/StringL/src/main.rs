fn main() {
    //Rust中表示字符串的两种类型 String和 &str
    
    //String是一个堆分配的可变字符串类型
    
    //&str是指字符串切片引用，在 栈分配。不可变引用
    
    //String是具有所有权的，而&str没有
    let str = String::from("Hello World");
    get_length_ptr(&str);//传递的引用，没有交出所有权
    println!("{}",str);
    
    //String to &str
    let s2 = str.as_str();
    //&str to String
    let str = s2.to_string();
    
    let s3 = "\x52";
    println!("{s3}");
    let p = Person{
        name:"one".to_string(),
        age:22
    };
}
struct Person
{
    name:String,
    age:u32,
}
struct Person2<'a>
{
    name:&'a str,//这里使用切片，所以需要标注其生命周期，这里的意思是name的生命周期和person2一致
    age:u32,
}

fn get_length_ptr(str:&String)->usize
{
    str.len()
}
fn get_length(str:String)->usize
{
    str.len()
}
