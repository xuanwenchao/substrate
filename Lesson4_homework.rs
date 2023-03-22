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

