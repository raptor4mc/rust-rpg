use std::io;
mod gameloop;

struct Player {
    health: i32,
    rubies: i32,
    name: &'static str,
    last_name: String,
    xp: i32,
    level: i32,
    health_type: String,
    amount_friends: i32,
    best_friend: String,
}

impl Player {
    pub fn levelup(&mut self) {
        if self.xp >= 10 {
            self.level += 1;
            self.xp -= 10;
        }
    }
    pub fn health_t(&mut self) {
        if self.health_type == "Poison" {
            if self.health > 1 {
                self.health = 1;
            }
        }
    }
}

fn main() {
    let mut Name = String::new();
    io::stdin().read_line(&mut Name).expect("Thats not a name?");
    let name = Name.trim();
    let mut name2 = String::new();
    io::stdin()
        .read_line(&mut name2)
        .expect("Do a real lastname");
    let nametwo = name2.trim();
    let user = Player {
        health: 5,
        rubies: 200,
        name: name,
        last_name: nametwo.to_string(),
        xp: 0,
        level: 0,
        health_type: String::from("Good to go"),
        amount_friends: 0,
        best_friend: String::from("Good to go"),
    };
}
