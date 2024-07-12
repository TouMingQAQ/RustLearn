fn main() {
    //元组和数组
    //元组和数组是复合类型，列表和字典是集合类型
    //元组和数组的长度是固定的，集合类型是动态大小
    
    let arrayInt = [1,2,44,6,2];
    let mut arrayIntMut : [i16; 2] = [2,5];
    let num =arrayInt[2];
    println!("Num:{num}");
    // println!("{arrayInt[2]}"); //Error
    // arrayInt[2] = 5;//error
    arrayIntMut[1] = 3;
    println!("{}",arrayIntMut[1]);
    println!("{}",arrayInt.len());
    for num in arrayInt {
        println!("{num}");
    }
    println!("Num:{num}");
    let arraryInt = [5;0];//定义长度为5，默认值为0的数组
    
    
    //元组
    //元组是固定长度的异构结构体，其数据没有固定类型
    let tuple = (3,true,33.4);
    println!("{}",tuple.2);//元组只能通过tuple.index的方式访问
}
