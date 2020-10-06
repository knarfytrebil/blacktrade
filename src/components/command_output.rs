use serde_json::{Value, json};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::Block;


use tui::widgets::{Paragraph, Wrap};
use tui::text::{Spans, Span};
use tui::Frame;

use structs::app::AppState;
use components::xml;

const DATA: &'static str = r#"
<Paragraph styles='{"wrap": {"trim": "true"}, "block": "default"}' scroll='true'>
    {{#each store.console_output_lines as |line| ~}}
        <Spans>
            <Span>{{line}}</Span>
        </Spans>
    {{/each}}
</Paragraph>"#;

pub fn render<B>(frame: &mut Frame<B>, store: &AppState, area: Rect)
where
    B: Backend,
{
    // let array = store.json_store["console_output_lines"].as_array().expect("Data Error");

    // let buf = get_buffer(
    //     area.height, 
    //     array.to_vec()
    // );

    let dom_root = xml::parse(
        DATA.to_string(), 
        &json!({
            "store": &store.json_store,
            "metrics": {
                "height": area.height,
                "width": area.width
            }
        })
    );

    let widget = match xml::create_element(dom_root) {
        xml::El::Paragraph(p) => p,
        _ => panic!("XML Parse Error !")
    };

    // let text: Vec<Spans> = buf.iter()
    //     .map(|l| { 
    //         Spans::from(Span::raw(l.as_str().expect("Data Error"))) 
    //     }).collect();

    // let widget = Paragraph::new(text)
    //     .block(Block::default())
    //     .wrap(Wrap { trim: true });

    frame.render_widget(widget, area);
}

// match styles {
//     Some(style) => {
//         match style {
//             Value::Object(obj) => {
//                 for (key, value) in obj.iter() {
//                     match key.as_str() {
//                         "block" => {
//                             match value.as_str().expect("Unexpected format styles value") {
//                                 "default" => { paragraph_node.block(Block::default()); }
//                                 &_ => { debug!("Unknown style Value") }
//                             }
//                         },
//                         // "scroll" => {},
//                         // "wrap" => {},
//                         &_ => { debug!("Unknown style attr") }
//                     }
//                 }
//             },
//             _ => { panic!("Unknown Style Format") }
//         }
//     }
//     None => {
//         El::Paragraph(paragraph_node)
//     }
// }
//

