fn main() {
    
    // println!("{subject} {verb} {object}",  object = "the lazy dog",
    //     subject="the quick brown fox", verb="jumps over");
    // println!("{number:0>width$}", number=1, width=6);
    // println!("My name is {0}, {1} {0}", "Bond","James");
    // #[allow(dead_code)]
    // struct Structure(i32);
    // println!("this struct '{}" won't print...", Structure(3));


    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("{}, '{name:.*}' has 3 characters", "Hello", 3, name=1234.56);
    println!("{}, '{name:.*}' has 3 characters", "Hello", 3, name="1234.56");
    println!("{}, '{name:>8.*}' has 3 right-aligned characters", "Hello", 3, name="1234.56");
    let pi = 3.141592;
    println!("Pi is roughly {name:.*}",  3, name=pi);
    
    }
