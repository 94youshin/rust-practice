fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.1415926;
}


/* 参数类型和返回值的类型必须显示定义，无返回值可省略，返回unit.
 * 函数内部提前返回使用return,否则最后一个表达式就是其返回值；最后一个表达式加了; 隐含其返回值为unit
*/
fn main() {
    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = {
        pi();
    };

    println!("is_pi: {:?}", is_pi);
    println!("is_unit1: {:?}", is_unit1);
    println!("is_unit2: {:?}", is_unit2);
}
