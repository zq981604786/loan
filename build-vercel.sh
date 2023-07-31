#!/bin/bash

# 清理旧的构建结果
rm -rf ./dist

# 编译 Release 版本
trunk build --release

# 拷贝编译生成的静态资源到部署目录
cp -r dist/. ./

