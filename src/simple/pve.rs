use super::RequestBuilder;
use super::request::RequestMethod;

/// This feels self-explanatory. Call the correct function for whatever
/// HTTP method you want.
///
/// **REMEMBER:** The path specified must **NOT** contain a leading slash.
impl super::SimpleApi for crate::Pve {
    fn simple_get<S: Into<String>>(&self, path: S) -> RequestBuilder {
        RequestBuilder::new(self, path.into(), RequestMethod::Get)
    }

    fn simple_post<S: Into<String>>(&self, path: S) -> RequestBuilder {
        RequestBuilder::new(self, path.into(), RequestMethod::Post)
    }

    fn simple_put<S: Into<String>>(&self, path: S) -> RequestBuilder {
        RequestBuilder::new(self, path.into(), RequestMethod::Put)
    }

    fn simple_delete<S: Into<String>>(&self, path: S) -> RequestBuilder {
        RequestBuilder::new(self, path.into(), RequestMethod::Delete)
    }
}
