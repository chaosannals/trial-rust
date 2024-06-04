# QuickWit

```bash
# 【请求】下载 示例 索引 配置文件。
curl -o stackoverflow-index-config.yaml https://raw.githubusercontent.com/quickwit-oss/quickwit/main/config/tutorials/stackoverflow/index-config.yaml

# 【命令】通过 yaml 创建索引
./quickwit index create --index-config ./stackoverflow-index-config.yaml
# 【请求】cmd 通过 yaml 创建索引
curl -XPOST http://127.0.0.1:7280/api/v1/indexes --header "content-type: application/yaml" --data-binary @./stackoverflow-index-config.yaml

# 下载插入的示例数据
curl -O https://quickwit-datasets-public.s3.amazonaws.com/stackoverflow.posts.transformed-10000.json

# 插入测试数据
curl -XPOST "http://127.0.0.1:7280/api/v1/stackoverflow/ingest?commit=force" --data-binary @stackoverflow.posts.transformed-10000.json

# 查询
curl "http://127.0.0.1:7280/api/v1/stackoverflow/search?query=search+AND+engine"
```