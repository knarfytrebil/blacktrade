type Reducer =  fn(i32, i32) -> i32;

fn multiplier(state: i32, action: i32) -> i32 { state * action }
fn adder(state: i32, action: i32) -> i32 { state + action }

fn combined_reducer(reducers: Vec<Reducer>) -> Box<Fn(i32, i32) -> i32> {
    Box::new(|state, action| {
        reducers.iter().fold(0, |state, action| {
            reducer(state, action)
        })
    })
}

fn main() {
    println!("{}", combined_reducer(vec![multiplier, adder])(1, 3));
}
