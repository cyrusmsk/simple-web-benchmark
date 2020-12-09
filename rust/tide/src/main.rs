use async_std::task;

fn main() -> Result<(), std::io::Error> {
    let pid = std::process::id().to_string();
    std::fs::write(".pid", &pid).expect("Unable to write file");
    println!("Master {} is running", pid);

    task::block_on(async {
        let mut app = tide::new();
        app.at("/").get(|_| async { Ok("Hello, world!") });
        app.at("/greeting/:name")
            .get(|req: tide::Request<()>| async move {
                Ok(format!("Hello, {}", req.param("name").unwrap()))
            });
        app.listen("127.0.0.1:3000").await?;
        Ok(())
    })
}
