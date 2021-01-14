fn main(){
    //切片(slice)其实就是指向一段内存的指针，可用于访问内存块中连续区间的数据
    //能够连续存储数据的数据结构有：数组，向量，字符串

    //和字符串使用，通过【前索引，后索引】的方式来访问特定区间的字符串
    let n = "Tutorial".to_string();
    let sl = &n[4..n.len()];
    println!("{}", sl);
    // println!("{}", n);   //切片不会改变原始数据的内容

    //切片作为函数参数
    let arr = [10, 20, 30, 40, 50];
    use_slice(&arr[0..4]);

    //更改切片内容
    let mut data = [10, 20, 30, 40, 50];
    slice(&mut data[1..4]);
    println!("{:?}", data);
}

fn use_slice(slice: &[i32]){
    println!("length of slice is {:?}", slice.len());
    println!("{:?}", slice);
}

fn slice(data:&mut [i32]){
    println!("{:?}", data);
    data[0] = 1010;
}