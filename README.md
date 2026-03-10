# 介绍
一个极简风格的桌面端App框架。

# 后端
1，基于tiny_http服务器。可以写自己的路由。

2，自动占用一个空闲端口。

3，自动打开浏览器。
# 用法举例
依赖。
```toml
[dependencies]
tokio = { version = "1", features = ["full"] } #异步
tiny_http = "0.12"                             #Web服务器
zxing_app="0.3.0"
```
示例代码。
```rust
//创建路由对象
let mut router = zxing_app::Router::new();
//添加路由处理
router.route(tiny_http::Method::Get, "/", |_, _| {
    tiny_http::Response::from_string("你好，世界！")
});
//启动服务器，打开浏览器
zxing_app::start(router);
```