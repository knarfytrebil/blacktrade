use itertools::Itertools;
use walkdir::{WalkDir, DirEntry};
use handlebars::Handlebars;

// Get Template files from a certain dir
pub fn get_templates<'a>(template_path: &'a str, suffix: &'a str) -> Vec<DirEntry> {
    let entries = WalkDir::new(template_path);
    entries.into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(suffix))
        .collect_vec()
}

// load the templates
pub fn load_templates(templates: Vec<DirEntry>, reg: &mut Handlebars<'_>) {
    for entry in templates {
        let prefix: Vec<&str> = entry
            .file_name()
            .to_str()
            .expect("prefix split error")
            .split(".")
            .collect();
        let _ = reg.register_template_file(
            prefix[0],
            entry.path()).expect("template registration error");
    }
}