
//　为枚举实现trait， trait包含返回时间的方法，不同灯持续的时间不同
fn main() {
    let r = TrafficLight::Red;
    println!("The red light time : {}", r.time());

    let g = TrafficLight::Green;
    println!("The green light time : {}", g.time());

    let y = TrafficLight::Yellow;
    println!("The yellow light time : {}", y.time());
}


enum TrafficLight{
    Red,
    Green,
    Yellow,
}

impl Time for TrafficLight {
    fn time(&self) -> u8 {
        match &self {
            TrafficLight::Red => { 60 },
            TrafficLight::Green => { 30 },
            TrafficLight::Yellow => { 3 }
        }
    }
}

pub trait Time {
    fn time(&self) -> u8;
}