fn main(){
    //简单教程里的内容
    
    // let tuple: (i32, f64, u8) = (-325, 4.9, 22);
    // println!("{:?}", tuple);

    let t:(i32, f64, u8) = (-325, 4.9, 33);
    println!("{}", t.0);
    println!("{}", t.1);
    println!("{}", t.2);

    let b:(i32, bool, f64) = (110, true, 10.9);
    print(b);

    //元组解构赋值
    //就是把元组中的每一个元素按照顺序一个个赋值给变量
    let s:(i32, bool, f64) = (30, true, 7.9);
    analyist(s);

    // 数组
    // 指定默认初始化
    // let arr:[i32; 4] = [1; 4];

    let arr:[i32; 4] = [1,2,3,4];
    println!("array is {:?}", arr);
    println!("the length of array is {}", arr.len());

    // 使用for..in..来遍历数组
    let arr1 = [19, 20, 30, 33];
    println!("{:?}", arr1);
    for index in 0..arr1.len(){
        println!("arr1 is {}:{}", index,arr1[index]);
    }
    
    //迭代数组
    let arr2 = [10, 20, 30, 40];
    for a in arr2.iter(){
        println!("value is {}", a);
    }

    //数组默认是只读的
    // let arr3 = [1, 2, 3, 4];
    // //consider changing this to be mutable: `mut arr3`
    // arr3[0] = 8;
    // println!("{:?}", arr3);
    // println!();
    // println!();
    //数组作为值传递
    let arr3 = [1, 3, 5, 7];
    update1(arr3);
    println!("{:?}", arr3);

    //将数组作为引用传递
    let mut arr4 = [1, 1, 1];
    println!("arr4 is {:?}", arr4);
    update(&mut arr4);
    println!("arr4 is change to {:?}", arr4);

    //数组的长度必须是常量
    // let l: usize = 20;
    // let arr = [0; l];// non-constant value
    // print!("{}", arr[10]);
    const L: usize = 20;
    let arr = [0; L];
    print!("{}", arr[10]);
}




//将元组传给函数作为形参
fn print(x:(i32, bool, f64)) {
    println!("Inside print method");
    println!("{:?}", x);
}
//元组解构赋值
fn analyist(x:(i32, bool, f64)) {
    println!("Inside print method");
    let (age, is_female, cgpa) = x;
    
    println!("age is {}, isFemale? {}, cgpa is {}", age, is_female, cgpa);

}

//将数组传参：值传递，引用传递
fn update1(mut arr3:[i32;4]){
    for i in 0..arr3.len(){
        arr3[i] = 4;
    }
    println!("Inside update {:?}", arr3);
}

//引用传递
fn update(arr4:&mut [i32; 3]){
    for i in 0..arr4.len(){
        arr4[i] = 5;
    }
}