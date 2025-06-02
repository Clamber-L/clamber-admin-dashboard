pub enum SettingType {
    LoginImage(i32),
    AppImage(i32),
}

impl SettingType {
    pub const LOGIN_IMAGE: SettingType = SettingType::LoginImage(1);
    pub const APP_IMAGE: SettingType = SettingType::AppImage(2);

    pub fn value(&self) -> i32 {
        match self {
            SettingType::LoginImage(v) => *v,
            SettingType::AppImage(v) => *v,
        }
    }
}
