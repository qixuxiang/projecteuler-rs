fn main()
{
    let mut result = 0;
    let mut idx = 0;
    while(idx < 1000)
    { 
        idx += 1;
        if(idx % 3 == 0 || idx % 5 == 0)
        {
            result += idx;
        }
        
    }
     println!("result is {:?}", result);
}