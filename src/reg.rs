use std::error::Error;

use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_SET_VALUE},
    RegKey,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Theme {
    Dark,
    Light,
}

//Set-ItemProperty -Path HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize -Name AppsUseLightTheme -Value {} -Type Dword -Force
pub(crate) fn set_theme(theme: Theme) -> Result<(), Box<dyn Error>> {
    println!("Attempting to set theme to {:?}", &theme);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = hkcu.open_subkey_with_flags(
        r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize"#,
        KEY_SET_VALUE,
    )?;

    path.set_value(
        "AppsUseLightTheme",
        &match theme {
            Theme::Dark => 0u32,
            Theme::Light => 1u32,
        },
    )?;
    Ok(())
}

impl<S: AsRef<str>> From<S> for Theme {
    fn from(st: S) -> Self {
        match st.as_ref() {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            _ => Theme::Dark,
        }
    }
}
