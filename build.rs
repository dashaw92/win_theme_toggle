use windres::Build;

fn main() {
    Build::new().compile("win_theme_toggle.rc").unwrap();
}
