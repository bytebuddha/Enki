mod location;
mod protocol;

pub use self::location::XiLocation;

pub mod client;
pub use self::client::{Client, ClientExt};

mod line_cache;
pub use self::line_cache::LineCache;

mod styles;
pub use self::styles::StyleCache;

pub use xi_core_lib::styles::ThemeSettings;
pub use xi_core_lib::tabs::ViewId;

pub use self::protocol::{
    Alert, Annotation, AvailableLanguages, AvailablePlugins, AvailableThemes, ConfigChanged,
    ConfigChanges, FindStatus, LanguageChanged, Line, MeasureWidth, Message, ModifySelection,
    Operation, OperationType, Plugin, PluginStarted, PluginStoped, Position, Query, ReplaceStatus,
    Request, Response, ScrollTo, Status, Style, StyleDef, ThemeChanged, Update, UpdateCmds,
    UpdateNotification, XiNotification,
};
