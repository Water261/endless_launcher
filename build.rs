fn main() -> Result<(), slint_build::CompileError> {
	let config = slint_build::CompilerConfiguration::new()
		.with_style("fluent".into());

	slint_build::compile_with_config("ui/app.slint", config)?;

	Ok(())
}