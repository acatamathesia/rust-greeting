
fn method_test() {
  test_function();
test_args_function(3,4);
// 函数表达式，必须要有返回值,使用{}可以执行比较复杂的表达式
// 在块中可以使用函数语句，最后一个步骤是表达式，此表达式的结果值是整个表达式块所代表的值。这种表达式块叫做函数体表达式
// 注意：x + 1 之后没有分号，否则它将变成一条语句！
let y = {
  let x = 1;
  x+1
};
println!("表达式: {}", y);
// 可嵌套的函数定义
fn five() -> i32 {
  5
}
println!("函数five的值是: {}", five());

// 函数返回值测试
let res: (bool, i32) = test_return_function(1, 2);
println!("函数test_args_function的返回值元组数据是: ({}, {})", res.0, res.1);
let (ok, r) = res;
println!("函数test_return_function的返回值元组通过解析后的数据, ok: bool = {}, r: i32 = {}", ok, r);
}

// 函数基本的定义格式
fn test_function() {
  println!("test_function is run");
}

// 函数中的参数必须有具体的类型
fn test_args_function(i: i32, j: i32){
  let res = i*j;
  println!("i({})*j({})=res({})", i, j, res);
}

// 函数的返回值 需要有->进行明确标识，不能使用：
// 如果函数返回值类型没有明确标识，那么会判断该函数是过程函数，无返回值
fn test_return_function(i: i32, j: i32) -> (bool, i32) {
  return (true, i+j);
}