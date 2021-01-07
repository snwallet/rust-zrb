use std::collections::HashMap;
use std::collections::HashSet;

fn main(){
    //容器中提供了通用的数据结构的实现：向量(Vector)、哈希表(HashMap)、哈希集合(HashSet)
    //向量跟数组类似，只是长度可变
    //可以使用Vec::new()来创建，也可以使用vec!()宏来创建
    //向量中的方法push()：追加, remove()：删除, contains()：查询是否包含, len()：查长度
    //push
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);

    println!("size of vector is :{}", v.len());
    println!("{:?}", v);

    let v = vec![1, 2, 3];
    println!("{:?}", v);
    //remove
    let mut v1 = vec![1, 2, 3, 4];
    v1.remove(1);
    println!("{:?}", v1);

    //contains()
    let v2 = vec![10, 20, 30];
    if v2.contains(&20){
        println!("found 20");
    }

    println!("{:?}", v2);
    //len
    println!("the size of v2 is {}", v2.len());

    let mut v3 = Vec::new();
    v3.push(20); v3.push(30); v3.push(40);
    //向量实现了迭代器特质，可以直接使用for in来遍历向量
    for i in v3 {
        println!("{}", i);

    }
    // println!("{:?}", v3);运行出错，因为向量转让了所有权
    //解决的方法是：for i in &v3 迭代向量的一个引用


    //哈希表 HashMap
    //里面数据的存储是键值对，不允许有相同的键，可以允许有相同的值
    //需要先显式导入std::collections模块，使用new()来创建一个实例
    // use std::collections::HashMap;

    //insert()方法可用于插入或者更新一个键值对到哈希表中
    //如果存在就更新然后返回旧的键值对，不存在就插入
    let mut hash_code = HashMap::new();
    hash_code.insert("name","zheng");
    hash_code.insert("size", "https://www.zheng1224.github.io");

    
    //len()返回哈希表的长度
    println!("size of map is {}", hash_code.len());
    println!("{:?}", hash_code);

    //get()用于根据键值从哈希表中获取相应的值
    match hash_code.get(&"name"){
        Some(value) => {
            println!("value for key name is {}", value);
        },
        None => {
            println!("no found");
        }
    }

    //iter()方法会返回哈希表中键值对的引用组成的无序迭代器
    for (key, val) in hash_code.iter(){
        println!("key: {}, val: {}", key, val);
    }
    
    //contains_key()用于判断哈希表中是否包含指定的键值对
    //如果有指定的键，则返回true
    if hash_code.contains_key(&"name") {
        println!("found key");
        
    }

    //remove()用于删除指定键值对，返回的是被删除的键值对
    println!("the length of map before remove is {}", hash_code.len());
    let value = hash_code.remove(&"name");
    println!("{:?}", value);
    println!("the length of map after remove is {}", hash_code.len());


    //哈希集合 HashSet
    //没有重复值的相同数据类型的值的集合
    //insert()如果存在则返回false,不存在则返回true
    //use std::collections::HashSet;

    let mut language = HashSet::new();
    language.insert("Python");
    language.insert("Rust");
    language.insert("Ruby");
    language.insert("PHP");

    language.insert("PHP");//插入失败，不会报异常
    println!("{:?}", language);

    //iter()用于返回集合中所有元素组成的无序迭代器
    for lang in language.iter(){
        println!("{}", lang);
    }

    //get()用于获取特定值的一个引用，不存在则返回None
    match language.get(&"Rust") {
        Some(value) =>{
            println!("found {}", value);
        },
        None => {
            println!("not found");
        }
    }

    println!("{:?}", language);


    //contains()方法用于判断集合是否包含指定的值
    if language.contains(&"Rust"){
        println!("found Rust");
    }

    //remove() 删除之前如果集合中存在则返回true，否则返回false
    println!("length of the HashSet: {}", language.len());
    language.remove(&"Python");
    println!("length of the HashSet after remove: {}", language.len());
}