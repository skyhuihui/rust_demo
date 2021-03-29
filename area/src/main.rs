fn main() {
    println!("正方形");
    let c:Square = Square{x: 2.0};
    println!("2*2 = {}",graphics_area(c));


    println!("三角形");
    let b: Triangle = Triangle{x: 2.0, y: 2.0};
    println!("2*2/2 = {}", graphics_area(b));

}

fn graphics_area<T: Area>(item: T) -> f64 {
    item.area()
}

// 三角形
struct Triangle {
     x: f64, // 底
     y: f64, // 高
}

// 正方形
struct Square {
     x: f64 // 边长
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        let result:f64 = (self.x * self.y) / 2.0;
        result
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        &self.x * self.x
    }
}
