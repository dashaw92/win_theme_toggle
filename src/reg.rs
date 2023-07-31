use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_SET_VALUE},
    RegKey,
};

use crate::WttResult;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Theme {
    Dark,
    Light,
}

impl Theme {
    fn to_value(&self) -> u32 {
        match self {
            Theme::Dark => 0u32,
            Theme::Light => 1u32,
        }
    }
}

//Set-ItemProperty -Path HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize -Name AppsUseLightTheme -Value {} -Type Dword -Force
pub(crate) fn set_theme(theme: Theme) -> WttResult<&'static str> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = hkcu.open_subkey_with_flags(
        r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize"#,
        KEY_SET_VALUE,
    )?;

    path.set_value(
        "AppsUseLightTheme",
        &theme.to_value(),
    )?;

    path.set_value(
        "SystemUsesLightTheme",
        &theme.to_value(),
    )?;
    Ok("Registry has been updated.")
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
