type Reducer =  Fn(i32, i32) -> Result<i32, String>;

// fn multiplier(state: i32, action: i32) -> i32 { state * action }
// fn adder(state: i32, action: i32) -> i32 { state + action }

fn combined_reducer(reducers: Vec<Box<Reducer>>) -> Box<Reducer> {
    Box::new(move |mut state, action| {
        for reducer in &reducers {
            state = reducer(state, action).unwrap()
        }
        Ok(state)
    })
}

fn main() {
    let multiply_reducer = |state: i32, action: i32| -> Result<i32, String> { 
        Ok(state * action) 
    };
    let plus_reducer = |state: i32, action: i32| -> Result<i32, String> { 
        Ok(state + action)
    };
    let reducers: Vec<Box<Reducer>> = vec![
        Box::new(plus_reducer),
        Box::new(multiply_reducer)
    ];
    let num = combined_reducer(reducers)(1, 3).unwrap();
    println!("{}", num);
}
