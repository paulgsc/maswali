fn me_not_async() {
	let foo = "bar";
	println!("foo: {:?}", foo);
}

#[tokio::main]
async fn main() {
	tokio::spawn(async move {
		me_not_async();
	});
}
