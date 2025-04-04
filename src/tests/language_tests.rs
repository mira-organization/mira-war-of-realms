#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::asset::AssetPlugin;
    use bevy::ecs::system::RunSystemOnce;
    use bevy_fluent::BundleAsset;
    use crate::languages::{pre_load_localizations, LocalManager};

    #[test]
    fn test_locale_manager_creation() {
        let manager = LocalManager::new();
        assert_eq!(manager.current_locale, "en-US");
        assert!(manager.locales.is_empty());
    }

    #[test]
    fn test_set_locale_existing() {
        let mut manager = LocalManager::new();
        manager.locales.insert("de-DE".to_string(), Handle::default());
        manager.set_locale("de-DE");
        assert_eq!(manager.current_locale, "de-DE");
    }

    #[test]
    fn test_set_locale_missing() {
        let mut manager = LocalManager::new();
        manager.set_locale("fr-FR");
        assert_eq!(manager.current_locale, "en-US"); // should not change
    }

    #[test]
    fn test_pre_load_localizations() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .init_resource::<LocalManager>()
            .init_asset::<BundleAsset>();

        app.update(); // init world etc.

        app.world_mut()
            .run_system_once(pre_load_localizations)
            .expect("Can't create one shot system!");

        let locale_manager = app.world().resource::<LocalManager>();
        for locale in ["en-US", "de-DE"] {
            assert!(locale_manager.locales.contains_key(locale));
        }
    }

}