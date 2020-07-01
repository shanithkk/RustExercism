pub fn find(array: &[i32], key: i32) -> Option<usize> {
   let mut l=0;
   let mut r=array.len();
   while r-l>1 {
      let m=l+(r-l)/2;
      if array[m]==key{
         return Some(m);
      }
      if array[m]<key{
         l=m;
      }
      else {
         r=m;
      }
   }

   if !array.is_empty() && array[l] == key {
        return Some(l);
    }
    None
}
