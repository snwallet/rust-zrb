use std::ops::Deref;

fn main() {

    //现代语言基础都是将数据存储在栈上，如果想存储在堆上，就要在堆上开辟内存，
    //此时就要用指针来指向内存空间，rust提供了在堆上存储数据的功能，就封装在box中
    //将栈上的数据迁移到堆上，我们称之为装箱


    //box指针也称之为装箱，把数据存储在堆上，栈上也会有包含指向堆数据的指针。
    let v_i32 = 4; //rust默认将数据存储在栈上
    let p = Box::new(v_i32); //使用box之后将数据存储在堆上
    println!("b = {}", p);

    //把数据存储在堆上之后，要访问具体的数据内容，要解引用
    //解引用和C或C++类似，用到*作为解引用符号

    let x = 6;
    let y = Box::new(x);// y是一个智能指针，指向堆上存储的数据6
    println!("{}", 6 == x);
    println!("{}", 6 == *y); //为了访问y存储的具体数据，需要解引用

    //实现Deref特质需要实现deref()方法，deref方法返回一个指向结构体体内部数据的指针
    let i = 5;
    let j = MyBox::new(i);//调用静态方法new()

    println!("5 == i is {}", 5 == i);
    println!("5 == *j is {}", 5 == *j);
    println!("i == *j is {}", i == *j);
}

struct MyBox<T>(T);
//结构体方法
impl <T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}
//实现Deref特质，调用deref（）方法
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T{
        &self.0 //返回数据
    }
}