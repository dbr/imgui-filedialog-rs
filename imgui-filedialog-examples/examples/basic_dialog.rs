use imgui::im_str;
mod support;

use imgui_filedialog::sys;

fn main() {
    let system = support::init(file!());

    // Create dialog instance to store state between frame
    let fd = imgui_filedialog::FileDialog::create(im_str!("Test"));

    system.main_loop(move |_run, ui| {
        imgui::Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], imgui::Condition::FirstUseEver)
            .build(ui, || {
                if ui.button(im_str!("Select a file"), [0.0, 0.0]) {
                    // If button is clicked, open the modal dialog
                    fd.open_modal();
                }

                // Render the dialog (as needed)
                if fd.display() {
                    // If `display()` returns true, either ok or
                    // cancel has been clicked!
                    println!("Was browsing in folder {:?}", fd.current_path());
                    if fd.is_ok() {
                        println!("Ok was clicked! {:?}", fd.selection().unwrap().files());
                    }
                    fd.close();
                }
            });
    });
}
