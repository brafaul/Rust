//File: main.rs
//Author: Brayden Faulkner
//Note: Escape velocity for planet earth is 11,186 metres per second
use std::io;

fn main() {
    println!("How many guns are there?");
    let mut gun_count = String::new();
    io::stdin().read_line(&mut gun_count)
        .expect("Failed to read line");
    let gun_count: f32 = match gun_count.trim().parse(){
        Ok(num) => num,
        Err(_) => panic!("We needed a number"),
    };
    println!("How many bullets are there?");
    let mut bullet_count = String::new();
    io::stdin().read_line(&mut bullet_count)
        .expect("Failed to read line");
    let bullet_count: f32 = match bullet_count.trim().parse(){
        Ok(num) => num,
        Err(_) => panic!("We needed a number"),
    };
    let gun_force_newts: f32 = 32955.0; //force extered by each gun in newtons
    let gun_force_lbs: f32  = 7408.58; //force exterted by each gun in pounds
    //let gun_force_newts: f32 = 3380000.0;
    let gun_weight: f32 = 1459.0; //weight of each gun in newts
    let gun_mass: f32 = 148.88; // mass of each gun in kilograms
    let bullet_weight: f32 = 3.83; // weight of each bullet in newts
    let bullet_mass: f32 = 0.39; // mass of each bullet in kilograms
    let shuttle_weight_lbs: f32 = 165000.0; // weight of the shuttle in pounds
    let shuttle_weight_newts: f32 = 733956.57; // shuttle weight in newts
    let shuttle_mass: f32 = 29930.0; //mass of shuttle in kilograms
    let thrust: f32 = gun_count * gun_force_newts;
    let mass: f32 = (gun_mass * gun_count) + (bullet_mass * bullet_count) + shuttle_mass;
    println!("Mass is {}kg", mass);
    println!("Thrust is {}N", thrust);
    let weight: f32 = (gun_weight * gun_count) + (bullet_weight * bullet_count)/2.0 + shuttle_weight_newts;
    println!("Average Weight is {}N", weight);
    let t2w: f32 = thrust / weight;
    println!("Thrust to weight ratio is {}", t2w);
    let resultant_force : f32 = thrust - weight;
    println!("Resultant Force is {}N", resultant_force);
    let acceleration: f32 = resultant_force/mass;
    println!("Acceleration is {} metres per second squared", acceleration);
    let flight_time: f32 = bullet_count/(gun_count * 100.0);
    println!("Flight Time is {} seconds", flight_time);
    let mut distance: f32 = 0.0; 
    let mut x = 0;
    let mut curr_speed: f32 = 0.0;
    let num_rounds: u32 = flight_time.round() as u32;
    while x < num_rounds{
        x += 1;
        curr_speed = curr_speed + acceleration;
        distance = distance + curr_speed;
    }
    println!("Top speed is {} meters per second", curr_speed);
    println!("Distance is {} meters", distance);
}
