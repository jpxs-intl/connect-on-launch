use crate::BASE_ADDRESS;
use std::{env, process};

/// Iterates over the maps of the current process to find the start and end of
/// the base executable's virtual address range
pub fn get_process_base() -> Option<(usize, usize)> {
    let Ok(process_path) = env::current_exe() else {
        return None;
    };
    let process_name = process_path.file_name()?;

    #[cfg(target_os = "windows")]
    let process_id = process::id();

    #[cfg(target_os = "linux")]
    let process_id = process::id() as i32;

    let Ok(maps) = proc_maps::get_process_maps(process_id) else {
        return None;
    };

    let range_iter = maps
        .iter()
        .filter_map(|v| match v.filename()?.file_name()? == process_name {
            true => Some((v.start(), v.start() + v.size())),
            false => None,
        });
    let (start_vec, end_vec): (Vec<_>, Vec<_>) = range_iter.unzip();
    let (start, end) = (start_vec.into_iter().min(), end_vec.into_iter().min());

    match (start, end) {
        (Some(start), Some(end)) => Some((start, end)),
        _ => None,
    }
}

#[inline]
pub fn address_from_base(offset: usize) -> *const () {
    (BASE_ADDRESS.get().unwrap() + offset) as *const ()
}
