use warp::{Filter, http::Response};



#[tokio::main]
async fn main() {
    let custom = warp::any().map(|| {
        Response::builder()
            .header("my-custom-header", "some-value")
            .body(
                r#"
                    <title>GCD Calculator</title>
                    <form action="/gcd" method="post">
                        <input type="text" name="" />
                        <input type="text" name="" />
                        <button type="submit">Compute GCD</button>
                    </form>
                "#
            )
    });

    warp::serve(custom).run(([127, 0, 0, 1], 3030)).await;
}