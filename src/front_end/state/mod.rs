use druid::{Data, Lens};

pub(crate) mod logic;

#[derive(Clone, Data, Lens)]
pub(crate) struct AppState {
    encrypting: bool,
    shift_size_automatic: bool,
    shift_size: f64,
    input: String,
    output: String,
}

pub(crate) fn get_initial_state() -> AppState {
    AppState {
        encrypting: false,
        shift_size_automatic: false,
        shift_size: 10.0,
        input: "".to_string(),
        output: "".to_string(),
    }
}

impl AppState {
    pub(crate) fn get_encrypting(&self) -> bool {
        self.encrypting
    }

    pub(crate) fn set_encrypting(&mut self, val: bool) {
        self.encrypting = val;
    }

    pub(crate) fn get_shift_size_automatic(&self) -> bool {
        self.shift_size_automatic
    }

    pub(crate) fn set_shift_size_automatic(&mut self, val: bool) {
        self.shift_size_automatic = val;
    }

    pub(crate) fn get_shift_size(&self) -> f64 {
        self.shift_size
    }

    pub(crate) fn get_output(&self) -> &str {
        &*self.output
    }

    pub(crate) fn update_output(&mut self) {
        self.output = logic::find_output(self.encrypting, self.shift_size_automatic, self.shift_size, (&*self.input).parse().unwrap());
    }
}

