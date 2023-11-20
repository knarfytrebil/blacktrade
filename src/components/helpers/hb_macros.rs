use handlebars::handlebars_helper;
use components::ele::powerline_symbol as PowerlineSym;

// Handlebar Helpers
handlebars_helper!(stringify: |v: Json| {
    v.to_string()
});

// 
handlebars_helper!(powerline_symbol: |v: String| {
    match v.as_str() {
        "right_arrow_line" => PowerlineSym::RIGHT_ARROW_LINE.to_string(),
        "right_arrow" => PowerlineSym::RIGHT_ARROW.to_string(),
        _ => String::from("") 
    }
});