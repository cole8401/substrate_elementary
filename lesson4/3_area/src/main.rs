trait AreaType{
    fn area_type_print(&self);
}

struct Rectangle{
    width:u32,
    long:u32,
}


struct Circle {
    radius:f32,
}

impl AreaType for Rectangle{
    fn area_type_print(&self){
        println!("area is(长方形的面积是){}",self.long*self.width);
    }
}
impl AreaType for Circle{
    fn area_type_print(&self){
        let pi:f32 = 3.14;
        let ci:f32 = self.radius*self.radius*pi;
        println!("area is(圆形的面积是){}",ci);
    }
}

fn area_print<T:AreaType>(item:T){
    item.area_type_print();
}
fn main() {
    let rec_area = Rectangle {width:30,long:20};
    let cir_area = Circle{radius:20.0};
    area_print(rec_area);
    area_print(cir_area);
}