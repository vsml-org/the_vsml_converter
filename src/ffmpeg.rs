use ffmpeg_next::sys as ffmpeg_sys;
use std::ffi::CString;
use std::ptr;

pub fn get_duration(file_path: &str) -> f64 {
    // FFmpegライブラリを初期化
    unsafe {
        ffmpeg_sys::av_register_all();
        ffmpeg_sys::avformat_network_init();
    }

    let mut format_context = unsafe { ffmpeg_sys::avformat_alloc_context() };
    let file_path_cstr = CString::new(file_path).unwrap();
    if unsafe {
        ffmpeg_sys::avformat_open_input(
            &mut format_context,
            file_path_cstr.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        )
    } != 0
    {
        return 0.0;
    }
    if unsafe { ffmpeg_sys::avformat_find_stream_info(format_context, ptr::null_mut()) } < 0 {
        unsafe {
            ffmpeg_sys::avformat_close_input(&mut format_context);
        }
        return 0.0;
    }

    let duration: f64;
    unsafe { duration = (*format_context).duration as f64 / ffmpeg_sys::AV_TIME_BASE as f64 }

    // メモリを解放
    unsafe {
        ffmpeg_sys::avformat_close_input(&mut format_context);
    };

    duration
}
