use super::errors::Error;
use std::fs;
use tauri::Window;

#[tauri::command]
pub fn read(path: String) -> Result<String, Error> {
    let data = fs::read(path)?;
    let string = String::from_utf8(data)?;
    Ok(string)
}

#[tauri::command]
pub fn write(path: String, contents: String) -> Result<(), Error> {
    fs::write(path, contents)?;
    Ok(())
}

#[tauri::command]
pub fn adjust_initial_window_size(height: f64, window: Window) -> Result<f64, Error> {
    let current_size = window
        .inner_size()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let new_height_scaling = (current_size.height as f64 - 250.0) / height;
    let new_height = height * new_height_scaling;
    eprintln!("new height: {}", new_height);
    window
        .set_size(tauri::Size::Physical(tauri::PhysicalSize {
            width: current_size.width,
            height: new_height as u32,
        }))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    // Return scaling factor
    Ok(new_height_scaling)
}

/// Resize the window to fit the content
///
/// # Arguments
///
/// * `height` - The height of the content
/// * `window` - The window to resize
///
/// # Returns
///
/// * `Ok(())` on success
/// * `Err(Error)` on failure
#[tauri::command]
pub fn resize_window(height: f64, scaling_factor: f64, window: Window) -> Result<(), Error> {
    // Add a small buffer to prevent scrollbar flickering (10px)
    let target_height = height;

    // Get the current size to preserve width
    let current_size = window
        .inner_size()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Set minimum height to 600px (the default from tauri.conf.json)
    // let new_height = target_height.max(600.0) as u32;
    eprintln!("scaling_factor: {}", scaling_factor);
    let new_height = target_height * scaling_factor;

    // Apply the new size
    window
        .set_size(tauri::Size::Physical(tauri::PhysicalSize {
            width: current_size.width,
            height: new_height as u32,
        }))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(())
}
