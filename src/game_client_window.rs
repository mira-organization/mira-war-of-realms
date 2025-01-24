use bevy::image::ImageSamplerDescriptor;
use bevy::prelude::*;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::window::WindowResolution;
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
    ))
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
        backends: Some(Backends::VULKAN),
        ..default()
    }
}