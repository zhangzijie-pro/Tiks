#!/bin/bash

# 获取用户主目录路径
HOME_DIR="$HOME"

# 创建 .my_app 文件夹
APP_DIR="$HOME_DIR/.Tiks"
mkdir -p "$APP_DIR"

# 创建 user.txt 文件
USER_FILE="$APP_DIR/tiks"
touch "$USER_FILE"
