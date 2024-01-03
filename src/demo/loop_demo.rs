
fn loop_test() {
  // while 循环
  let mut i = 0;
  while i<5 {
      println!("while do number is {}", i);
      i+=1;
  }
  // for 循环
  let arr = ["jjb", "zzx", "fyy", "lyy", "myy"];
  for item in arr {
      println!("for do str is {}",  item);
  }
  // loop 循环， 无限循环除非主动跳出
  let mut loop_i = 0;
  let res = loop{
      if loop_i == 3 {
          break i;
      }
      loop_i += 1;
  };
  println!("loop result is {}", res);
}