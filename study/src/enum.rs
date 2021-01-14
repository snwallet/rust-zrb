fn main(){
    //枚举元素的使用
    let selected = Fruits::Banana;
    println!("{:?}", selected);

    //枚举类型作为结构体元素的类型
    let p = Person{
        name: String::from("zheng"),
        gender: GenderGategory::Female
    };
    let p1 = Person{
        name: String::from("wo"),
        gender: GenderGategory::Male
    };
    //格式化输出结构体实例
    println!("{:?}", p);
    println!("{:?}", p1);

    //option枚举
    let result = is_even(3);
    println!("{:?}", result);
    println!("{:?}", is_even(30));

    //match用于枚举判断
    print_size(CarType::Hatch);
    print_size(CarType::Sedan);
    print_size(CarType::SUV);

    //match 和 Option
    match is_even(5) {
        Some(data) => {
            if data == true {
                println!("Even no");
            }
        },
        None => {
            println!("not even");
        }
    }

    //match 语句和带数据类型的枚举
    let p1 = GenderGategory1::Name(String::from("zheng"));
    let p2 = GenderGategory1::User_Id(100);
    println!("{:?}", p1);
    println!("{:?}", p2);

    match p1 {
        GenderGategory1::Name(val) => {
            println!("{}", val);
        },
        GenderGategory1::User_Id(val) => {
            println!("{}", val);
        }
    }

}
//枚举类型
#[derive(Debug)]
enum Fruits {
    Banana,
    Pear,
    Mandarin,//橘子
    Eggplant//茄子
}

#[derive(Debug)]
enum GenderGategory {
    Male,Female
}
#[derive(Debug)]
struct Person{
    name: String,
    gender: GenderGategory
}

fn is_even(no:i32) -> Option<bool>{
    if no%2 == 0{
        Some(true)
    }else{
        None
    }
}
#[derive(Debug)]
enum CarType {
    Hatch,
    Sedan,
    SUV
}

fn print_size(car: CarType){
    match car {
        CarType::Hatch =>{
            println!("small sized car");
        },
        CarType::Sedan =>{
            println!("medium sized car");
        },
        CarType::SUV =>{
            println!("large sized sports utility car");
        }
    }
}
#[derive(Debug)]
enum GenderGategory1 {
    Name(String),User_Id(i32)
}

