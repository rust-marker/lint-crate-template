use marker_uitest::ui_test::*;
use std::env;

fn main() -> color_eyre::Result<()> {
    let mut config = marker_uitest::simple_ui_test_config!()?;

    // To use external crates in ui tests, they need to be defined in a `Cargo.toml`
    // of a valid crate. The line below will use the `Cargo.toml` if this lint crate.
    // config.dependencies_crate_manifest_path = Some("./Cargo.toml".into());

    config.filter(r"\\/", "/");
    config.filter(r"\\\\", "/");

    run_tests_generic(
        vec![config],
        default_file_filter,
        default_per_file_config,
        status_emitter::Text::quiet(),
    )
}
