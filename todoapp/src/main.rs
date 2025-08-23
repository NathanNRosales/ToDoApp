use popout::{Dialog, LogicalSize};

fn main() {
    Dialog::new()
        .with_line("Hello from popup!")
        .with_button("Close")
        .with_size(LogicalSize::new(200, 100))
        .show()
        .unwrap();
}
