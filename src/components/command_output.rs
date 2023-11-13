use serde_json::json;
use ratatui::layout::Rect;
// use tui::widgets::Block;

// use tui::text::{Span, Spans};
// use tui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

use components::xml;
use structs::app::AppState;

const DATA: &'static str = r#"
<Paragraph 
    styles='{"fg": {"Color": "red"}}' 
    scroll='{"offset": [1, 20]}' wrap='{"trim": true}' alignment='{"position" : "Left"}'>
    {{#each store.console_output_lines as |line| ~}}
        <Line>
            <Span>{{line}}</Span>
        </Line>
    {{/each}}
</Paragraph>"#;

pub fn render(frame: &mut Frame, store: &AppState, area: Rect)
{
    // let array = store.json_store["console_output_lines"]
    //     .as_array()
    //     .expect("Data Error");

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
        }),
    );

    let widget = match xml::create_element(dom_root) {
        xml::El::Paragraph(p) => p,
        _ => panic!("XML Parse Error !"),
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
