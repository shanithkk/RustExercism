pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    
    let mut a=vec![];
     let mut b :u32= 0;
    for i in 1..limit{
        for j in factors{
            if (*j as u32)==0{
                break;
            }
            if i%j==0{
                a.push(i);
                break;
            }
        }
        
    }
    for i in a.iter(){
        b+=i;
    }
   b
}
