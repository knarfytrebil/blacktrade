use handlebars::handlebars_helper;

// Handlebar Helpers
handlebars_helper!(stringify: |v: Json| {
    v.to_string()
});