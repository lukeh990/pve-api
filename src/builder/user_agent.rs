use super::PveBuilder;

impl PveBuilder {
    pub fn user_agent<S: Into<String>>(mut self, value: S) -> Self {
        let value = value.into();

        self.user_agent = Some(value);

        self
    }
}
