struct Point {
  x: f32,
  y: f32
}

struct Rectangle{
  top_left : Point,
  bottom_right : Point
}

fn rect_area(rectangle : Rectangle) -> f32 {
  //height * width
  //(top_left.y - bottom_right.y) * (bottom_right.x - top_left.x)
  let height : f32 = rectangle.top_left.y - rectangle.bottom_right.y;
  let width : f32 = rectangle.bottom_right.x - rectangle.top_left.x; 

  return height * width;
}


fn main() {
  let rectangle = Rectangle{top_left: Point{x: 0.0, y: 2.0}, bottom_right : Point{x: 2.0, y: 0.0}};
  println!("{rectangle_area}", rectangle_area=rect_area(rectangle));
}