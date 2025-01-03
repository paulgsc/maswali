use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
	tokio::spawn(async {
		let rc = Rc::new("hello");

		yield_now().await;
		println!("{}", rc);
	});
}
