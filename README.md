# 整体设计

### 一、需要使用的 crate

#### 1、server 端

* askama，可以在文件中使用变量，用来编写 prompt
* openai，调用 openai 的接口
* db_schema，查看数据库 schema，最为 prompt 中的上下文提示
* sqlx，查询数据库
* actix-web，创建 web 服务



#### 2、client 端

* sycamore，web 应用
* tauri，桌面应用
* web-sys，调用 webapi 接口



# 详细设计 server

### 一、模块设计

分为 chat 模块 和 db 模块。

* chat 用来处理和 gpt 交互相关。
* db 用来处理和数据库交互相关。



### 二、设计 exec_chat 功能

1. 准备 prompt。使用 db_schema crate 获取数据库的结构信信息，使用 askama crate 将数据库信息填充到 prompt.txt 的中作为上下文。
2. 调用 openai 接口。使用 openai crate 调用 openai 接口。
3. 解析数据。使用 serde_json crate 进行数据格式化。



### 三、设计 query_schema 功能

* 使用 db_schema crate 可以获取数据库结构信息。



### 四、设计 query_tables 功能

1. 使用 db_schema crate 可以获取数据库中的所有 tables 信息。
2. 使用 regex 正则表达式对 tables 信息进行过滤，得到 tables_name。



### 五、设计 exec_sql 功能

1. 使用 sqlx 执行 sql 语句。
2. 解析数据。



【问题】怎么解析数据呢？因为 sqlx 中执行结果的返回需要一个明确的数据结构进行接收？





# 详细设计 client

### 一、组件设计

创建 app、chatinput、chatoutput、connection 组件



#### 1、app 组件设计

1. 初始化。将 openai_key、db_url、db_ns 等作为 state 存储到上下文中。
2. 加载 connection、chatinput、chatoutput 组件



#### 2、connection 组件设计

1. 获取 html 中的连接按钮。
2. 获取 html 中的 openai_key、db_url、db_ns 三个输入框的内容。
3. 监听按钮的点击事件，将三个输入框的内容保存到上下文中。



【准备】

* web-sys crate 中的 get_element_by_id，可以获取 html 中的内容。
* web-sys crate 中的 add_event_listener_with_callback，可以设置监听事件。



#### 3、chatinput 组件设计

1. 获取 html 中的聊天按钮。
2. 获取 html 中的 chat_input 输入框的内容。
3. 监听聊天按钮的点击事件，调用 server 的接口并且将响应内容保存上下文中。



#### 4、chatoutput 组件设计

1. 获取 html 中的文本框。
2. 监听上下文中的变量，设置文本框的内容为上下文中变量的值。



【准备】

* sycamore crate 中的 create_memo，可以监听上下文中变量的值。



### 二、模块设计

分为 chat 模块 和 db 模块。

* chat 用来处理和 server 端的关于 gpt 交互相关。
* db 用来处理和 server 端的关于数据库交互相关。



### 三、设计 exec_chat 功能

1. 初始化 http client。跨域配置、请求头配置、post 请求消息体配置等
2. 发送 http 请求。
3. 解析响应。



# 一些问题

【问题】怎么解析数据呢？因为 sqlx 中执行结果的返回需要一个明确的数据结构进行接收？





# TODO

#### 1、client 中使用不了 dotenv crate，导致 app 组件初始化的时候读取不到配置文件中数据。



#### 2、考虑是否使用 sycamore 组件动态显示内容，现在是静态 html 中的 dom 元素直接设置内容。



#### 3、chat 模块和 db 模块中，功能的流程都是一样的，出来参数不一样，考虑将 http 请求抽离出来。



#### 4、server 与 client 之间交互的数据结构没有统一，有的用 String、有的用 JsValue。后续统一用 actix crate 的封装数据结构，参数：post 请求用 web::Json，get 请求用 web::Query，响应都用 HttpResponse。

* 已完成





# 启动

* 进入 server 目录，cargo run --bin chat
* 进入 client 目录，trunk serve
