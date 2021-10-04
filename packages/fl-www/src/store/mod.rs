use crate::prelude::*;

mod i18n;
mod metadata;
mod theme;

use styling::ThemeKind;

#[derive(Clone, Debug)]
pub(crate) enum Action {
    SetThemeKind(Option<ThemeKind>),
    ThemeUpdated,

    RouteUpdated,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct State {
    pub theme: theme::ThemeState,
    pub i18n: i18n::I18nState,
    pub metadata: metadata::MetadataState,
}

impl Reducer for State {
    type Action = Action;

    fn new() -> Self {
        Self::default()
    }

    fn reduce(&mut self, action: Self::Action) -> ShouldNotify {
        log::debug!("{:?}", action);

        match action {
            Action::SetThemeKind(kind) => self.theme.set_theme_kind(kind),
            Action::RouteUpdated => {
                self.i18n.sync();
                true
            }

            Action::ThemeUpdated => {
                self.theme.sync();
                true
            }
        }
    }
}

pub(crate) type Store = ReducerStore<State>;
pub(crate) type AppDispatch = DispatchProps<Store>;

#[macro_export]
macro_rules! impl_dispatch_mut {
    ($props_name:ident) => {
        impl DispatchPropsMut for $props_name {
            type Store = crate::store::Store;

            fn dispatch(&mut self) -> &mut crate::store::AppDispatch {
                &mut self.dispatch
            }
        }
    };
}
