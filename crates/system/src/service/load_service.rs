use bevy::{
    prelude::*,
    render::{
        render_resource::{CachedPipelineState, PipelineCache},
        MainWorld, RenderApp,
    },
};

/// The `LoadService` is responsible for managing the loading process of rendering pipelines.
/// It tracks the number of ready pipelines and updates the game state accordingly.
pub struct LoadService;

/// A `Resource` that stores the number of pipelines that are ready.
///
/// This is used to keep track of completed rendering pipelines in the main world.
#[derive(Resource, Default, Debug)]
pub struct PipelinesReady(usize);

impl PipelinesReady {
    /// Returns the number of pipelines that are ready.
    ///
    /// # Returns
    /// - `usize`: The number of successfully loaded pipelines.
    pub fn get(&self) -> usize {
        self.0
    }
}

impl Plugin for LoadService {
    /// Registers the `PipelinesReady` resource and sets up the pipeline tracking system.
    ///
    /// - Initializes `PipelinesReady` as a tracked resource.
    /// - Adds the `update_pipelines_ready` system to the `RenderApp`, which monitors pipeline status.
    ///
    /// # Parameters
    /// - `app`: The Bevy application instance where the systems are added.
    fn build(&self, app: &mut App) {
        app.init_resource::<PipelinesReady>();

        app.sub_app_mut(RenderApp)
            .add_systems(ExtractSchedule, update_pipelines_ready);
    }
}

/// Updates the number of ready pipelines in the `PipelinesReady` resource.
///
/// This function iterates through the pipeline cache and counts the number of pipelines
/// that have successfully reached the `Ok` state.
///
/// # Parameters
/// - `main_world`: Mutable reference to the main Bevy world, where `PipelinesReady` is stored.
/// - `cache`: Reference to the `PipelineCache`, which contains information about all pipelines.
fn update_pipelines_ready(mut main_world: ResMut<MainWorld>, cache: Res<PipelineCache>) {
    if let Some(mut pipelines_ready) = main_world.get_resource_mut::<PipelinesReady>() {
        let count = cache
            .pipelines()
            .filter(|pipeline| matches!(pipeline.state, CachedPipelineState::Ok(_)))
            .count();

        if pipelines_ready.0 == count {
            return;
        }

        pipelines_ready.0 = count;
    }
}
