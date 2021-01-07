fn main(){
    //所有权转让
    //把一个变量赋值给另一个变量
    // let v = vec![1, 2, 3];
    // let v1 = v;
    // println!("{:?}", v);//value borrowed here after move

    //把变量传递给函数作为参数
    // let v = vec![1,2,3];
    // let v1 = v;
    // display(v1);
    // println!("In main {:?}", v1);//value borrowed here after move


    //函数中返回一个变量作为返回值
    let v = vec![4, 5, 6];
    let v1 = v;
    let v1_return = display(v1);
    println!("in main {:?}", v1_return);

    //所有权与基本数据类型
    //将一个变量赋值给另一个变量，并不是所有权的转让
    //而是将一个数据复制过去，原因在于原始数据类型不需要占用多大内存
    let u1 = 1;
    let _u2 = u1;
    println!("u1 = {}", u1);
}

// fn display(v:Vec<i32>){
//     println!("inside display {:?}", v);
// }

fn display(v:Vec<i32>) -> Vec<i32> {
    println!("inside display {:?}",v);
    return v;
}