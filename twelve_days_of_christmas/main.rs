fn main() {
   println!("Twelve Days of Christmas Lyrics");
   let lyrics = [
      "A partridge in a pear tree",
      "turtle doves, and",
      "french hens",
      "calling birds",
      "golden rings",
      "geese a-laying",
      "swans a-swimming",
      "maids a-milking",
      "ladies dancing",
      "lords a-leaping",
      "pipers piping",
      "drummers drumming",
   ];
   let days = ["first", "second", "third", "fourth", "fifth", "sixth", 
   "seventh", "eight", "nine", "tenth", "eleventh", "twelfth"];
   let mut index = 0;
   while index < 12 {
      println!("On the {} day of Christmas my true love sent to me", days[index]);
      for i in (0..index + 1).rev() {
         if i == 0 {
            println!("{}", lyrics[i]);
         } else {
            println!("{} {}", i + 1, lyrics[i]);
         }
      }
      index += 1;
   }

}