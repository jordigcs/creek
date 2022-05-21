// Creek is an Actor system designed for Games

pub mod actors;
use crate::actors::*;
use std::{rc::Rc, collections::VecDeque, cell::RefCell, mem};

impl CreekEvent for GlobalEvent {}

#[derive(Debug, Copy, Clone)]
pub enum CreekError {
    ActorDoesNotStoreId,
    ActorDoesNotExistAtIndex,
    ActorIdInvalidated,
}

#[derive(Debug, Clone)]
pub enum GlobalEventType {
    Init,
    Update(f32),
    ActorAdded(ActorID),
    ActorRemoved(ActorID)
}

#[derive(Debug, Clone)]
pub struct GlobalEvent {
    event_type: GlobalEventType,
    target: Option<ActorID>,
}

impl GlobalEvent {
    pub fn new(event_type:GlobalEventType) -> Self {
        GlobalEvent { event_type, target: None }
    }

    pub fn with_target(self, target: ActorID) -> Self {
        GlobalEvent { event_type: self.event_type, target: Some(target) }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CreekActionType {
    Destroy
}

#[derive(Debug, Clone, Copy)]
pub struct CreekAction {
    action_type: CreekActionType,
    target: ActorID,
}

pub struct Creek<T: ActorTypes + Clone> {
    actors:Vec<ActorHandle<T>>,
    events:Vec<GlobalEvent>
}

impl<T: ActorTypes + Clone> Creek<T> {
    pub fn new() -> Creek<T> {
        Creek {
            actors: Vec::new(),
            events: Vec::new()
        }
    }

    pub fn push_event(&mut self, event_type:GlobalEventType, target:Option<ActorID>) {
        self.events.push(GlobalEvent {
            event_type,
            target
        });
    }

    pub fn add_actor(&mut self, actor_type:T) -> ActorHandle<T> {
        // Find empty spot
        let mut actor_handle = ActorHandle::new(Rc::new(RefCell::new(actor_type)));
        for (index, handle) in self.actors.iter().enumerate() {
            if let None = handle.inner {
                actor_handle.id.index = index;
                actor_handle.id.gen = handle.id.gen + 1;
            }
        }
        if actor_handle.id.gen > 0 {
            self.actors[actor_handle.id.index] = actor_handle.clone();
        }
        else {
            actor_handle.id.index = self.actors.len();
            actor_handle.id.gen = 0;
            self.actors.push(actor_handle.clone());
        }
        self.push_event(GlobalEventType::ActorAdded(actor_handle.id), None);
        ActorHandle { id: actor_handle.id, inner: Some(actor_handle.inner.unwrap().clone()) }
    }

    pub fn get_actor(&self, id:ActorID) -> Result<&ActorHandle<T>, CreekError> {
        let valid = self.validate_actor_id(id);
        if valid.is_ok() {
            return Ok(&self.actors[id.index]);
        }
        else {
            return Err(valid.unwrap_err());
        }
    }

    pub fn get_actor_mut(&mut self, id:ActorID) -> Result<&mut ActorHandle<T>, CreekError> {
        let valid = self.validate_actor_id(id);
        if valid.is_ok() {
            return Ok(&mut self.actors[id.index]);
        }
        else {
            return Err(valid.unwrap_err());
        }
    }

    pub fn validate_actor_id(&self, id:ActorID) -> Result<(), CreekError> {
        if id.index < self.actors.len() {
            if self.actors[id.index].id.gen == id.gen {
                return Ok(());
            }
            return Err(CreekError::ActorIdInvalidated);
        }
        Err(CreekError::ActorDoesNotExistAtIndex)
    }

    pub fn propagate_events(&mut self) {
        let mut actions_pending = Vec::<CreekAction>::new();
        for handle in &self.actors {
            if let Some(actor) = &handle.inner {
                for event in &self.events {
                    if let Some(id) = event.target {
                        if handle.id != id {
                            continue;
                        }
                    }
                    if let Some(actions) = actor.borrow().propogate_global_event(event) {
                        actions_pending.append(&mut actions.clone());
                    }
                }
            }
        }
        println!("{:?}", actions_pending);
        for action in actions_pending {
            let mut actor_handle = self.get_actor_mut(action.target);
            if let Ok(handle) = actor_handle {
                match action.action_type {
                    CreekActionType::Destroy => {
                        println!("REMOVE");
                        if let Some(rc) = &handle.inner {
                            mem::drop(rc);
                        }
                        handle.inner = None;
                    }
                }
            }
        }
        self.events.clear();
    }
}
