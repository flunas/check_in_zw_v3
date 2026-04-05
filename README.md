```
签到流程：
1、获取b2_token
2、通过sign.rs进行签到

获取b2_token有两种方式
1、通过b2_token.rst获取 (需要启动独立的chromedriver服务，该服务内存占用大)
2、通过b2_token_by_headless_chrome.rs获取 (推荐这种方式，不需要独立的chromedriver服务)
```
## 构建镜像有两种方式
### 1、Dockerfile + build.sh 构建镜像
### 2、Containerfile + workflow 构建镜像