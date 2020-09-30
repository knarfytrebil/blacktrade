use treexml::{Document, Element};

use tui::widgets::Paragraph;
use tui::text::Spans;

enum El {
    Div(Paragraph<'static>),
    Text(Spans<'static>)
}

const DATA: &'static str = r#"
<Paragraph>
    <Spans>{store.command}</Spans>
</Paragraph>"#;

fn parse_xml(xml: &'static str) -> Element {
    let doc = Document::parse(xml.as_bytes()).unwrap();
    doc.root.unwrap()
}

fn create_element(el: Element) -> El {
    let children: Vec<El> = match el.children.len() > 0 {
        true =>  { 
            el.children.into_iter().map(|chd_el| {
                create_element(chd_el)
            }).collect()
        },
        false => vec!()
    };

    let parent = match el.name.as_str() {
        "Paragraph" => {
            El::Div(Paragraph::new(vec!()))
        },
        "Spans" => { 
            El::Text(Spans::from(""))
        },
        &_ => { panic!("Unknown DOM Token") }
    };

    parent
}
