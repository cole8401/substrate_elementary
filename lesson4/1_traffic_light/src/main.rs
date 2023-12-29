fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;
    
    println!("Red light duration: {}", red_light.duration());
    println!("Yellow light duration: {}", yellow_light.duration());
    println!("Green light duration: {}", green_light.duration());
}
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match *self {
            Self::Red => 30,
            Self::Yellow => 5,
            Self::Green => 60,
        }
    }
}
