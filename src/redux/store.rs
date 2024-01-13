use std::marker::PhantomData;

type Mutator<State, Action> = dyn Fn(State, Action) -> State;

pub struct Store<'a, State, Action>
where
    State: Clone,
{
    state: State,
    mutate: &'a Mutator<State, Action>,
    listeners: Vec<&'a mut dyn FnMut()>,
    phantom: PhantomData<Action>,
}

impl<'a, State: Clone, Action> Store<'a, State, Action> {
    /// Creates a new [`Store<State, Action>`].
    pub fn new(initial_state: State, mutate: &'a Mutator<State, Action>) -> Self {
        Store {
            state: initial_state,
            mutate,
            listeners: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn get_state(&self) -> &State {
        return &self.state;
    }

    pub fn dispatch(&mut self, action: Action) {
        self.state = (self.mutate)(self.state.clone(), action);

        self.raise_events();
    }

    pub fn subscribe(&mut self, listener: &'a mut impl FnMut()) {
        self.listeners.push(listener);
    }

    fn raise_events(&mut self) {
        for listener in &mut self.listeners {
            listener();
        }
    }
}
