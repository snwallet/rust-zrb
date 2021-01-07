use std::fs::File;

fn main(){
    //分为可恢复错误和不可恢复错误，相当于异常和错误
    //panic!(string_error_msg) 会导致程序立即退出，并反馈错误
    // panic!("hello");//any code following this expression is unreachable
    // println!("End of main");
    //数组越界错误

    //可恢复错误和Result枚举
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E)
    // }

    let f = File::open("main.js");
    // println!("{:?}", f);
    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        },
        Err(e) => {
            println!("file not found \n{:?}", e);
        }
    }

    println!("end of main");

    //函数返回错误
    let f = is_even(13);
    match f {
        Ok(d) => {
            println!("is even? {}", d);
        },
        Err(msg) => {
            println!("Error message is {}", msg);
        }
    }

    // unwrap()函数如果是Ok或Some则返回包含的值，否则调用宏panic!()
    let result = is_even(10).unwrap();
    println!("result is {}", result);

    //expect()和unwrap()差不多，只是会返回自定义的错误消息
    let f = File::open("pq.txt").expect("File not able to open");
    println!("{:?}", f);
}

fn is_even(num:i32)-> Result<bool, String>{
    if num%2 == 0{
        return Ok(true);
    }else{
        return Err("NOT_AN_EVEN".to_string());
    }
}