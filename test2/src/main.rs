fn main() {

    //for循环
    for i in 1..=9 {
        // println!("i = {}", i);
        for j in 1..=i{
            if i == 3&&j == 3 || i == 4&&j == 4
            || i == 4&&j == 3 {
                print!(" {}x{} = {}", i, j, i*j);
                continue;
            }
            print!("{}x{} = {} ", i, j, i*j);

        }
        println!();
    }

    println!();
    println!();
    //loop循环
    let mut i = 0u32;
    let mut j = 0u32;
    loop {
        i += 1;
        j = 0;
        loop {
            j += 1;
        
            if  i == 3 && j == 3 || i == 4 && j == 3 
            || i == 4 && j == 4{
                print!(" {}x{}= {}", i, j, i*j);
                
            }else{
            print!("{}x{}= {} ", i, j, i*j);
            }

                  
            if j == i{
                println!();
                break;
            }
            
        }
        if i == 9{
            break;
        }
    }
    
    println!();
    println!();
    //while循环
        let mut i = 1u32;
        let mut j = 1u32;
        while i <= 9 {
            while j <= i {
                if i == 3&&j == 3 || i == 4&&j == 4
                || i == 4&&j == 3 {
                    print!(" {}x{} = {}", i, j, i*j);
                    
                }else{
                    print!("{}x{} = {} ", i, j, i*j);
                }   
                j += 1;
            }
            i += 1;
            j = 1;
            println!();
        }
}
