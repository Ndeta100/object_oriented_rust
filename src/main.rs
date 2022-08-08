use object_oriented::{Button, Screen, SelectBox};
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
}
