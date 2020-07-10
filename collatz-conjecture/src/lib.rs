pub fn collatz(n: u64) -> Option<u64> {
   if n==0{
   		return None;
   }
   let mut m=n;
   let mut c=0;
   while m>1{
   		if m%2==0{
   			m=m/2;
   		}
   		else{
   			m=3*m+1;
   		}
   		c+=1;
   }
   
	Some(c)
}