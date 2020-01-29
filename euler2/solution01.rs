
fn fib(n:i32) -> i32{
    let mut res = 0;
    let mut next_item = 1;
    let mut tmp = 0;
    if(n <= 0){
        panic!("input can not be less than zero!");

    }
    for _ in 0..=n{
        let mut tmp = next_item;
        next_item += res;
        res = tmp;
    }
    return res;
}

fn main()
{
    
    let mut idx = 0;
    let mut counter = 0;
    let mut tmp = 0;
    loop {
        idx += 1;
        tmp = fib(idx);
        if tmp > 400_0000 {
            break;
        }
        if (tmp % 2 == 0) {
            counter = counter + tmp;
        }
        
    };
    println!("result is {:?}", counter);
    

    
}