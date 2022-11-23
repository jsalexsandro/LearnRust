fn types(){
  // scalar types
  // primitive types
  // compost types (Tuple, Array)

  // int types -> performance
  // bits - signed - unsigned
  // 8    - i8     - u8
  // 16   - i16    - u16
  // 32   - i32    - u32
  // 64   - i64    - u64
  // 128  - i128   - u128
  // arch - isize  - usize

  // signed support negative numbers and positive numbers
  // range: -(2 ^ n - 1) to (2 ^ n - 1) - 1

  // unsigned value ever positive numbers
  // range: 0 to (2 ^ n) - 1
  
  // it is possible using (Hex, Bin, Octal) Numbers

  // let hex  = 0xfff;
  // let bin = 0b01;
  // let oct = 0o12;

  // using char -> 4 bytes
  // let c = 'a';
  // it is possible unicode characters

  // using bool 
  // let b = true;
  // let c = false;

  // using float, double
  // let f:f32 = 1.0; // -> float
  // let d:f32 = 2.0; // -> double
  
  // compost types
  // tuple -> fixed size
  // info: accepted mixed primitives types
  // let numbers = (1, 2, 3);
  // to get the number using numbers.position
  // println!("{}", numbers.0); // 1
  // to destructure the tuple using
  // let (a, b, c) = numbers;
  // for mut tuple using in defination
  // let mut a = (1, 2, 3)

  // array -> not support diferent types
  // let n = [1, 2, 3, 4, 5, 6];
  // to get the number using = n[local]
  // println!("{:?}", numbers[0]); // 1
  // for slice using
  // println!("{:?}", &n[2..5]) // [3, 4, 5]
}