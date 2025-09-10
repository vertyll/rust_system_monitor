use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Config error: {0}")]
    Config(#[from] confy::ConfyError),

    #[error("Tray icon error: {0}")]
    Tray(#[from] tray_icon::Error),

    #[error("Tray icon menu error: {0}")]
    Menu(#[from] tray_icon::menu::Error),

    #[error("Icon creation error: {0}")]
    BadIcon(#[from] tray_icon::BadIcon),
}

pub type Result<T> = std::result::Result<T, AppError>;
