use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            background: "rgb(240, 240, 240)",
            padding: "20",
            label {
                font_size: "24",
                font_weight: "bold",
                "gtypist Freya GUI"
            }
            label {
                font_size: "16",
                "Welcome to the gtypist typing tutor!"
            }
            rect {
                direction: "horizontal",
                padding: "10",
                Button {
                    onclick: move |_| count += 1,
                    label {
                        "Count: {count}"
                    }
                }
            }
        }
    )
}
