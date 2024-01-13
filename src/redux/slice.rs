pub trait HasActionType {
    type ActionType;
}

pub trait Slice<State, Action> {
    fn mutate(self, action: Action) -> State;
}

/// Creates the State struct, the action enum and the root mutator function
/// For the specified aggregated state (aggregated from the list of slice types)
#[macro_export]
macro_rules! build_state {
    ($vis:vis $struct_name:ident, $actions_name:ident, $($slice:ty),+) => {
        paste::paste! {
            #[derive(Debug, PartialEq, Default, Clone)]
            $vis struct $struct_name {
                $(
                    [< $slice:snake >]: $slice,
                )+
            }

            $vis enum [<$actions_name>] {
                Reset,
                Restore($struct_name),
                $(
                    [<$slice>](<$slice as HasActionType>::ActionType),
                )+
            }

            impl Slice<$struct_name, [<$actions_name>]> for $struct_name {
                fn mutate(self, action: [<$actions_name>]) -> $struct_name {
                    match action {
                        [<$actions_name>]::Reset => Self::default(),
                        [<$actions_name>]::Restore(s) => s,
                        $(
                            [<$actions_name>]::[<$slice>](arg) => {
                                return $struct_name {
                                    [<$slice:snake>]: self.[<$slice:snake>].mutate(arg),
                                    ..self
                                }
                            },
                        )+
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    use crate::redux::store::Store;

    use super::{Slice, HasActionType};

    #[derive(Debug, PartialEq, Default, Clone)]
    struct Settings {
        enable_voiceover: bool,
    }

    impl Slice<Settings, SettingActions> for Settings {
        fn mutate(self, action: SettingActions) -> Self {
            match action {
                SettingActions::EnableVoiceOver(v) => Self {
                    enable_voiceover: v,
                    ..self
                }
            }
        }
    }

    impl HasActionType for Settings {
        type ActionType = SettingActions;
    }

    #[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
    enum Theme { Dark, Light }

    impl Default for Theme {
        fn default() -> Self {
            Theme::Light
        }
    }

    #[derive(Debug, PartialEq, Default, Clone)]
    struct Preferences {
        theme: Theme
    }

    impl Slice<Preferences, PreferenceActions> for Preferences {
        fn mutate(self, action: PreferenceActions) -> Self {
            match action {
                PreferenceActions::SetTheme(theme) => Self {
                    theme,
                    ..self
                }
            }
        }
    }

    impl HasActionType for Preferences {
        type ActionType = PreferenceActions;
    }

    build_state!(State, Actions, Preferences, Settings);

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    enum SettingActions {
        EnableVoiceOver(bool),
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    enum PreferenceActions {
        SetTheme(Theme),
    }

    fn set_theme(theme: Theme) -> Actions {
        Actions::Preferences(PreferenceActions::SetTheme(theme))
    }

    #[test]
    fn with_default() {
        let mut store = Store::new(State::default(), &State::mutate);

        assert_eq!(store.get_state().preferences.theme, Theme::Light);
        assert_eq!(store.get_state().settings.enable_voiceover, false);

        store.dispatch(Actions::Settings(SettingActions::EnableVoiceOver(true)));

        assert_eq!(store.get_state().preferences.theme, Theme::Light);
        assert_eq!(store.get_state().settings.enable_voiceover, true);

        store.dispatch(Actions::Reset);

        assert_eq!(store.get_state().preferences.theme, Theme::Light);
        assert_eq!(store.get_state().settings.enable_voiceover, false);

        store.dispatch(set_theme(Theme::Dark));

        assert_eq!(store.get_state().preferences.theme, Theme::Dark);
        assert_eq!(store.get_state().settings.enable_voiceover, false);

        let saved = store.get_state().clone();

        store.dispatch(set_theme(Theme::Light));

        assert_eq!(store.get_state().preferences.theme, Theme::Light);
        assert_eq!(store.get_state().settings.enable_voiceover, false);

        store.dispatch(Actions::Restore(saved));

        assert_eq!(store.get_state().preferences.theme, Theme::Dark);
        assert_eq!(store.get_state().settings.enable_voiceover, false);

        let mut listened = false;

        let mut listener = || { listened = true };

        store.subscribe(&mut listener);

        store.dispatch(Actions::Reset);

        assert_eq!(&listened, &true);
    }
}
