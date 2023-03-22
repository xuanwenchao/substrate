/*
* 1.为枚举交通信号灯实现一个trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同；
*/

trait TrafficTime {
    fn get_time_seconds(&self) -> u16;
}

enum TrafficLight {
    Green,
    Red,
    Yellow,
}

impl TrafficTime for TrafficLight {
    fn get_time_seconds(&self) -> u16 {
        match self {
            TrafficLight::Green => 30,
            TrafficLight::Red => 40,
            TrafficLight::Yellow => 5,
        }
    }
}

fn main() {
    let green = TrafficLight::Green;
    println!("green time:{}",green.get_time_seconds());

    let red = TrafficLight::Red;
    println!("red time:{}",red.get_time_seconds());

    let yellow = TrafficLight::Yellow;
    println!("yellow time:{}",yellow.get_time_seconds());
}

/*
* 2.实现一个函数，为u32类型的整数集合求和，参数类型为&[u32]，返回类型为Option,溢出时返回None
*/
use std::{option::Option};
fn get_sum(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for val in nums {

        let result = sum.checked_add(*val);
        match result {
            Some(_) => sum+=val,
            None => return None,
        }
    }
    return Option::Some(sum);
}

fn main() {
    let arr:Vec<u32> = vec![19, 23, 200, 12, 30];
    let res = get_sum(&arr);
    match res {
        Some(r) => println!("{}", r),
        None => println!("overflow!"),
    }
    
}

/*
* 2.实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到范型和范型约束。
*/


trait CaculateArea {
    fn get_area(&self) -> u32;
}

struct Circle {
    radius: f32,
}
impl CaculateArea for Circle {
    fn get_area(&self) -> u32 {
        let PI: f32 = 3.14159265;
        return (PI * self.radius).round() as u32;
    }
}

struct Rectangle {
    //结构体的定义
    width: u32,
    height: u32,
}

impl CaculateArea for Rectangle {
    fn get_area(&self) -> u32 {
        return self.width * self.height;
    }
}

struct Square {
    s: u32,
}
impl CaculateArea for Square {
    fn get_area(&self) -> u32 {
        return self.s * self.s;
    }
}

struct Triangle {
    bottom: u32,
    height: u32,
}
impl CaculateArea for Triangle {
    fn get_area(&self) -> u32 {
        return self.bottom * self.height / 2;
    }
}

fn get_area<T>(shate:T) -> u32
where
    T: CaculateArea,
{
    return shate.get_area();
}

fn main() {
   let circle = Circle{
        radius: 16.0,
    };

    let triangle = Triangle{
        bottom:10,
        height:8,
    };

    let square = Square{
        s: 9,
    };

    let rectangle = Rectangle{
        width:20,
        height:10,
    };


    println!("Circle Area:{}",get_area(circle));
    println!("Triangle Area:{}",get_area(triangle));
    println!("Square Area:{}",get_area(square));
    println!("Rectangle Area:{}",get_area(rectangle));
}


