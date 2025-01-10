fn main() {
  //the from request allocates heap memory
  let s1 = String::from("hello");
  //this is a move from s1 to s2
  let s2 = s1;
  //this is a deep copy of s2 to s3
  let s3 = s2.clone();

  println!("{s1}, world", s1=s1);


}