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
    println!();

    //只读可重入迭代器
    //返回的是一个指向容器元素的引用，迭代器在遍历之后还可以使用
    let names = vec!["简单教程","简明教程", "简单教程"];
    for name in names.iter(){
        match name {
            &"简明教程" => println!("我们当中有一个异类"),
            _ => println!("Hello {}", name)
        }
    }
    println!("{:?}", names);//迭代之后容器仍然可以使用

    //自动拆箱迭代into_iter()
    //把所有迭代的值从容器中移动到一个迭代器对象中

    let n = vec![1, 2, 3];
    for i in n.into_iter(){
        match i {
            2 => println!("现在是 二"),
            _ => println!("hello {}", i)
        }
    }

    //迭代器之后容器不可在重复使用
    // println!("{:?}", n);//value borrowed here after move

    //iter_mut()返回的是容器的一个引用类型或者智能指针，
    //可以对迭代变量解引用的方式来重新赋值
    let mut str = vec!["math", "engligh", "history"];
    for s in str.iter_mut(){
        match s {
            &mut "history" => {*s = "chinese"; println!("这是修改过的值")},
            _ => println!("hello {}", s)
        }
    }

    //容器还可以使用
    println!("{:?}", str);
}