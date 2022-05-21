use std::{rc::Rc, cell::{RefCell, Ref, RefMut} };

use crate::{GlobalEvent, GlobalEventType, CreekError, CreekAction, CreekActionType};

pub trait CreekEvent {}


pub trait Actor {
    type Event;
    fn receive_event(&mut self, event:Self::Event);
    fn recieve_global_event(&mut self, event:GlobalEvent) {}
    fn added_to_creek(&mut self) {}
    fn removed_from_creek(&mut self) {}
    fn get_creek_actions(&self) -> &Vec<CreekAction>;
    fn set_id(&mut self, id:ActorID) -> Result<(), ()> {
        Err(())
    }
    fn get_id(&self) -> Option<ActorID>;
    fn creek_action(&self, action_type:CreekActionType) -> Result<CreekAction, CreekError> {
        if let Some(id) = self.get_id() {
            Ok(CreekAction {
                action_type,
                target: id,
            })
        } else {
            Err(CreekError::ActorDoesNotStoreId)
        }
    }
}

pub trait ActorTypes {
    fn propogate_global_event(&self, event:&GlobalEvent) -> Option<&Vec<CreekAction>>;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ActorID {
    pub index: usize,
    pub gen: usize,
}

impl Default for ActorID {
    fn default() -> ActorID {
        ActorID { index: 0, gen: 0 }
    }
}

impl ActorID {
    pub fn empty() -> ActorID {
        ActorID { index: 0, gen: usize::MAX }
    }
}

#[derive(Clone, Debug)]
pub struct ActorHandle<T: ActorTypes + Clone> {
    pub id: ActorID,
    pub inner: Rc<Option<RefCell<T>>>
}

impl<T: ActorTypes + Clone> Default for ActorHandle<T> {
    fn default() -> Self {
        Self {
            id: ActorID::empty(),
            inner: Rc::new(None)
        }
    }
}

impl<T: ActorTypes + Clone> ActorHandle<T> {
    pub fn new(actor: T) -> Self {
        ActorHandle {
            id: ActorID::default(),
            inner: Rc::new(Some(RefCell::new(actor))),
        }
    }

    pub fn empty() -> Self {
        ActorHandle {
            id: ActorID::empty(),
            inner: Rc::new(None),
        }
    }

    pub fn borrow_actor(&self) -> Option<Ref<T>> {
        if let Some(v) = *self.inner {
            Some(v.borrow())
        }
        else {
            None
        }
    }

    pub fn borrow_actor_mut(&mut self) -> Option<RefMut<T>> {
        if let Some(v) = *self.inner {
            Some(v.borrow_mut())
        }
        else {
            None
        }
    }

    // pub fn get_actor<F>(&self, event_closure: F) 
    //     where F: Fn(&T) -> ()
    // {
    //     if let Some(a_type) = self.borrow_actor() {
    //         event_closure(&*a_type);
    //     }
    // }

    pub fn edit_actor<F>(&mut self, mut closure: F) 
        where F: FnMut(&mut T) -> ()
    {
        if let Some(mut a_type) = self.borrow_actor_mut() {
            closure(&mut *a_type);
        }
    }
}