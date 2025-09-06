use crate::DynResolver;
use crate::ResolverContext;
use crate::ResolverResult;

pub async fn run_resolvers(
    resolvers: &Vec<DynResolver>,
    ctx: ResolverContext,
) -> crate::Result<ResolverResult> {
    for resolver in resolvers {
        match resolver(ResolverContext {
            fs: ctx.fs.clone(),
            specifier: ctx.specifier.clone(),
            from: ctx.from.clone(),
        })
        .await?
        {
            Some(result) => {
                return Ok(result);
            }
            None => continue,
        }
    }

    return Err(crate::Error::FileNotFound(ctx.specifier));
}
