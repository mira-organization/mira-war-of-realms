use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use bevy::image::ImageSamplerDescriptor;
use bevy::log::tracing_subscriber::Layer;
use bevy::log::{tracing_subscriber, BoxedLayer, Level, LogPlugin};
use bevy::log::tracing_subscriber::fmt::writer::BoxMakeWriter;
use bevy::prelude::*;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::window::WindowResolution;
use chrono::Utc;
use system::LOG_ENV_FILTER;
use crate::manager::ManagerPlugin;

/// Initializes a new `App` with custom plugins and configurations.
///
/// # Parameters
/// - `app`: A mutable reference to the `App` instance to be configured.
/// - `title`: The title of the primary window.
/// - `width`: The width of the primary window in logical pixels.
/// - `height`: The height of the primary window in logical pixels.
///
/// # Returns
/// A mutable reference to the updated `App` instance.
///
/// # Plugins Configured
/// - `DefaultPlugins`: Initializes the default Bevy plugins.
/// - `WindowPlugin`: Configures the primary window with the specified `title`, `width`, and `height`.
/// - `RenderPlugin`: Sets up the render system with automatic GPU settings created by [`create_gpu_settings`].
/// - `ImagePlugin`: Configures the default image sampler to use nearest neighbor sampling.
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
///
/// fn main() {
///     let mut app = App::new();
///     create(&mut app, "My Game", 800.0, 600.0);
///     app.run();
/// }
/// ```
pub fn create<'a>(app: &'a mut App, title: &'a str, width: f32, height: f32) -> &'a mut App {
    app.add_plugins(DefaultPlugins.set(
        WindowPlugin {
            primary_window: Some(Window {
                title: title.to_string(),
                resolution: WindowResolution::new(width, height),
                ..default()
            }),
            ..default()
        }
    ).set(
        RenderPlugin {
            render_creation: RenderCreation::Automatic(create_gpu_settings()),
            ..default()
        }
    ).set(
        ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest()
        }
    ).set(LogPlugin {
        level: Level::DEBUG,
        filter: LOG_ENV_FILTER.to_string(),
        custom_layer: log_file_appender,
    }))
        .add_plugins(ManagerPlugin)
}

/// Creates GPU settings for rendering.
///
/// The settings include enabling Vulkan as the rendering backend and enabling
/// the `POLYGON_MODE_LINE` feature for wireframe rendering.
///
/// # Returns
/// A configured [`WgpuSettings`] instance.
///
/// # Example
/// ```rust
/// let gpu_settings = create_gpu_settings();
/// ```
pub fn create_gpu_settings() -> WgpuSettings {
    WgpuSettings {
        features: WgpuFeatures::POLYGON_MODE_LINE,
        backends: Some(Backends::PRIMARY),
        ..default()
    }
}

/// Initializes a log file appender for the application.
///
/// This function creates a `logs` directory if it does not exist and generates a log file
/// with a timestamped name in the format `bevy-DD-MM-YYYY.log`. It then sets up a
/// logging layer that writes log messages to this file.
///
/// # Parameters
/// - `_app`: A mutable reference to the Bevy `App`. (Currently unused)
///
/// # Returns
/// - `Some(BoxedLayer)`: If the log file was successfully created and opened.
/// - `None`: If there was an error creating the log directory or opening the file.
///
/// # Logging Details
/// - The log file is set up to append new logs.
/// - ANSI formatting is disabled for better readability in plain text files.
/// - The log writer ensures proper synchronization using `Arc<Mutex<File>>`.
fn log_file_appender(_app: &mut App) -> Option<BoxedLayer> {
    let log_dir = PathBuf::from("logs");
    std::fs::create_dir_all(&log_dir).ok()?; // Ensure the logs directory exists

    // Generate a timestamped log filename
    let timestamp = Utc::now().format("bevy-%d-%m-%Y.log").to_string();
    let log_path = log_dir.join(timestamp);

    // Open the log file for appending, creating it if necessary
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .ok()?;

    // Wrap the file in an Arc<Mutex<File>> for safe shared access
    let file_arc = Arc::new(Mutex::new(file));

    // Create a shutdown log marker to write an initial log entry when dropped
    let _shutdown_logger = StartLogText {
        file: Arc::clone(&file_arc),
    };

    // Create a log writer that clones the file handle for each log entry
    let writer = BoxMakeWriter::new(move || {
        let file = file_arc.lock().unwrap().try_clone().expect("Failed to clone log file handle");
        Box::new(file) as Box<dyn Write + Send>
    });

    // Return a tracing subscriber layer configured to write logs to the file
    Some(Box::new(tracing_subscriber::fmt::layer()
        .with_ansi(false) // Disable ANSI formatting
        .with_writer(writer)
        .boxed()
    ))
}

/// Helper struct to insert a start log entry when logging is initialized.
///
/// When this struct is dropped, it writes a separator message to the log file
/// to indicate when logging has started.
struct StartLogText {
    file: Arc<Mutex<File>>, // Shared reference to the log file
}

impl Drop for StartLogText {
    /// Writes a log entry to indicate the start of a new logging session.
    fn drop(&mut self) {
        let mut file = self.file.lock().unwrap();
        let _ = writeln!(
            file,
            "\n====================================== [ Start ] ======================================\n"
        );
        let _ = file.flush(); // Ensure the message is written to disk
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::window::Window;

    #[test]
    fn test_window_settings_values() {
        let title = "Test Window";
        let width = 800.0;
        let height = 600.0;

        let win = Window {
            title: title.to_string(),
            resolution: WindowResolution::new(width, height),
            ..default()
        };

        assert_eq!(win.title, title);
        assert_eq!(win.resolution.width(), width);
        assert_eq!(win.resolution.height(), height);
    }

    #[test]
    fn test_gpu_settings_values() {
        let settings = create_gpu_settings();
        assert_eq!(settings.features, WgpuFeatures::POLYGON_MODE_LINE);
        assert_eq!(settings.backends, Some(Backends::PRIMARY));
    }
}


