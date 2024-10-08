用 dashMap 做了一个简单的 kv 存储，熟悉了端到端交互的数据流以及数据结构的相互转化。
测试：
1. 启动服务端
```
    cargo run --bin server
```
2. 启动客户端
```
    cargo run --bin client
```
可以更改 client.rs 中发送的请求，测试不同的功能。
目前只支持 get，put

后续：
1. 实现 delete, exist, set_all, get_all, delete_all等命令
2. 实现持久化存储
3. 命令行运行客户端时直接发送请求