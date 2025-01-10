fn main() {
  //there are two main types
  // scalars like bools, integers, unsigned integers, floats, chars
  let logical : bool = true;
  //formatted print is done using curly braces 
  // println!("{logical}", logical=logical);
  // compound types like arrays and tuples
  // static arrays have syntax [type, size]

  let number = 32; //numbers default to 32 bit integers
  println!("{number}", number=number);
  
  // let array : [i32; 5] = [1, 2, 3, 4, 5];
  // for i in 0..5 { //arrays are printed using the start..(end + 1) syntax
  //   println!("{integer}", integer=array[i]); 
  // }

  //Tuples are collections of different types that is describede with () round braces
  let tuple = (100, true, "Hello");

  //tuples elements are accessed with . notation and the index for the element
  //seems like it cannot be programatically accessed?
  println!("{tuple_out}", tuple_out=tuple.0);
  

  
}