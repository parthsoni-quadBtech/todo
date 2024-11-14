use candid::CandidType;
use ic_cdk::{query,update};
use std::cell::RefCell;
use serde::Deserialize;


#[derive(CandidType, Deserialize, Clone)]
pub struct Note {
    pub id: u8,
    pub content: String
}
#[derive(Default)]
pub struct TodoState {
    pub list: Vec <Note>,
    pub total_todo:u8
}
thread_local! {
    static STATE: RefCell<TodoState> = RefCell::default();
}
#[update]
pub fn add_note(content: String) -> Result<(), String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();

        let new_note = Note {
            id: state.total_todo,
            content,
        };
        state.list.push(new_note);
        state.total_todo += 1;

        Ok(())
    })
}
#[query]
pub fn get_notes() -> Vec<Note> {
    STATE.with(|state| {
        state.borrow().list.clone()
    })
}
#[update]
pub fn delete_note(id: u8) -> Result<(), String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        
        if let Some(pos) = state.list.iter().position(|note| note.id == id) {
            state.list.remove(pos);
            Ok(())
        } else {
            Err("Note with the given ID not found".to_string())  // ID doesn't exist
        }
    })
}
