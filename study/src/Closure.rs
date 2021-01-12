fn main(){
    //闭包是函数内部一个没有函数名的内联函数，对于只是使用一次的函数，使用闭包比较好
    let is_even = |x| {
        return x%2 == 0;
    };
    let n = 13;
    println!("{} is even? {:?}", n, is_even(n));

    //访问外层函数可以访问的变量
    let val = 10;
    let add = |x|{ x + val};
    println!("{}", add(3));
}