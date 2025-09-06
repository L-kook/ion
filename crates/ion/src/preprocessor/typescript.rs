use crate::PreprocessorContext;
use crate::PreprocessorResult;

pub fn typescript(_ctx: PreprocessorContext) -> crate::Result<PreprocessorResult> {
    println!("TypeScript preprocessor not implemented yet");
    Ok(PreprocessorResult {
        code: Default::default(),
    })
}
