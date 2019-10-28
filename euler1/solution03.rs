fn main() {
    let mut idx = 0;
    let mut counter = 0;
    let result = loop {
        idx += 1;
        if(idx % 5 ==0 || idx % 3 ==0)
        {
            counter += idx;

        }

        if idx == 1000 {
            break counter;
        }
    };
    println!("result is {:?}", result);

}