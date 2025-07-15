
trait Vehicle {
    type Fuel<'f> ;
    type Speed<'s> ;

    fn refuel<'f>(&self, fuel: Self::Fuel<'f>) -> String;
    fn get_speed<'s>(&self) -> Self::Speed<'s>;
}

// กำหนด struct Car
struct Car;

// Implement trait Vehicle สำหรับ Car
impl Vehicle for Car {
    type Fuel<'f> = &'f str; // Fuel เป็น &str
    type Speed<'s> = &'s str; // Speed เป็น &str

    fn refuel<'f>(&self, fuel: Self::Fuel<'f>) -> String {
        format!("Car refueled with {} fuel type", fuel)
    }

    fn get_speed<'s>(&self) -> Self::Speed<'s> {
        "120 km/h"
    }
}

struct Bicycle;

// Implement trait Vehicle สำหรับ Bicycle
impl Vehicle for Bicycle {
    type Fuel<'f> = &'f str; // Fuel เป็น &str
    type Speed<'s> = &'s str; // Speed เป็น &str

    fn refuel<'f>(&self, fuel: Self::Fuel<'f>) -> String {
        format!("Bicycle rider ate {}", fuel)
    }

    fn get_speed<'s>(&self) -> Self::Speed<'s> {
        "25 km/h"
    }
}

// ฟังก์ชันที่รับ Vehicle และใช้งาน associated type
fn use_vehicle<'f,'s, V,F,S>(vehicle: V, fuel_amount: F) -> (String, S)
where
    V: Vehicle<Fuel<'f>=F,Speed<'s>=S> + 'f+'s,
{
    let refuel_result = vehicle.refuel(fuel_amount);
    let speed = vehicle.get_speed();
    (refuel_result, speed)
}

fn main() {
    // สร้าง instance ของ Car และ Bicycle
    let car = Car;
    let bicycle = Bicycle;

    // เรียกใช้ฟังก์ชันกับ Car
    let (car_refuel, car_speed) = use_vehicle(car, "50.0");
    println!("Car: {}, Speed: {} km/h", car_refuel, car_speed);

    // เรียกใช้ฟังก์ชันกับ Bicycle
    let (bicycle_refuel, bicycle_speed) = use_vehicle(bicycle, "500");
    println!("Bicycle: {}, Speed: {} km/h", bicycle_refuel, bicycle_speed);
}
