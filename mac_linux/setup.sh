#!/bin/bash

# 获取用户主目录路径
HOME_DIR="$HOME"

# 创建 .my_app 文件夹
APP_DIR="$HOME_DIR/.Tiks"
mkdir -p "$APP_DIR"
mkdir -p bin

# 创建 user: password 文件
USER_FILE="$APP_DIR/tiks"
touch "$USER_FILE"

cd $HOME/.Tiks/bin
cp ./target/release/tiks  $HOME/.Tiks/bin

export PATH=$PATH:/$HOME/.Tiks/bin/ >> $HOME/.bashrc