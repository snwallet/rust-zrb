fn main(){
    
    let selected = Fruits::Banana;
    println!("{:?}", selected);

    let p = Person{
        name: String::from("zheng"),
        gender: GenderGategory::Female
    };
    let p1 = Person{
        name: String::from("wo"),
        gender: GenderGategory::Male
    };
    println!("{:?}", p);
    println!("{:?}", p1);

    //option枚举
    let result = is_even(3);
    println!("{:?}", result);
    println!("{:?}", is_even(30));
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