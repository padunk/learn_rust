struct Rectangle {
   width: u32,
   height: u32,
}

fn main() {
   let rect = Rectangle {
      width: 32,
      height: 10,
   };

   let area = rectangle(&rect);

   println!("area {}", area);
}

fn rectangle(r: &Rectangle) -> u32 {
   r.width * r.height
}