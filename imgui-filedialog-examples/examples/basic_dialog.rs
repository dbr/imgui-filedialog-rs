/*
void drawGui()
{
  // open Dialog Simple
  if (ImGui::Button("Open File Dialog"))
    ImGuiFileDialog::Instance()->OpenDialog("ChooseFileDlgKey", "Choose File", ".cpp,.h,.hpp", ".");

  // display
  if (ImGuiFileDialog::Instance()->Display("ChooseFileDlgKey"))
  {
    // action if OK
    if (ImGuiFileDialog::Instance()->IsOk())
    {
      std::string filePathName = ImGuiFileDialog::Instance()->GetFilePathName();
      std::string filePath = ImGuiFileDialog::Instance()->GetCurrentPath();
      // action
    }

    // close
    ImGuiFileDialog::Instance()->Close();
  }
}
*/

use std::ffi::c_void;

use imgui::im_str;
mod support;

use imgui_filedialog::sys;

fn main() {
    let system = support::init(file!());

    let igfd_ctx = unsafe { sys::IGFD_Create() };

    unsafe {
        // Set extension colours
        let col = sys::ImVec4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        sys::IGFD_SetExtentionInfos(
            igfd_ctx,
            im_str!(".txt").as_ptr(),
            col,
            im_str!("").as_ptr(),
        );
    }

    system.main_loop(move |_run, ui| {
        imgui::Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], imgui::Condition::FirstUseEver)
            .build(ui, || {
                if ui.button(im_str!("Select a file"), [0.0, 0.0]) {
                    unsafe {
                        let filters = im_str!(".*,.txt,.md");
                        sys::IGFD_OpenPaneDialog(
                            igfd_ctx,
                            im_str!("ChooseFileDlgKey").as_ptr(),
                            im_str!(" Choose a File").as_ptr(),
                            filters.as_ptr(),
                            im_str!(".").as_ptr(),
                            im_str!("").as_ptr(),
                            None,
                            0.0,
                            1,
                            0 as *mut c_void,
                            sys::ImGuiFileDialogFlags::from(0),
                        );
                    }
                }
                unsafe {
                    sys::IGFD_DisplayDialog(
                        igfd_ctx,
                        im_str!("ChooseFileDlgKey").as_ptr(),
                        0,
                        sys::ImVec2 { x: 200.0, y: 300.0 },
                        sys::ImVec2 { x: 700.0, y: 500.0 },
                    );
                    use std::ffi::CStr;
                    if sys::IGFD_IsOk(igfd_ctx) {
                        println!("Cool");
                        let filename_raw = sys::IGFD_GetFilePathName(igfd_ctx);
                        let filename = CStr::from_ptr(filename_raw);
                        dbg!(&filename);
                    }
                }
            });
    });
    unsafe {
        sys::IGFD_Destroy(igfd_ctx);
    }
}
