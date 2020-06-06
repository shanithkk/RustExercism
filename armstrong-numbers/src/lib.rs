pub fn is_armstrong_number(num: u32) -> bool 
{
  let mut n=num;
  let mut d=num;
  let mut c=0; let mut rem: u32=0;
  let mut sum=0;
  while n>0 
  {
    n=n/10;
    c+=1;  
  }
  while d>0 {
    rem=d%10;
    sum += u32::pow(rem,c);
    d /=10;
  }
  println!("{:?}  {} {}",c,sum,rem );

  if sum==num{
    true
  }  
  else {
    false
  }
}