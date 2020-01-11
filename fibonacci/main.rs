use std::io;

fn main () {
   println!("How much Fibonacci numbers you want to generate?");
   let mut nth = String::new();
   io::stdin().read_line(&mut nth).expect("Failed to read line");
   let nth: i32 = nth.trim().parse().expect("Failed to read line");
   fibo(nth);
}

fn fibo(n: i32) {
   let mut begin = 0;
   let mut start = 1;
   let mut count = 0;
   while count < n {
      let fibo = begin + start;
      println!("Fibonacci sequence is {}", fibo);
      begin = start;
      start = fibo;
      count += 1;
   }
}