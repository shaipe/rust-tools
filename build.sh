#!/bin/bash

echo "start build react ..."

# 使用webpack对项目进行打包发布
# yarn build

# copy ./build/ 
# exit 1
# 将打包的build目录进行压缩
echo "build react end, start zip ..."
zip -q -r build.zip ./build/

# 拷贝压缩文件到本地测试目录
cp build.zip ~/dist/kide

# 删除打包的临时文件
rm -f build.zip

if [ $1 == "local" ];
then
    echo "begin local deploy ..."
    
    # 进入本地测试目录并进行解压和处理
    cd ~/dist/kide
    unzip build.zip
    mv build www
    echo "build and local deploy end ."
else
    echo "build end ."
fi
