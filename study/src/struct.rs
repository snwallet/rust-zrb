fn main(){
    //初始化结构体
    let mut empl = Employee{
        name: String::from("TutorialsPoint"),
        company: String::from("Mohtashim"),
        age: 23
    };

    empl.age = 50;
    println!("Name is {} company is {} age is {}", 
    empl.name, empl.company, empl.age);

    //结构体实例作为参数
    display(empl);

    //将结构体实例作为函数的返回值
    let empl2 = Employee{
        name: String::from("li"),
        company: String::from("xiaomi"),
        age: 40
    };
    let empl3 = Employee{
        name: String::from("zheng"),
        company: String::from("huawei"),
        age: 30
    };

    let elder = who_is_elder(empl2, empl3);
    display(elder);

    //调用结构体方法，使用方法的属主
    let small = Rectangle{
        width: 20,
        height: 20
    };
    println!("width is {}, height is {}, area is {}",
            small.width, small.height, small.area());

    //结构体的静态方法，直接用结构体的名字来调用方法
    let point = Point::get_instance(10, 20);
    point.display();
}

//定义结构体
struct Employee {
    name: String,
    company: String,
    age: u32
}

fn display(empl: Employee){
    println!("name is {}, company is {}, age is {}", 
    empl.name, empl.company, empl.age);
}

fn who_is_elder(empl2:Employee, empl3:Employee)-> Employee{
    if empl2.age > empl3.age{
        return empl2;
    }else{
        return empl3;
    }
}

struct Rectangle{
    width: u32,
    height: u32
}
impl Rectangle{//从属结构体Employee
    // 方法第一个参数是一个默认的&self参数，是结构体的一个实例
    fn area(&self)-> u32{
        self.width * self.height
    }
}

struct Point {
    x: i32,
    y: i32
}

impl Point{
    fn get_instance(x:i32, y:i32) -> Point{

        Point{x:x, y:y}
    }

    fn display(&self){
        println!("x = {}, y = {}", self.x, self.y);
    }
}