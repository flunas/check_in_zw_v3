FROM rust:latest AS builder

# 安装 musl 工具链
RUN apt-get update && \
    apt-get install -y \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

# 使用musl目标编译
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM debian:bookworm-slim

# 安装chrome依赖
RUN apt-get update && apt-get install -y \
    chromium \
    chromium-driver \
    && rm -rf /var/lib/apt/lists/*

# 设置时区和更新时间
RUN ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && \
    echo "Asia/Shanghai" > /etc/timezone && \
    date -R

# 设置环境变量，避免交互式安装
ENV DEBIAN_FRONTEND=noninteractive

# 更新包列表并安装中文字体
RUN apt-get update && \
    apt-get install -y \
    fonts-noto-cjk \
    fonts-wqy-microhei \
    fonts-wqy-zenhei \
    && rm -rf /var/lib/apt/lists/*

# 刷新字体缓存
RUN fc-cache -fv

# 设置环境变量
ENV ELECTRON_DISABLE_SANDBOX=1
ENV PUPPETEER_SKIP_CHROMIUM_DOWNLOAD=true
ENV PUPPETEER_EXECUTABLE_PATH=/usr/bin/chromium

# 创建应用目录
RUN mkdir -p /app/config

# 复制二进制文件
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/check_in_zw_v3 /usr/local/bin/

# 创建非 root 用户
RUN useradd -m -u 1000 appuser
USER appuser

# 设置工作目录
WORKDIR /app


CMD ["check_in_zw_v3", "--no-sandbox"]