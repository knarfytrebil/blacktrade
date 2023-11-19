use handlebars::Context;
use handlebars::Handlebars;
use handlebars::Helper;
use handlebars::HelperResult;
use handlebars::Output;
use handlebars::RenderContext;
use handlebars::RenderError;

#[derive(Debug, Error)]
enum HelperError {
    #[error("missing param {position} '{name}' of '{helper_signature}'")]
    MissingParameter {
        position: usize,
        name: String,
        helper_signature: String,
    },
}

#[derive(Clone, Copy)]
pub struct WithHelper;

impl HelperDef for WithHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        r: &'reg Registry<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h
            .param(0)
            .ok_or(RenderErrorReason::ParamNotFoundForIndex("with", 0))?;

        if param.value().is_truthy(false) {
            let mut block = create_block(param);

            if let Some(block_param) = h.block_param() {
                let mut params = BlockParams::new();
                if param.context_path().is_some() {
                    params.add_path(block_param, Vec::with_capacity(0))?;
                } else {
                    params.add_value(block_param, param.value().clone())?;
                }

                block.set_block_params(params);
            }

            rc.push_block(block);

            if let Some(t) = h.template() {
                t.render(r, ctx, rc, out)?;
            };

            rc.pop_block();
            Ok(())
        } else if let Some(t) = h.inverse() {
            t.render(r, ctx, rc, out)
        } else if r.strict_mode() {
            Err(RenderError::strict_error(param.relative_path()))
        } else {
            Ok(())
        }
    }
}