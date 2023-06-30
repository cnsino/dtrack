# dtrack

dtrack 是 DependencyTrack 的cli程序

```
Usage: dtrack [OPTIONS]

Options:
  -u, --url <Url>                  设置 Dependency-track 的访问地址. eg. http://dtrack.abc.local:8081 [default: http://127.0.0.1:8081]
  -k, --key <Apikey>               设置 Dependency-track 的apikey. eg. adfadfe343g [default: Oh9LHLfrLgk77e67DEZtiitOWZwvFVXI]
      --proxy <Proxy>              设置 Dependency-track 的连接代理. eg. http://127.0.0.1:8080
  -p, --project <Project Name>     设置 Dependency-track 的项目名称 [default: test]
  -e, --edition <Project Version>  设置 Dependency-track 的项目版本 [default: default]
  -f, --file <Bom File>            设置 Dependency-track 的文件
  -r, --rule <Rule>                设置 Dependency-track 的规则
  -d, --dversion                   输出 Dependency-track 版本信息
  -o, --output <Scan Result>       设置 Dependency-track 的扫描结果存储位置 [default: results.json]
  -l, --log-level <LOG_LEVEL>      设置输出日志的级别(选择off不输出日志) [default: debug] [possible values: off, debug, info, warn, error]
  -h, --help                       Print help
  -V, --version                    Print version
```

## 如何下载

### 1.Cargo
```
cargo install dtrack
```
### 2.Windows
```
直接下载dtrack-x86_64-pc-windows-msvc.zip解压即可
```

### 3.Linux
```
LATEST_VERSION=$(curl -s https://api.github.com/repos/cnsino/dtrack/releases/latest | jq -r '.tag_name')
DOWNLOAD_URL="https://github.com/cnsino/dtrack/releases/download/$LATEST_VERSION/dtrack-x86_64-unknown-linux-gnu.tar.gz"
curl -L $DOWNLOAD_URL
tar -xzvf dtrack-x86_64-unknown-linux-gnu.tar.gz
mv dtrack /usr/local/bin/
```

## 使用方式
```bash
dtrack -p test2 -e 0.0.1 -f /root/bom.xml -o /root/results.json
```