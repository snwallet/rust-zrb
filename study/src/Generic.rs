use::std::fmt::Display;

fn main(){
    //泛型可以在运行时指定数据类型
    //泛型应用在容器，结构体，函数，枚举，特质

    //泛型容器， 向量
    let mut v : Vec<i32> = vec![1, 2, 4];
    v.push(40);
    // v.push("name");//expected `i32`, found `&str`泛型限制类型，让容器更加安全
    println!("{:?}", v);

    //泛型结构体，
    let data : Data<i32> = Data{value: 320};
    println!("value is {}", data.value);

    let data1 : Data<String> = Data{value: "hello".to_string()};
    println!("value1 is {}", data1.value);

    //特质trait类似于接口，是一组方法的原型，其中有具体方法和抽象方法
    trait SomeTrait {
        fn method1(&self);//无任何具体实现的抽象方法
        fn method2(&self){ //有具体实现的具体方法
            //方法的具体代码
        }
    }
    //可以证明两个结构体之间的关系，跨多个结构体实现一组标准方法
    //特质里面的方法如果需要共享在不同的实现了特质的结构体，则使用具体方法
    //具体方法可以在实现了特质的结构体内部重写
    //特质里面的方法如果想让实现特质的结构体自己定义就用抽象方法

    //实现特质，impl.2. for.1.，为 1来实现2.

    //特质简单使用
    let book  = Book{
        id: 1000,
        name: "如何阅读一本书"
    };
    book.print();


    //泛型函数
    print_pro(43 as u8);
    print_pro(50 as u16);
    print_pro("Inside main function.");
}

struct Data<T>{
    value: T
}

struct Book{
    id:u32,
    name: &'static str
}

//声明特质
trait Printable {
    fn print(&self);
}
//实现特质
impl Printable for Book{
    fn print(&self){
        println!("Printing book with id:{} and name:{}", self.id, self.name);
    }
}

//泛型函数
fn print_pro<T: Display>(t:T){
    println!("Inside print_pro generic function:");
    println!("{}", t);
}