# dtrack

dtrack is the rust script to facilitate usage of DependencyTrack in the CI, optionally failing the build based on different parameters.

```
Usage: dtrack [OPTIONS]

Options:
  -u, --url <Url>                  设置dependencytrack的访问地址. eg. http://dtrack.abc.local:8080 [default: http://127.0.0.1:8081]
  -k, --key <Apikey>               设置dependencytrack的apikey. eg. adfadfe343g [default: Oh9LHLfrLgk77e67DEZtiitOWZwvFVXI]
  -p, --project <Project Name>     设置dependencytrack的项目名称 [default: test]
  -e, --edition <Project Version>  设置dependencytrack的项目版本 [default: default]
  -f, --file <Bom File>            设置dependencytrack的文件
  -r, --rule <Rule>                设置dependencytrack的规则
  -d, --dversion                   输出dependencytrack版本信息
  -o, --output <Scan Result>       设置dependencytrack的扫描结果存储位置 [default: results.json]
  -h, --help                       Print help
  -V, --version                    Print version
```

## 使用方式
```bash
dtrack -p test2 -e 0.0.1 -f /root/bom.xml -o /root/results.json
```