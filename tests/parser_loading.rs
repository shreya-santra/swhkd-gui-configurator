
mod data_model {
    include!("../src/data_model.rs");
}

use data_model::AppState;

#[test]
fn gui_can_load_all_sweet_sample_configs() {
    let files = [
        "tests/sweet_samples/basic_keybind.skwhrc",

    ];
    for path in files {
        let mut state = AppState::default();
        match state.load_from_swhkd_config_at(path) {
            Ok(_) => {
                println!("Loaded and parsed config: {}", path);
                assert!(
                    !state.modes.is_empty(),
                    "No modes loaded for {}",
                    path
                );
            }
            Err(e) => panic!("Failed to parse {}: {e:?}", path),
        }
    }
}
