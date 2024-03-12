use crate::{request, types, api};

pub struct Routes(Vec<Route>);

struct Route {
    path: String,
    method: request::Method,
    api_function: types::ServerFunction,
    exact: bool,
}

struct RouteBuilder {
    path: Option<String>,
    method: Option<request::Method>,
    api_function: Option<types::ServerFunction>,
    exact: Option<bool>,
}

impl RouteBuilder {
    fn new() -> Self {
        Self {
            path: None,
            method: None,
            api_function: None,
            exact: None,
        }
    }

    fn path(self, path: &str) -> Self {
        Self {
            path: Some(path.to_string()),
            ..self
        }
    }

    fn method(self, method: request::Method) -> Self {
        Self {
            method: Some(method),
            ..self
        }
    }

    fn api_function(self, api_function: types::ServerFunction) -> Self {
        Self {
            api_function: Some(api_function),
            ..self
        }
    }

    fn exact(self, exact: bool) -> Self {
        Self {
            exact: Some(exact),
            ..self
        }
    }

    fn build(self) -> Route {
        Route {
            path: self.path.unwrap(),
            method: self.method.unwrap(),
            api_function: self.api_function.unwrap(),
            exact: self.exact.unwrap(),
        }
    }
}

impl Routes {
    pub fn get_api_function(
        &self,
        path: String,
        method: request::Method,
    ) -> Option<types::ServerFunction> {
        self.0
            .iter()
            .filter(|route| route.method == method)
            .find(|route| {
                if route.exact {
                    route.path == path
                } else {
                    path.starts_with(&route.path)
                }
            })
            .map(|route| route.api_function.clone())
    }

    pub fn init() -> Self {
        let routes = vec![
            RouteBuilder::new()
                .path("/")
                .method(request::Method::Get)
                .api_function(api::home)
                .exact(true)
                .build(),
            RouteBuilder::new()
                .path("/echo")
                .method(request::Method::Get)
                .api_function(api::echo)
                .exact(false)
                .build(),
            RouteBuilder::new()
                .path("/user-agent")
                .method(request::Method::Get)
                .api_function(api::user_agent)
                .exact(true)
                .build(),
            RouteBuilder::new()
                .path("/files")
                .method(request::Method::Get)
                .api_function(api::file_get)
                .exact(false)
                .build(),
            RouteBuilder::new()
                .path("/files")
                .method(request::Method::Post)
                .api_function(api::file_post)
                .exact(false)
                .build(),
        ];
        Self(routes)
    }
}
