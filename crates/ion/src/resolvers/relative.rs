use crate::ResolverContext;
use crate::ResolverResult;

pub fn relative(_ctx: ResolverContext) -> crate::Result<ResolverResult> {
    println!("Relative resolver not implemented yet");
    Ok(ResolverResult {
        code: Default::default(),
        kind: "js".to_string(),
    })
}
