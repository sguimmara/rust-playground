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
            // We generate the root state struct, with a field for each slice.
            // For example, to generate a state MyState with the FooSlice and
            // BarSlice slices, we have the following result:
            //
            // struct MyState {
            //   foo_slice: FooSlice,
            //   bar_slice: BarSlice
            // }
            #[derive(Debug, PartialEq, Default, Clone)]
            $vis struct $struct_name {
                $(
                    [< $slice:snake >]: $slice,
                )+
            }

            // We generate one "routing" action for each slice,
            // plus the Reset, Id and Restore utility actions.
            $vis enum [<$actions_name>] {
                /// Returns the state without modification.
                Id,
                /// Resets the state to its default value.
                Reset,
                /// Resets the state to the specified value.
                Restore($struct_name),
                $(
                    // One routing action for each slice that serves
                    // as a wrapper for this slice's specific actions.
                    [<$slice>](<$slice as HasActionType>::ActionType),
                )+
            }

            impl Slice<$struct_name, [<$actions_name>]> for $struct_name {
                fn mutate(self, action: [<$actions_name>]) -> $struct_name {
                    match action {
                        [<$actions_name>]::Id => self,
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

            // We also implement Into for each slice so that we
            // can automatically wrap a slice action into the root
            // action type, useful to reduce boilerplate
            $(
                impl Into<$actions_name> for <$slice as HasActionType>::ActionType {
                    fn into(self) -> $actions_name {
                        <$actions_name>::[<$slice>](self)
                    }
                }
            )+
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

        store.dispatch(Actions::Id);

        assert_eq!(store.get_state().preferences.theme, Theme::Light);
        assert_eq!(store.get_state().settings.enable_voiceover, false);

        store.dispatch(PreferenceActions::SetTheme(Theme::Dark).into());

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
