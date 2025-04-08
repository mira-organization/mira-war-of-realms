use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_fluent::{BundleAsset, FluentPlugin};
use fluent_bundle::FluentArgs;

pub struct LanguagesPlugin;

impl Plugin for LanguagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FluentPlugin);
        app.insert_resource(LocalManager::new());
        app.add_systems(Startup, pre_load_localizations);
    }
}

/// `LocalManager` is a struct that manages the current locale and the associated
/// localization assets, allowing easy switching between languages and retrieving
/// localized strings.
#[derive(Debug, Resource)]
pub struct LocalManager {
    /// The current locale identifier (e.g., "en-US").
    pub current_locale: String,

    /// A map from locale identifiers (e.g., "en-US") to handles of the associated
    /// localization assets.
    pub locales: HashMap<String, Handle<BundleAsset>>,
}

impl Default for LocalManager {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl LocalManager {
    /// Creates a new instance of `LocalManager` with a default locale (`"en-US"`)
    /// and an empty map for `locales`.
    ///
    /// # Returns
    /// A new `LocalManager` with a default locale set to `"en-US"`.
    ///
    /// # Example
    /// ```rust
    /// let manager = LocalManager::new();
    /// assert_eq!(manager.current_locale, "en-US");
    /// ```
    pub fn new() -> Self {
        Self {
            current_locale: "en-US".to_string(),
            locales: HashMap::new(),
        }
    }

    /// Sets the current locale to the provided locale string, if the locale exists
    /// in the `locales` map.
    ///
    /// # Arguments
    /// - `locale`: The locale identifier (e.g., "en-US", "de-DE").
    ///
    /// # Example
    /// ```rust
    /// let mut manager = LocalManager::new();
    /// manager.set_locale("de-DE");
    /// assert_eq!(manager.current_locale, "de-DE");
    /// ```
    pub fn set_locale(&mut self, locale: &str) {
        if self.locales.contains_key(locale) {
            self.current_locale = locale.to_string();
            info!("Change locale to: {}", locale);
        } else {
            info!("No localized locale: {}", locale);
        }
    }

    /// Retrieves a translation for a given key from the current locale.
    ///
    /// # Arguments
    /// - `assets`: The `Assets` resource containing the `BundleAsset`s for all locales.
    /// - `key`: The translation key to retrieve (e.g., `"debug"`).
    /// - `args`: Optional `FluentArgs` to format the message with dynamic arguments.
    ///
    /// # Returns
    /// `Some(String)` if the translation is found and successfully formatted,
    /// or `None` if the translation key is not found or there was an error.
    ///
    /// # Example
    /// ```rust
    /// let text = local_manager.get_translation(&assets, "debug", Some(&args));
    /// match text {
    ///     Some(translated_text) => println!("Translated: {}", translated_text),
    ///     None => println!("Translation not found!"),
    /// }
    /// ```
    pub fn get_translation(&self, assets: &Assets<BundleAsset>,
                           key: &str, args: Option<&FluentArgs>) -> Option<String> {
        if let Some(handle) = self.locales.get(&self.current_locale) {
            if let Some(bundle) = assets.get(handle) {
                let message = bundle.get_message(&key)
                    .expect(format!("Failed to get message from locale: {}", key).as_str());

                let pattern = message.value()
                    .expect("Failed to get message value");

                let mut errors = vec![];
                let result = bundle.format_pattern(&pattern, args, &mut errors);
                if errors.is_empty() {
                    return Some(result.to_string())
                }
            }
        }
        None
    }
}

/// This system pre-loads the localization assets for the supported locales at the
/// beginning of the application.
///
/// # Arguments
/// - `asset_server`: The `AssetServer` used to load localization assets.
/// - `locale_manager`: A mutable reference to the `LocalManager` that will hold the loaded assets.
pub fn pre_load_localizations(asset_server: Res<AssetServer>,
                          mut locale_manager: ResMut<LocalManager>) {
    info!("Loading localizations [ {} ]", locale_manager.current_locale);
    let locales = ["en-US", "de-DE"];
    let mut loaded_locales = HashMap::new();

    // Load the assets for each locale and store their handles.
    for locale in locales {
        let path = format!("locales/{}/main.ftl.yml", locale);
        let handle = asset_server.load::<BundleAsset>(&path);
        loaded_locales.insert(locale.to_string(), handle);
    }

    locale_manager.locales = loaded_locales;
}