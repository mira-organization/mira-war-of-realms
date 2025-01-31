use bevy::asset::{LoadState, LoadedFolder};
use bevy::prelude::*;
use bevy_fluent::{FluentPlugin, Locale, Localization, LocalizationBuilder};
use unic_langid::langid;

pub struct LanguagesPlugin;

impl Plugin for LanguagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FluentPlugin);
        app.insert_resource(Locale::new(langid!("en-US")));
        app.add_systems(Startup, pre_load_localizations);
    }
}

fn pre_load_localizations(asset_server: Res<AssetServer>,
                          localization_builder: LocalizationBuilder,
                          mut handle: Local<Option<Handle<LoadedFolder>>>,
                          mut localization: Local<Option<Localization>>
) {
    info!("Loading localization...");
    let handle = &*handle.get_or_insert_with(|| asset_server.load_folder("locales"));
    if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
        let localization = localization.get_or_insert_with(|| localization_builder.build(handle));
        info!("{:?}", localization);
    }
}