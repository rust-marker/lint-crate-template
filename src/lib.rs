use marker_api::{LintPass, LintPassInfo, LintPassInfoBuilder};

#[derive(Default)]
struct MyLintPass {}
marker_api::export_lint_pass!(MyLintPass);

marker_api::declare_lint! {
    /// # What it does
    /// Here you can explain what your lint does. The description supports normal
    /// markdown.
    ///
    /// # Example
    /// ```rs
    /// // Bad example
    /// ```
    ///
    /// Use instead:
    /// ```rs
    /// // Good example
    /// ```
    MY_LINT,
    Warn,
}

impl LintPass for MyLintPass {
    fn info(&self) -> LintPassInfo {
        LintPassInfoBuilder::new(Box::new([MY_LINT])).build()
    }
}
