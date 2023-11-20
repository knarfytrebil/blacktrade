use components::parsing::xml::get_u16_value;
use handlebars::{
    Context, 
    Handlebars, 
    HelperDef,
    HelperResult,
    Helper, Output, 
    RenderContext, 
    RenderError,
    Renderable,
    PathAndJson,
    ScopedJson,
    BlockParams,
    JsonValue,
};

use components::helpers::utils::create_block;

#[derive(Clone, Copy)]
pub struct HeightBufferHelper;

// define a custom helper
impl HelperDef for HeightBufferHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        // get parameter from helper or throw an error
        // debug!("cxt {:?}", ctx);
        // debug!("rcxt {:?}", rc);
        let param = h
            .param(0)
            .ok_or(RenderError::new("Param 0 is required for format helper."))?;

        // debug!("param {:?}", param);
        if param.value().is_array() {
            let lines = param.value().as_array().expect("Param 0 value error");
            let ctx_data = ctx.data().as_object().expect("Context data error");
            let props = ctx_data.get("props")
                .expect("Value Unpack Error: props")
                .as_object()
                .expect("Value Get Error: props");

            let area = props.get("area")
                .expect("Value Unpack Error: area")
                .as_object()
                .expect("Value Get Error: area");
 
            let height_u64 = get_u16_value(area, "height");
            let buffered_lines = match lines.len() > height_u64 as usize {
                true => lines[lines.len() - height_u64 as usize ..].to_vec(),
                false => lines.to_vec()
            };

            // debug!("buffered_lines {:?}", buffered_lines.len());

            let mut block = create_block(param);
            let s_json = ScopedJson::Derived(
                JsonValue::Array(buffered_lines)
            );
            let buffered_param = PathAndJson::new(
                param.relative_path().map(|p| p.to_owned()),
                s_json
            );

            // debug!("buffered_param {:?}", buffered_param);
            // debug!("helper {:?}", h);

            if let Some(block_param) = h.block_param() {
                debug!("block_param {:?}", block_param);
                let mut params = BlockParams::new();
                if buffered_param.context_path().is_some() {
                    params.add_path(block_param, Vec::with_capacity(0))?;
                } else {
                    params.add_value(block_param, buffered_param.value().clone())?;
                }
                block.set_block_params(params);
            }
            rc.push_block(block);
            if let Some(t) = h.template() {
                t.render(r, ctx, rc, out)?;
            };
            rc.pop_block();
            Ok(())
        } else {
            Err(RenderError::new("Param 0 must be an array"))
        }
    }
}

pub static HEIGHT_BUFFER_HELPER: HeightBufferHelper = HeightBufferHelper;