use crate::PreprocessorContext;
use crate::PreprocessorResult;

pub fn json(_ctx: PreprocessorContext) -> crate::Result<PreprocessorResult> {
    println!("JSON preprocessor not implemented yet");
    Ok(PreprocessorResult {
        code: Default::default(),
    })
}
