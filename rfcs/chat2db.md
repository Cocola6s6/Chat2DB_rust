

# Chat2DB_rust

- Feature Name: Chat2DB_rust
- Start Date: 2023-07-15

## Summary

参考 [Chat2DB](https://github.com/chat2db/Chat2DB) 的效果，使用 chatgpt 进行自然语言翻译，然后对数据库进行操作，使用 rust 语言实现的 web 应用、桌面应用。



## Motivation

* 整合已经学习的 rust crate，sycamore、sqlx、openai、wasm、actix-web、tauri 等。
* [Chat2DB](https://github.com/chat2db/Chat2DB) 是 java 语言实现的，基于此实现一个 rust 版本。



## Guide-level explanation

### 一、需要使用的 crate

#### 1、server 端

* askama，可以在文件中使用变量，用来编写 prompt
* openai，调用 openai 的接口
* db_schema，查看数据库的 schema，作为 prompt 中的上下文提示
* sqlx，查询数据库数据
* actix-web，创建 web 服务



#### 2、client 端

* sycamore，web 应用
* tauri，桌面应用
* web-sys，调用 webapi 接口



### 二、要实现的功能

* exec_chat。使用 chatgpt 对自然语言进行翻译。
* query_tables。查询所有的数据库表。
* exec_sql。执行翻译得到的 sql，并且将结果展示出来。



~~~bash
# exec_chat
service chat {
    rpc exec_chat(chat_req) returns (chat_resp) {}
}
chat_req {
    openai_key: String,
    db: db_req,
    text: String,
}

# query_tables
# exec_sql
service db {
    rpc query_tables(tables_req) returns (tables_resp) {}
    rpc exec_sql(sql_req) returns (sql_resp) {}
}

tables_req {
    db_url: String,
    db_ns: String,
}

sql_req {
    db_url: String,
    db_ns: String,
}

~~~



## Reference-level explanation of Server

### 一、模块设计

分为 chat 模块 和 db 模块。

* chat 用来处理和 gpt 交互相关。
* db 用来处理和数据库交互相关。



### 二、设计 exec_chat 功能

1. 准备 prompt。使用 db_schema crate 获取数据库的结构信信息，使用 askama crate 将数据库信息填充到 prompt.txt 的中作为上下文。
2. 调用 openai 接口。使用 openai crate 调用 openai 接口。
3. 解析数据。使用 serde_json crate 进行数据格式化。



【准备】

* serde_json crate，可以将 json 转换为 object。



### 三、设计 query_schema 功能

* 使用 db_schema crate，可以获取数据库结构信息。



### 四、设计 query_tables 功能

1. 使用 db_schema crate 可以获取数据库中的所有 tables 信息。
2. 使用 regex 正则表达式对 tables 信息进行过滤，得到 tables_name。



### 五、设计 exec_sql 功能

1. 使用 sqlx 执行 sql 语句。
2. 解析数据。



【问题】怎么解析数据呢？因为 sqlx 中执行结果的返回需要一个明确的数据结构进行接收？这是这个功能的重点。

以下是如何解析数据的流程：

1. 获得 PgRow 数组，里面是查询回来的所有行的信息。
2. 获得 PgRow 里面的 PgColumn 数组，里面是当前行的每一列的信息。通过它可以获得字段名、字段类型。
3. 遍历 PgColumn 数组，通过 PgRow[index] 可以获得当前行的index列的信息，也就是字段值。
4. 统一使用 serde_json::Value 接收字段值。
5. 使用 map 保存字段名和字段值。



【统一使用 serde_json::Value 接收字段值】

~~~rust
// 获取字段值
// TODO 统一返回类型，使用serde_json::Value，否则会类型不匹配报错
let value: serde_json::Value = match type_info.name() {
  "INT4" => serde_json::Value::Number(row.get::<i32, _>(i).into()),
  "VARCHAR" => serde_json::Value::String(row.get::<String, _>(i).into()),
  _ => serde_json::Value::Null,
};
~~~





## Reference-level explanation of Client

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
* sycamore crate 中的 create_signal，可以声明 signal 用于 view 与 view 直接的通信。



### 一、组件设计（优化：使用 sycamore 方式，局部渲染）

前面使用 html 的方式实现了，现在改用 sycamore 的方式实现。

需要注意的是，sycamore 组件中的元素的渲染是【局部渲染】，除了设置了 signal 监听的才会重新渲染，其它是不会变化的。



#### 1、input 组件的值的绑定
* 使用 bind:value=text_signal 将 input 组件的值和 text_signal
绑定，当 input 组件的值发生变化的时候，text_signal 的值也会发生变化。
* 就不用疯狂使用 web-sys 去获取 html 中的 dom 元素了。



#### 2、button 组件的点击事件
* 使用 on:click=callback 将 button 组件的点击事件和 callback 绑定，当 button 组件被点击的时候，callback 会被调用。
* 就不用疯狂使用 web-sys 去设置 button 的监听事件了。



#### 3、chatoutput 组件设计

1. 接收 input 控件传过来的值，包括了：询问AI的到的 sql、执行sql得到的数据库数据列表、数据库的结构信息。
2. 使用 create_memo 创建监听，对所有传过来的值进行监听。
3. 将监听绑定到 sycamore 的 view 中。



【问题】怎么使用 sycamore 来渲染列表数据？这是重点。

渲染列表的流程如下：

1. 解析得到表头数据，设置监听，渲染表头。
2. 使用 sycamore Indexed 进行两层循环遍历数据，渲染列表数据。外层遍历所有数据得到每一行数据。内层遍历表头数据的到当前行的某一列。



【注意】需要注意的是，使用 Indexed 时，内层 view 是不能直接使用外层 view 的变量的。需要通过 create_signal 声明信号值传递。

~~~rust
// 使用 Indexed 嵌套迭代渲染数据
Indexed(
    iterable=db_output_text,
    view=move|ctx, entry| {
        // TODO 注意：1）view中只能是dom元素，写方法会报错。2）内层view不能直接使用外层的变量。
        // 需要通过信号量来通信
        let entry_signal = create_signal(ctx, entry.clone());
        view! { ctx,
            tr(class="bg-white border-b dark:bg-gray-800 dark:border-gray-700") {
                // 使用 Indexed 迭代渲染单元格数据
                Indexed(
                    iterable=db_output_keys_text,
                    view=move|ctx, key| {
                        view! { ctx,
                            td(class="px-6 py-4") {
                                (entry_signal.get().get(&key).unwrap())
                            }
                        }
                    }
                )
            }
        }
    }
)
~~~





### 二、模块设计

分为 chat 模块 和 db 模块。

* chat 用来处理和 server 端的关于 gpt 交互相关。
* db 用来处理和 server 端的关于数据库交互相关。



### 三、设计 exec_chat、 query_tables、exec_sql功能

1. 初始化 http client。跨域配置、请求头配置、post 请求消息体配置等
2. 发送 http 请求。
3. 解析响应。使用 serde_json crate 进行数据格式化。



## Drawbacks

* [Chat2DB](https://github.com/chat2db/Chat2DB)  具有很多效果，比如支持多数据源、更好的 UI 交互等等，现在 Chat2DB_rust 只是简单地支持Postgres，而且代码的兼容性也不好。



## Rationale and alternatives

N/A

## Prior art

N/A

## Unresolved questions

#### 【问题1】怎么解析数据呢？因为 sqlx 中执行结果的返回需要一个明确的数据结构进行接收？

* 已解决，查看【设计 exec_sql 功能】中的解答。



#### 【问题2】信号量监听问题

观察以下两段代码，同样是设置监听信号量，为什么第二段代码会存在问题，列表没有渲染出来？

* 第二段代码列表渲染是监听的是 db_output_keys_text 的变化而变化。按理来说，db_output_keys_text 是根据 db_output_text 计算得来的，db_output_text 变化了 db_output_keys_text 也会变化才对。



第一段如下：

~~~rust
let db_output_keys_text = create_memo(ctx, move || props
        .db_output_text
        .get()
        .get(0)
        .iter()
        .flat_map(|dict| dict.keys().cloned())
        .collect());
~~~

第二段如下：

~~~rust
let db_output_keys_text = props.db_output_text
    .get()
    .get(0)
    .iter()
    .flat_map(|dict| dict.keys().cloned())
    .collect();
    
let db_output_keys_text = create_memo(ctx, move || db_output_keys_text.to_vec());
~~~





## Future possibilities



#### 1、client 中使用不了 dotenv crate，导致 app 组件初始化的时候读取不到配置文件中数据。



#### 2、考虑是否使用 sycamore 组件动态显示内容，现在是静态 html 中的 dom 元素直接设置内容。
* 已完成，但是数组的换行展示还有问题。


#### 3、chat 模块和 db 模块中，功能的流程都是一样的，出来参数不一样，考虑将 http 请求抽离出来。
* 已完成

#### 4、server 与 client 之间交互的数据结构没有统一，有的用 String、有的用 JsValue。后续统一用 actix crate 的封装数据结构，参数：post 请求用 web::Json，get 请求用 web::Query，响应都用 HttpResponse。

* 已完成


#### 5、CLI 还没有实现，需要实现一个 CLI。1）可以直接启动 server 和 client； 2）可以直接执行 exec_chat、query_tables、exec_sql 功能。
* 已完成


#### 6、增加单元测试
* 已完成


#### 7、server 的 panic 没有处理，需要处理 panic。
* 已完成
