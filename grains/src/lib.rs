pub fn square(s: u32) -> u64 {
   if s<=64 && s>=1{
       let b:u64=2;
       let a: u64= b.pow(s-1);
       a
    }else{
       panic!("Square must be between 1 and 64");
    }

}
pub fn total() -> u64 {
  let b:u64=2;
  let c :u64=b.pow(63)-1+b.pow(63);
  c
}