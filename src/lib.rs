use std::ffi::c_void;
use std::ffi::CStr;

use imgui::{im_str, ImStr};

pub extern crate imgui_filedialog_sys as sys;

pub struct Context {
    ptr: *mut sys::ImGuiFileDialog,
}

fn ptr_to_string(ptr: *mut std::os::raw::c_char) -> String {
    unsafe {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

#[must_use]
impl Context {
    fn new() -> Self {
        let igfd_ctx = unsafe { sys::IGFD_Create() };

        unsafe {
            // Set extension colours
            let col = sys::ImVec4 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
                w: 1.0,
            };
            sys::IGFD_SetExtentionInfos(
                igfd_ctx,
                im_str!(".txt").as_ptr(),
                col,
                im_str!("Exciting file!").as_ptr(),
            );
        };

        Self { ptr: igfd_ctx }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sys::IGFD_Destroy(self.ptr) }
    }
}

pub struct FileDialog<'p> {
    id: &'p ImStr,
    context: Context,
}

impl<'p> FileDialog<'p> {
    pub fn create(id: &'p ImStr) -> Self {
        Self {
            context: Context::new(),
            id,
        }
    }

    pub fn open_modal(&self) {
        let filters = im_str!(".*,.txt,.md");

        unsafe {
            sys::IGFD_OpenPaneModal(
                self.context.ptr,
                self.id.as_ptr(),
                im_str!(" Choose a File").as_ptr(),
                filters.as_ptr(),
                im_str!(".").as_ptr(),
                im_str!("").as_ptr(),
                None,
                0.0,
                1,
                std::ptr::null_mut::<c_void>(),
                sys::ImGuiFileDialogFlags::from(0),
            );
        }
    }

    pub fn display(&self) -> bool {
        unsafe {
            sys::IGFD_DisplayDialog(
                self.context.ptr,
                self.id.as_ptr(),
                0,
                sys::ImVec2 { x: 200.0, y: 300.0 },
                sys::ImVec2 { x: 700.0, y: 500.0 },
            )
        }
    }

    pub fn close(&self) {
        unsafe {
            sys::IGFD_CloseDialog(self.context.ptr);
        }
    }

    pub fn is_ok(&self) -> bool {
        unsafe {
            sys::IGFD_IsOk(self.context.ptr)
        }
    }

    /// Path being browsed
    pub fn current_path(&self) -> Option<String> {
        if self.is_ok() {
            let ptr = unsafe { sys::IGFD_GetCurrentPath(self.context.ptr) };
            Some(ptr_to_string(ptr))
        } else {
            None
        }
    }

    pub fn selection(&self) -> Option<Selection> {
        Some(unsafe {
            Selection::new(sys::IGFD_GetSelection(self.context.ptr), &self.context)
        })
    }
}

pub struct Selection<'ui> {
    ptr: sys::IGFD_Selection,
    context: &'ui Context,
}

impl <'ui>Selection<'ui> {
    fn new(ptr: sys::IGFD_Selection, context: &'ui Context) -> Self {
        Selection{ptr, context}
    }
    pub fn files(&self) -> Vec<std::path::PathBuf> {
        let mut ret = vec![];
        for i in 0..self.ptr.count {
            let path = ptr_to_string(unsafe {
                (*self.ptr.table.offset(i as isize)).filePathName
            });
            let fixme = ptr_to_string(unsafe { sys::IGFD_GetCurrentPath(self.context.ptr) });
            // FIXME: Why does `path` contain same as filename?

            let fname = ptr_to_string(unsafe {
                (*self.ptr.table.offset(i as isize)).fileName
            });
            ret.push(std::path::PathBuf::new().join(fixme).join(fname));
        }
        ret
    }
}

impl Drop for Selection<'_> {
    fn drop(&mut self) {
        // TODO: Verify nothing needs dropped
    }
}
