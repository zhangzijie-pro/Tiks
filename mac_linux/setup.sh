#!/bin/bash

# 获取用户主目录路径
HOME_DIR="$HOME"

# 创建 .my_app 文件夹
APP_DIR="$HOME_DIR/.Tiks"
mkdir -p "$APP_DIR"


# 创建 user: password 文件
USER_FILE="$APP_DIR/user"
touch "$USER_FILE"

APP_DIR_BIN="$HOME_DIR/.Tiks/bin"
mkdir -p "$APP_DIR_BIN"

# 添加环境变量
#cp ./target/release/tiks  "$APP_DIR_BIN"

#PATH=$PATH:/$HOME/.Tiks/bin