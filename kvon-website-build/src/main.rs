fn main() {
	use mini_builder_rs::value::Value;
	mini_builder_rs::builder::Builder::new(
		Some("../html-src".into()),
		Some("../html-templates".into()),
		Some("../html-generated".into()),
		None,
		Default::default(),
	)
	.unwrap()
	.add_variables(&[(
		"rust_library".to_string(),
		Value::List(vec![
			Value::text("kvon-rs"),
			Value::text("https://crates.io/crates/kvon-rs"),
		]),
	)])
	.add_variables(&[(
		"java_library".to_string(),
		Value::List(vec![
			Value::text("kvon-java"),
			Value::text("https://github.com/kvon-format/kvon-java"),
		]),
	)])
	.watch()
	.unwrap();
}
