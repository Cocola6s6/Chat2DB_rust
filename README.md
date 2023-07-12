# crate

### server

* askama，可以在文件中使用变量
* async-openai，
* db_schema，查看数据库 schema 
* sqlx，查询数据库



### client

* sycamore，web 应用
* tauri，桌面应用
* web-sys，调用接口



# Chat2DB 流程

1. server 只负责请求 chatgpt
2. client 负责 prompt，并将 chatgpt response 解析、处理，如展示、执行等
3. cs 之间使用 SSE 进行通信。



# Chat2DB_rust 流程

