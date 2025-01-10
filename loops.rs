//fizz buzz


fn fizz_buzz(n : i32) {
  if(n <= 0){
    return;
  }
  for i in 1..(n + 1) {
    if i % 3 == 0 && i % 5 == 0 {
      println!("FizzBuzz");
    }
    else if i % 3 == 0 {
      println!("Fizz");
    }
    else if i % 5 == 0 {
      println!("Buzz");
    }
    else{
      println!("{Number}", Number=i);
    }
  }
}


//iterators

//print names in a list
//use the match keyword which is a switch statement
fn print_names() {
  let names = ["john", "sue", "darryl", "XYZ123"];
  for name in names {
    match name {
      "john" => println!("JOHN!"),
      "sue" | "darryl" => println!("yo!!!"),
      // this handles the rest of the cases
      _ => println!("HELLO")
    }
    // println!("{name}", name=name);
  }
}

//if let 

fn main() {
  // print_names();
  // fizz_buzz(15);
}