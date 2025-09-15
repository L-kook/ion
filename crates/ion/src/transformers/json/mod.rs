use crate::JsTransformer;
use crate::TransformerContext;
use crate::TransformerResult;

pub fn json() -> JsTransformer {
    JsTransformer {
        kind: "json".to_string(),
        transformer: Box::new(transformer),
    }
}

fn transformer(ctx: TransformerContext) -> crate::Result<TransformerResult> {
    let mut code = String::from_utf8(ctx.content)?;
    // TODO escapes
    code = format!("export default JSON.parse(`{}`)", code);

    Ok(TransformerResult { code })
}
