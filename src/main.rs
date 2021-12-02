extern crate image;
extern crate ocl;

use ocl::{core, flags};

fn main() {
    let args = mdbrot::parse_args();
    mandelbrot_gpu(
        args[0].width().unwrap(),
        args[1].height().unwrap(),
        args[2].path().unwrap(),
    )
    .unwrap();
}

fn mandelbrot_gpu(w: u32, h: u32, path: std::path::PathBuf) -> ocl::Result<()> {
    let _image_size = w * h * 4;
    let _workgroup_size = 512;
    let _group_count = (_image_size - 1) / _workgroup_size + 1;
    let _global_work_size = _group_count * _workgroup_size;

    let platform_ids = core::get_platform_ids()?;
    assert!(platform_ids.len() > 0, "No compute platform found");
    let platform_id = platform_ids[0];

    let device_ids = core::get_device_ids(platform_id, Some(flags::DEVICE_TYPE_GPU), Some(1))?;
    assert!(device_ids.len() > 0, "No GPU device found");
    let device_id = device_ids[0];

    let context_props = core::ContextProperties::new().platform(platform_id);
    let context = core::create_context(Some(&context_props), &[device_id], None, None)?;

    let queue =
        core::create_command_queue(&context, device_id, Some(flags::QUEUE_PROFILING_ENABLE))?;

    let mut image = vec![0 as u8; _image_size as usize];

    let image_buffer = unsafe {
        core::create_buffer(
            &context,
            flags::MEM_WRITE_ONLY | flags::MEM_COPY_HOST_PTR,
            _image_size as usize,
            Some(&image),
        )?
    };

    let program = core::create_program_with_source(&context, &[mdbrot::csource()])?;
    core::build_program(
        &program,
        Some(&[device_id]),
        &std::ffi::CString::new("")?,
        None,
        None,
    )?;

    match core::program_build_err(&program, &[device_id]) {
        Err(e) => eprintln!("Error: Program build {}", e),
        _ => {}
    }

    let kernel = core::create_kernel(&program, "mandelbrot")?;
    core::set_kernel_arg(&kernel, 0, core::ArgVal::mem(&image_buffer))?;
    core::set_kernel_arg(&kernel, 1, core::ArgVal::scalar(&w))?;
    core::set_kernel_arg(&kernel, 2, core::ArgVal::scalar(&h))?;

    unsafe {
        core::enqueue_kernel(
            &queue,
            &kernel,
            1,
            None,
            &[_global_work_size as usize, 1, 1],
            None,
            None::<core::Event>,
            None::<&mut core::Event>,
        )?;
    }

    unsafe {
        core::enqueue_read_buffer(
            &queue,
            &image_buffer,
            true,
            0,
            &mut image,
            None::<core::Event>,
            None::<&mut core::Event>,
        )?;
    }

    image::save_buffer(path, &image, w, h, image::ColorType::Rgba8).unwrap();

    Ok(())
}
