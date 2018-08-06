type Reducer =  Fn(i32, i32) -> i32;

// fn multiplier(state: i32, action: i32) -> i32 { state * action }
// fn adder(state: i32, action: i32) -> i32 { state + action }

// fn combined_reducer(reducers: &Vec<Reducer>) -> Box<Fn(i32, i32) -> i32> {
fn combined_reducer(reducers: Vec<Reducer>) -> Reducer {
    |mut state, action| {
        for reducer in reducers {
            state = reducer(state, action)    
        }
        state
    }
}

fn main() {
    let multiply_reducer = |state: i32, action: i32| -> i32 { state * action };
    let adder = |state: i32, action: i32| -> i32 { state + action };

    let reducers: Vec<Reducer> = vec![adder];
    let num = combined_reducer(reducers)(1, 3);
    println!("{}", num);
}
