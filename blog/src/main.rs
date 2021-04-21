use blog::blog;
fn main() {
    let mut post = blog::Post::new();

    post.add_text("It's been an awesome year");

    let post = post.request_review();

    let post = post.request_review();

    let post = post.reject();

    let post = post.request_review();

    let post = post.request_review();

    let post = post.approve();

  

    assert_eq!(post.content(), "It's been an awesome year");
    println!("{}", post.content())
}