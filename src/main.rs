#[allow(unused, dead_code, unused_imports)]
use serde::{Serialize, Deserialize, };
use serde_json::Result;
use std::str;

#[derive(Deserialize, Clone)]
struct Player {
    pub team: Team
}
#[derive(Deserialize, Clone)]
pub struct Team {
    pub id: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Car {
    model: i32,
    year: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32
}

fn deserialize<'a, T>(data: &'a [u8]) ->T where T: Deserialize<'a> {
    let msg = str::from_utf8(data).unwrap();
    serde_json::from_str::<T>(msg).unwrap()
}

fn serialize(object: &impl Serialize)-> String {
    let msg = serde_json::to_string(object).unwrap();
    return msg;
}
fn main() {
    let data  = br#"{"team": {"id": "42"}}"#;
    match serde_json::from_slice::<Player>(data) {
        Ok(player) => {
            println!("Team: {}", player.team.id);
        }
        Err(err) => {
            eprintln!("{err}")
        }
    };

    let car  = Car{model: 7, year: 3333};
    let person = Person{name: "bob".to_string(), age:22};

    let car_json = serialize(&car);
    println!("car data {}", car_json);
    let per_json = serialize(&person);

    let de_car:Car = deserialize(car_json.as_bytes());
    println!("Deserialaziton of car {:?}", de_car);
    let de_per: Person = deserialize(per_json.as_bytes());
    println!("Deserialization of person {:?}", de_per);


  
}
