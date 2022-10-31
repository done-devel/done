fn main() -> Result<(), Box<dyn std::error::Error>> {
	tonic_build::configure()
		.protoc_arg("--experimental_allow_proto3_optional")
		.type_attribute(".", "#[derive(serde::Deserialize)]")
		.compile(&["provider.proto"], &["."])?;
	glib_build_tools::compile_resources(
		"data/resources/",
		"data/resources/resources.gresource.xml",
		"resources.gresource",
	);
	Ok(())
}
