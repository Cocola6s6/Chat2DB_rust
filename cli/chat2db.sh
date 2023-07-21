#!/bin/bash
set -e

# 获取脚本所在目录的绝对路径
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# 全部启动
function start() {
    start_server
    start_client
}

# 全部停止
function stop() {
    stop_server
    stop_client
}

# 启动 server
function start_server() {
    echo "Starting chat server..."
    cd $SCRIPT_DIR/../chat2db_server
    cargo run --bin server &
}

# 启动 client
function start_client() {
    echo "Starting chat client..."
    cd $SCRIPT_DIR/../chat2db_client
    trunk serve &
}

# 停止 server
function stop_server() {
    echo "Stopping chat server..."
    pid=$(ps aux | grep "cargo run --bin server" | grep -v grep | awk '{print $2}')
    if [ -n "$pid" ]; then
        kill "$pid"
    else
        echo "Chat server is not running."
    fi
}

# 停止 client
function stop_client() {
    echo "Stopping chat client..."
    pid=$(ps aux | grep "trunk serve" | grep -v grep | awk '{print $2}')
    if [ -n "$pid" ]; then
        kill "$pid"
    else
        echo "Chat client is not running."
    fi
}

# 执行 chat
function exec_chat() {
    echo "exec_chat..."

}

# 执行 sql
function exec_sql() {
    echo "exec_sql..."
    cd $SCRIPT_DIR/../chat2db_server
    cargo run --bin db
}

# 查询 tables
function query_tables() {
    echo "query_tables..."
    cd $SCRIPT_DIR/../chat2db_server
    cargo run --bin db
}

## 安装单个服务
function startSingle() {
    case $1 in
    server)
        start_server
        ;;
    client)
        start_client
        ;;
    *) ;;
    esac
}

## 安装单个服务
function stopSingle() {
    case $1 in
    server)
        stop_server
        ;;
    client)
        stop_client
        ;;
    *) ;;
    esac
}

# 执行命令
case $1 in
start)
    start
    ;;
stop)
    stop
    ;;
start_one)
    startSingle $2
    ;;
stop_one)
    stopSingle $2
    ;;
exec_chat)
    exec_chat
    ;;
exec_sql)
    exec_sql
    ;;
query_tables)
    query_tables
    ;;
*)
    echo "Usage: $0 [start/stop/exec_chat/exec_sql/query_tables]"
    exit 1
    ;;
esac
