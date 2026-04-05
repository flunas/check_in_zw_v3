#!/bin/bash

# build.sh - 简单 Rust 项目构建脚本

set -e  # 遇到错误立即退出

# 设置环境变量
PROJECT_NAME="check_in_zw_v3"
LIBRARY_NAME="seawindzouus"


echo "🚀 开始构建 Rust 项目..."

# 检查是否安装了 Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ 未安装 Rust, 请先安装 Rust: https://rustup.rs"
    exit 1
fi

# 构建项目
cargo build --release

echo "✅ 构建完成！二进制文件在 target/release/ 目录中"

# 检查是否安装了 Podman
echo "🚀 检查是否安装了 Podman..."
if ! command -v podman &> /dev/null; then
    echo "❌ 未安装 Podman, 请先安装 Podman"
    exit 1
fi

# 构建镜像
echo "🚀 构建镜像..."
podman build -t $PROJECT_NAME:latest .

echo "✅ 镜像构建完成！"

# 获取latest版本号
echo "🚀 获取$PROJECT_NAME:latest版本号..."
if [[ -d "get_docker_image_latest_tag" ]]; then
    cd get_docker_image_latest_tag
    echo "🚀 pull get_docker_image_latest_tag"
    git pull
else
    echo "🚀 clone get_docker_image_latest_tag"
    git clone https://github.com/flunas/get_docker_image_latest_tag.git
    cd get_docker_image_latest_tag
fi
LATEST_VERSION=$(cargo run -- $PROJECT_NAME $LIBRARY_NAME)
echo "✅ $PROJECT_NAME:latest版本号为: $LATEST_VERSION"

# 使用awk分割和处理版本号
NEW_VERSION=$(echo "$LATEST_VERSION" | awk -F. '{
    # 将最后一个字段加1
    $NF = $NF + 1
    # 重新组合为点分隔的字符串
    for (i=1; i<=NF; i++) {
        printf "%s", $i
        if (i < NF) printf "."
    }
}')
echo "🚀 新版本号为: $NEW_VERSION"
cd ..
rm -rf get_docker_image_latest_tag

# 修改镜像tag name
podman tag $PROJECT_NAME:latest $LIBRARY_NAME/$PROJECT_NAME:$NEW_VERSION
echo "🚀 修改镜像tag为 $LIBRARY_NAME/$PROJECT_NAME:$NEW_VERSION"

# 推送镜像
echo "🚀 推送镜像..."
podman push $LIBRARY_NAME/$PROJECT_NAME:$NEW_VERSION
echo "✅ 镜像推送完成！"
