fn main() {
    //Int类型
    let i8 : i8 = -8;
    let i16 : i16 = 16;
    let i32 : i32 = -32;
    let i64 : i64 = 64;
    let i128 : i128 = -128;
    println!("i8 Max:{} Min:{} Size:{}",i8::MAX,i8::MIN,std::mem::size_of::<i8>());
    println!("i16 Max:{} Min:{} Size:{}",i16::MAX,i16::MIN,std::mem::size_of::<i16>());
    println!("i32 Max:{} Min:{} Size:{}",i32::MAX,i32::MIN,std::mem::size_of::<i32>());
    println!("i64 Max:{} Min:{} Size:{}",i64::MAX,i64::MIN,std::mem::size_of::<i64>());

    //无符号int
    // let uint8 : u8 = -8; error
    let uint8 : u8 = 8;
    let uint16 : u16 = 16;
    let uint32 : u32 = 32;
    let uint64 : u64 = 64;
    let uint128 : u128 = 128;
    println!("u8 Max:{} Min:{} Size:{}",u8::MAX,u8::MIN,std::mem::size_of::<u8>());
    println!("u16 Max:{} Min:{} Size:{}",u16::MAX,u16::MIN,std::mem::size_of::<u16>());

    //usize 和 isize
    //其大小由平台决定
    let isize : isize = -1231451231;
    let usize : usize = 1231451231;
    println!("isize Max:{} Min:{} Size:{}",isize::MAX,isize::MIN,std::mem::size_of::<isize>());
    println!("usize Max:{} Min:{} Size:{}",usize::MAX,usize::MIN,std::mem::size_of::<usize>());

    //float类型
    let f32 : f32 = 12.316;
    let f64 : f64 = 12.315;
    println!("f32 Max:{} Min:{} Size:{}",f32::MAX,f32::MIN,std::mem::size_of::<f32>());
    println!("f64 Max:{} Min:{} Size:{}",f64::MAX,f64::MIN,std::mem::size_of::<f64>());
    
    //五舍六入
    println!("{:.2}",f32);
    println!("{:.2}",f64);
    
    let num = (100.0 * f64).round()/ 100.0;
    println!("{:.2}",num);
    format!("{:.2}",num);
    //bool类型
    let b1 : bool = true;
    let b2 : bool = false;
    
    //char类型
    //支持Unicode字符
    //标识Char类型用单引号
    let c1 : char = 'A';
    // let c1 : char = "A";//不能用双引号给单字符
    
    
    let a1 = 0x1e0f4;//十六进制
    println!("十六进制：{:X}",a1);
    let a2 = 0o7;
    println!("{a2}");
    
}
