use object_oriented::{Button, Post, Screen, SelectBox};
fn main() {
    let screen = Screen {
        component: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                option: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: "Ok".to_string(),
            }),
        ],
    };
    screen.run();
    let mut post = Post::new();

    post.add_text("what's up Ndeta");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("What's up rust", post.content());
}
