use super::PveBuilder;

impl PveBuilder {
    pub fn weak_tls(mut self, value: bool) -> Self {
        self.weak_tls = Some(value);

        self
    }
}
