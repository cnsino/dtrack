# dtrack

dtrack is the rust script to facilitate usage of DependencyTrack in the CI, optionally failing the build based on different parameters.


Usage: dtrack [OPTIONS]

Options:
  -u, --url <Url>          设置dependencytrack的访问地址. eg. http://dtrack.abc.local:8080 [default: http://127.0.0.1:8080]
  -k, --key <Apikey>       设置dependencytrack的apikey. eg. adfadfe343g
  -p, --project <Project>  设置dependencytrack的项目名称
  -f, --file <File>        设置dependencytrack的文件
  -r, --rule <Rule>        设置dependencytrack的规则
  -h, --help               Print help
  -V, --version            Print version