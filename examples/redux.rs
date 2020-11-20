extern crate redux;
extern crate tui;

use redux::{Reducer, Store};
use std::boxed::Box;
use std::default::Default;

use tui::backend::MouseBackend;
use tui::Terminal;

#[derive(Clone, Debug)]
struct Todo {
    name: &'static str,
}

#[derive(Clone, Debug)]
struct TodoState {
    todos: Vec<Todo>,
    terminal: Terminal<MouseBackend>,
}

impl TodoState {
    fn new() -> TodoState {
        TodoState { todos: vec![] }
    }

    fn push(&mut self, todo: Todo) {
        self.todos.push(todo);
    }
}

#[derive(Clone)]
enum TodoAction {
    Insert(&'static str),
}

impl Default for TodoState {
    fn default() -> Self {
        TodoState::new()
    }
}

impl Reducer for TodoState {
    type Action = TodoAction;
    type Error = String;

    fn reduce(&mut self, action: Self::Action) -> Result<Self, Self::Error> {
        match action {
            TodoAction::Insert(name) => {
                let todo = Todo { name: name };
                self.push(todo);
            }
        }

        Ok(self.clone())
    }
}

fn log(st: &Store<TodoState>) {
    println!("{:?}", st.get_state());
}

fn main() {
    // Terminal initialization
    let backend = MouseBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();

    let store: Store<TodoState> = Store::new(vec![]);
    store.subscribe(Box::new(|store, _| {
        log(store);
    }));

    let _ = store.dispatch(TodoAction::Insert("Clean the bathroom"));
    let _ = store.dispatch(TodoAction::Insert("Clean the kitchen"));
}
