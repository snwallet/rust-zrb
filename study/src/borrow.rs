fn main(){
    //原来将参数传递给函数就可以将所有权转让给函数
    //脱离函数之后所有权没办法转回原来的变量，可以用return返回值
    //借用 就是一个函数中将一个变量传递给另一个函数作为参数暂时使用。
    //Rust 也引用了自动 还 的概念，就是要求函数的参数离开其作用域时需要将 所有权 还给当初传递给他的变量
    //要实现就要把参数定义为引用，传递变量也是引用
    let v = vec![10, 20, 30];
    print_vector(&v);
    println!("inside main {}", v[0]);

    //引用默认情况下是只读的，如果需要修改要在参数引用上加上&mut关键字
    let mut i = 3;
    println!("before {}", i);
    add_one(&mut i);
    println!("after {}", i);

    //字符串的可变引用
    let mut name:String = String::from("Tutorial");
    dispaly(&mut name);
    println!("the value of name after modificiation is {}", name);
}

fn print_vector(v:&Vec<i32>){
    println!("inside print function {:?}", v);
}

fn add_one(i:&mut i32){
    *i += 1;
}

fn dispaly (name: &mut String){
    println!("name value is: {}", name);
    name.push_str("Rocks");
}