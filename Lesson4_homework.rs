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



