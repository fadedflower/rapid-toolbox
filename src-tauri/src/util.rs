use std::io;
use std::path::Path;
use std::fs::read as fs_read;
use base64::prelude::*;

fn encode_image_url_from_bytes<T: AsRef<[u8]>>(image_data: T, mime_type: String) -> String {
    format!("data:{};base64,{}", mime_type, BASE64_STANDARD.encode(image_data.as_ref()))
}

pub fn encode_image_url_from_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let path = path.as_ref();
    let image_data = fs_read(path)?;
    let mime_type;
    if let Some(ext) = path.extension() {
        let ext_str: &str = &ext.to_string_lossy().to_ascii_lowercase();
        mime_type = match ext_str {
            "bmp" => "image/bmp",
            "gif" => "image/gif",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "svg" => "image/svg+xml",
            "webp" => "image/webp",
            "tiff" | "tif" => "image/tiff",
            _ => "application/octet-stream"
        };
    }
    else {
        mime_type = "application/octet-stream";
    }
    Ok(encode_image_url_from_bytes(image_data, mime_type.to_string()))
}

#[cfg(windows)]
pub fn encode_image_url_from_app_icon<P: AsRef<Path>>(path: P) -> Option<String> {
    use std::mem::{MaybeUninit, size_of, swap};
    use std::io::Cursor;
    use image::{RgbaImage, ImageFormat};
    use windows::core::HSTRING;
    use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO};
    use windows::Win32::UI::Shell::{SHGetFileInfoW, SHGetImageList, SHFILEINFOW, SHGFI_SYSICONINDEX, SHIL_EXTRALARGE};
    use windows::Win32::UI::Controls::{IImageList, ILD_TRANSPARENT};
    use windows::Win32::Graphics::Gdi::{DeleteObject, GetObjectW, GetDIBits, GetDC, ReleaseDC, BITMAP, BITMAPINFOHEADER, BITMAPINFO, HGDIOBJ, BI_RGB, DIB_RGB_COLORS};
    use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;

    let mut hicon = HICON::default();
    unsafe {
        let mut file_info = SHFILEINFOW::default();
        // get icon index from file path
        if SHGetFileInfoW(
            &HSTRING::from(path.as_ref()),
            FILE_FLAGS_AND_ATTRIBUTES(0),
            Some(&mut file_info),
            size_of::<SHFILEINFOW>() as u32,
            SHGFI_SYSICONINDEX
            )== 0 {
            return None;
        }
        // get icon handle
        if SHGetImageList(SHIL_EXTRALARGE as i32).and_then(|image_list: IImageList| {
            image_list.GetIcon(file_info.iIcon, ILD_TRANSPARENT.0).and_then(|icon| {
                hicon = icon;
                Ok(())
            })
        }).is_err() {
            return None;
        }
    }

    let mut icon_info: MaybeUninit<ICONINFO> = MaybeUninit::uninit();
    let mut bitmap: MaybeUninit<BITMAP> = MaybeUninit::uninit();
    unsafe {
        // get bitmap handle of the icon
        GetIconInfo(hicon, icon_info.as_mut_ptr()).unwrap();
        DestroyIcon(hicon).unwrap();
    }
    let icon_info = unsafe { icon_info.assume_init_ref() };
    unsafe {
        let bitmap_size = GetObjectW(HGDIOBJ::from(icon_info.hbmColor), i32::try_from(size_of::<BITMAP>()).unwrap(), Some(bitmap.as_mut_ptr() as *mut _));
        assert_eq!(bitmap_size, i32::try_from(size_of::<BITMAP>()).unwrap());
    }
    let bitmap = unsafe { bitmap.assume_init_ref() };

    let dc = unsafe { GetDC(None) };
    let bitmap_width = usize::try_from(bitmap.bmWidth).unwrap();
    let bitmap_height = usize::try_from(bitmap.bmHeight).unwrap();
    let bitmap_buf_size = bitmap_width * bitmap_height * 4;
    let mut bitmap_buf = vec![0u8; bitmap_buf_size];
    let mut bitmap_info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: bitmap.bmWidth,
            biHeight: -bitmap.bmHeight, // negative height for top-down bitmap
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            ..Default::default()
        },
        ..Default::default()
    };
    unsafe {
        // get bitmap data
        let scan_line = GetDIBits(
            dc,
            icon_info.hbmColor,
            0,
            bitmap.bmHeight as u32,
            Some(bitmap_buf.as_mut_ptr() as *mut _),
            &mut bitmap_info,
            DIB_RGB_COLORS);
        assert_eq!(scan_line, bitmap.bmHeight);
        assert_eq!(ReleaseDC(None, dc), 1);
        DeleteObject(HGDIOBJ::from(icon_info.hbmColor)).unwrap();
        DeleteObject(HGDIOBJ::from(icon_info.hbmMask)).unwrap();
    }
    // transform from [b,g,r,a] to [r,g,b,a]
    for chunk in bitmap_buf.chunks_exact_mut(4) {
        let [b, _, r, _] = chunk else { unreachable!() };
        swap(b, r);
    }
    let img = RgbaImage::from_vec(bitmap.bmWidth as u32, bitmap.bmHeight as u32, bitmap_buf).expect("Invalid bitmap data");
    let mut encoded_img_buf: Vec<u8> = Vec::new();
    encoded_img_buf.reserve(bitmap_width * bitmap_height * 2); // reserve space for avoiding reallocations

    img.write_to(&mut Cursor::new(&mut encoded_img_buf), ImageFormat::Png).expect("Failed to encode image");    
    Some(encode_image_url_from_bytes(encoded_img_buf, "image/png".to_string()))
}