use std::io::Cursor;
use tiny_http::{Method, Response, Server, StatusCode};
//类型别名
type HandlerFn = dyn Fn(&Method, &str) -> Response<Cursor<Vec<u8>>> + Send + Sync;

//路由结构体
pub struct Router {
    routes: Vec<(Method, String, Box<HandlerFn>)>,
}

impl Router {
    pub fn new() -> Self {
        Router { routes: Vec::new() }
    }

    pub fn route<F>(&mut self, method: Method, path: &str, handler: F)
    where
        F: Fn(&Method, &str) -> Response<Cursor<Vec<u8>>> + Send + Sync + 'static,
    {
        self.routes
            .push((method, path.to_string(), Box::new(handler)));
    }
    //查找处理函数
    fn find_handler(&self, method: &Method, path: &str) -> Option<&Box<HandlerFn>> {
        for (m, p, handler) in &self.routes {
            if m == method {
                if p == path {
                    return Some(handler);
                }
            }
        }
        None
    }
}
pub fn start(router: Router) {
    //启动服务器
    let server = Server::http("127.0.0.1:0").unwrap();
    let url = format!("http://{}", server.server_addr());
    println!("服务器运行在 {}", url);
    //打开浏览器
    webbrowser::open(&url).unwrap();
    //循环处理
    for request in server.incoming_requests() {
        let method = request.method().clone();
        let full_url = request.url();
        let path = full_url.split('?').next().unwrap_or(full_url);
        let response = if let Some(handler) = router.find_handler(&method, path) {
            handler(&method, path)
        } else {
            Response::new(
                StatusCode(404),
                vec![],
                Cursor::new(format!("Not Found: {}", path).into_bytes()),
                None,
                None,
            )
        };
        request.respond(response).unwrap();
    }
}
