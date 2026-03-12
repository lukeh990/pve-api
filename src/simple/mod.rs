mod pve;
mod request;

pub use request::RequestBuilder;

pub trait SimpleApi {
    fn simple_get<S: Into<String>>(&self, path: S) -> RequestBuilder;

    fn simple_post<S: Into<String>>(&self, path: S) -> RequestBuilder;

    fn simple_put<S: Into<String>>(&self, path: S) -> RequestBuilder;

    fn simple_delete<S: Into<String>>(&self, path: S) -> RequestBuilder;
}
