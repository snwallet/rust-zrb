fn main(){
    //迭代器是用来遍历collection的
    //包括数组、向量、哈希表
    //Iterator特质有两个函数必须实现
    //iter()用于返回一个迭代器对象，迭代器中存储的值，称之为项
    //next()返回迭代器中的下一个元素，如果到达末尾则返回None

    let a = [19, 20, 21, 22];
    let iter = a.iter();
    println!("{:?}", iter);

    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());

    for i in iter{
        print!("{}\t", i);
    }

}