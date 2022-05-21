use creek::*;
use creek::actors::*;
use std::rc::Rc;
use std::cell::RefCell;
use game_loop::*;
use std::time::*;

#[derive(Debug, Clone)]
pub enum Actors {
    Player(Player),
    Monster(Monster),
}

pub enum ActorEvent {
    Damage(i16),
    Flirt,
}

impl CreekEvent for ActorEvent {}

impl ActorTypes for Actors {
    fn propogate_global_event(&self, event:&GlobalEvent) -> Option<&Vec<CreekAction>> {
        match self {
            Actors::Player(p) => {
                println!("{}", p.str());
                return Some(p.get_creek_actions());
            },
            Actors::Monster(m) => {
                println!("{}", m.str());
                return Some(m.get_creek_actions());
            }
            _ => {

            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    monster: ActorHandle<Actors>,
    creek_actions: Vec<CreekAction>
}

impl Actor for Player {
    type Event = ActorEvent;
    fn get_id(&self) -> Option<ActorID> {
        None
    }
    fn receive_event(&mut self, event:Self::Event) {
    }

    fn get_creek_actions(&self) -> &Vec<CreekAction> {
        &self.creek_actions
    }
}
impl Player {
    pub fn str(&self) -> String {
        format!("Player: \"{}\"", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Monster {
    id: Option<ActorID>,
    health: i16,
    creek_actions: Vec<CreekAction>,
}

impl Actor for Monster {
    type Event = ActorEvent;
    fn get_id(&self) -> Option<ActorID> {
        self.id
    }
    fn receive_event(&mut self, event:Self::Event) {
        match event {
            ActorEvent::Damage(amount) => {
                if self.health - amount > 0 {
                    println!("Monster took {} damage!", amount);
                    self.health -= amount;
                }
                else {
                    self.health = 0;
                    println!("Monster died!");
                    let c_action = self.creek_action(CreekActionType::Destroy);
                    if let Ok(action) = c_action {
                        self.creek_actions.push(action);
                    }
                    else if let Err(e) = c_action {
                        println!("{:?}", e);
                    }
                }
            },
            _ => {}
        }
    }

    fn get_creek_actions(&self) -> &Vec<CreekAction> {
        &self.creek_actions
    }
}

impl Monster {
    pub fn str(&self) -> String {
        format!("Monster has {} health", self.health)
    }
}

struct G();

fn main() {
    let p = Player { name:String::from("TEA"), monster: ActorHandle::<Actors>::default(), creek_actions: Vec::new() };
    let mut c = Creek::<Actors>::new();
    let mut p_handle = c.add_actor(Actors::Player(p));
    let mut m_handle = c.add_actor(Actors::Monster(Monster {
        id: None,
        health: 200,
        creek_actions: Vec::new()
    }));
    p_handle.edit_actor(|actor| {
        if let Actors::Player(player) = actor {
            player.monster = m_handle.clone();
        }
    });
    let m_id = m_handle.id.clone();
    m_handle.edit_actor(|a| {
        if let Actors::Monster(m) = a {
            m.id = Some(m_id);
        }
    });

    println!("{:?}", m_handle);

    
    let mut line = String::new();

    let mut delta_time = Instant::now();
    game_loop(c, 240, 0.1, |g| {
        g.game.push_event(GlobalEventType::Update(delta_time.elapsed().as_secs_f32()), None);
        g.game.propagate_events();
        delta_time = Instant::now();
        std::io::stdin().read_line(&mut line);
        println!("{}", line);
        if line.to_lowercase().starts_with("d") {
            m_handle.edit_actor(|actor| {
                if let Actors::Monster(monster) = actor {
                    monster.receive_event(ActorEvent::Damage(80));
                }
            });
        }
        line = String::new();
    }, |g| {
    });
}
