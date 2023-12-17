#[allow(unused, dead_code, unused_imports)]
use serde::{Serialize, Deserialize, };
use serde_json::Result;

#[derive(Deserialize, Clone)]
struct Player {
    pub team: Team
}
#[derive(Deserialize, Clone)]
pub struct Team {
    pub id: String
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
    }

  
}
