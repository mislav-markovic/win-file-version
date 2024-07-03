use std::{ffi::c_void, path::Path, ptr::null_mut};

use windows::{
    core::{Result, PSTR},
    Win32::{
        Foundation::GetLastError,
        Storage::FileSystem::{GetFileVersionInfoA, GetFileVersionInfoSizeA, VerQueryValueA},
    },
};
const TEST_PATH: &'static str = r#"C:\git\PumaHost\Output\Publish\Result\Server\bin\Avl.PumaHost.ResultStorage.Server.ResultService.WebApi.exe"#;
const TEST_PATH_2: &'static str = r#"C:\git\file-version\test_file.exe"#;

fn main() -> Result<()> {
    println!("Hello, world!");

    let version = get_fixed_version(TEST_PATH_2);
    println!("Version: {version}");
    Ok(())
}

fn get_fixed_version(p: impl AsRef<Path>) -> String {
    let p = p.as_ref();

    let mut path_str = p.to_str().unwrap().to_owned();
    let path_ptr = path_str.as_mut_ptr();
    let pstr = PSTR::from_raw(path_ptr);

    println!("Given path: {path_str}");

    unsafe {
        let size = GetFileVersionInfoSizeA(pstr, None);
        let buffer_size = usize::try_from(size).unwrap();
        println!("File version info size: {size}");

        if size == 0 {
            let err = GetLastError();
            println!("Fetching size caused error: {err:?}");
            return "".into();
        }

        let mut buffer = vec![0u8; buffer_size];
        let data_ptr: *mut c_void = buffer.as_mut_ptr() as *mut _;

        let file_ver_result = GetFileVersionInfoA(pstr, 0, size, data_ptr);
        file_ver_result.unwrap();

        println!("Read file info version!");

        let mut query_str = QUERY_ROOT.to_owned();
        let query_ptr = path_str.as_mut_ptr();
        let query_pstr = PSTR::from_raw(path_ptr);
        let mut result_size = 0u32;
        let mut result_buffer: *mut c_void = null_mut();
        let query_version_result = VerQueryValueA(
            data_ptr,
            query_pstr,
            &mut result_buffer as *mut *mut _,
            &mut result_size as *mut _,
        );

        if query_version_result.into() {
            println!("Version successfully queried!");
        } else {
            let err = GetLastError();
            println!("Querying for version caused error: {err:?}");
            return "".into();
        }

    }

    "TODO".into()
}

fn get_string_version(p: impl AsRef<Path>) -> String {
    let p = p.as_ref();

    let mut path_str = p.to_str().unwrap().to_owned();
    let path_ptr = path_str.as_mut_ptr();
    let pstr = PSTR::from_raw(path_ptr);

    println!("Given path: {path_str}");

    unsafe {
        let size = GetFileVersionInfoSizeA(pstr, None);
        let buffer_size = usize::try_from(size).unwrap();
        println!("File version info size: {size}");

        if size == 0 {
            let err = GetLastError();
            println!("Fetching size caused error: {err:?}");
            return "".into();
        }

        let mut buffer = vec![0u8; buffer_size];
        let data_ptr: *mut c_void = buffer.as_mut_ptr() as *mut _;

        let file_ver_result = GetFileVersionInfoA(pstr, 0, size, data_ptr);
        file_ver_result.unwrap();

        println!("Read file info version!");

        let mut translation_query = QUERY_TRANSALTIONS.to_owned();
        let translation_query_ptr = translation_query.as_mut_ptr();
        let translation_query_pstr = PSTR::from_raw(translation_query_ptr);
        let mut translation_query_len = 0u32;
        let translation_query_len_ptr = &mut translation_query_len as *mut u32;
        let mut translations_ptr: *mut lang_codepage = null_mut() as *mut _;
        let query_translations_result = VerQueryValueA(
            data_ptr,
            translation_query_pstr,
            (&mut (translations_ptr as *mut c_void)) as *mut *mut c_void,
            translation_query_len_ptr,
        );

        if query_translations_result.into() {
            println!("Translations successfully queried!");
        } else {
            let err = GetLastError();
            println!("Querying for translations caused error: {err:?}");
            return "".into();
        }

        // let query_version_result = VerQueryValueA(data_ptr, lpsubblock, lplpbuffer, pulen);
    }

    "TODO".into()
}

const QUERY_TRANSALTIONS: &'static str = r#"\VarFileInfo\Translation"#;
const QUERY_ROOT: &'static str = r#"\"#;
const QUERY_FILE_VERSION_FORMAT: &'static str = r#"\StringFileInfo\{:04x}{:04x}\FileVersion"#;

#[repr(C)]
struct lang_codepage {
    lang: usize,
    code_page: usize,
}
