fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("counter = {}", counter);
        if counter == 10 {
            break counter * 2; 
            //loop是一个死循环，会将操作尝试到成功为止，
            //如果你想将一个值返回，可以将它放在break之后，他就能被loop返回
        }
    };
    assert_eq!(result, 20);    
    println!("rusult = {}", result);
}
