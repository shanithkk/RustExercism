pub fn factors(n: u64) -> Vec<u64> 
{

  let mut res=Vec::new();
  let mut d= 2;
  let mut a=n;
  while a>1 
  {
    if a%d==0
    {
      res.push(d);
      a=a/d;
    }
    else
    {
      d=d+1;
    }
  }

res
}
