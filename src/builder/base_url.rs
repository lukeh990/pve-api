use super::PveBuilder;

impl PveBuilder {
    pub fn base_url<S: Into<String>>(mut self, value: S) -> Self {
        let value = value.into();
        self.base_url = Some(value);

        self
    }
}
