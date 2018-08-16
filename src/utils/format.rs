#[macro_export]
macro_rules! format_output {
    ($color:expr, $title:expr, $output:expr) => {
        format!("{{fg={} [{}] }} {:?}\n", $color, $title, $output)
    };
}
