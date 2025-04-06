# ts-derive

Tushare API 的 Rust 过程宏实现

## 概述

这个 crate 提供了两个过程宏，简化了使用 Tushare API 的工作：

1. `TsEndpoint` - 用于定义 API 请求结构体
2. `TsResponse` - 用于定义响应数据模型，映射到 Tushare API 响应

## TsEndpoint

`TsEndpoint` 过程宏允许你定义一个表示 Tushare API 请求的结构体。它会自动实现发送请求和处理响应的方法。

### 使用方法

```rust
use ts_derive::TsEndpoint;
use serde::Serialize;

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "concept", desc = "获取概念股列表", resp = ConceptListResponse)]
struct ConceptListRequest {
    #[serde(rename = "trade_date")]
    trade_date: String,
    // 添加更多参数...
}

// 链式调用方法
async fn example() -> Result<(), Box<dyn std::error::Error>> {
    // 创建请求结构体
    let request = ConceptListRequest {
        trade_date: "20250403".to_string(),
    };
    
    // 以三种方式使用:
    
    // 1. 直接执行，返回原始JSON
    let json = request.clone().execute().await?;
    
    // 2. 指定字段并执行，返回字段映射的字典集合
    let dicts = request.clone()
        .with_fields(vec!["trade_date", "ts_code", "name"])
        .execute_as_dicts()
        .await?;
        
    // 3. 指定字段并执行，返回类型化响应
    let items = request
        .with_fields(vec!["trade_date", "ts_code", "name"])
        .execute_typed()
        .await?;
    
    // ... 处理响应
    
    Ok(())
}
```

### 环境变量

需要在环境变量或`.env`文件中设置`TUSHARE_TOKEN`，例如：

```
TUSHARE_TOKEN=your_token_here
```

### 属性

- `api` - Tushare API 名称/端点（必需）
- `desc` - API 描述（必需）
- `resp` - 响应类型（可选），当指定时可以使用 `execute_typed()` 返回类型化响应

### 方法

- `execute()` - 直接执行请求，无需指定字段，返回原始JSON
- `with_fields(fields)` - 指定请求字段，返回链式调用对象
- `execute_as_dicts()` - 执行请求并返回字段映射的字典集合
- `execute_typed()` - 执行请求并返回类型化的响应对象（需要指定`resp`属性）

## TsResponse

`TsResponse` 过程宏帮助你定义映射到 Tushare API 响应数据的数据结构。它处理将响应项解析为结构化数据。

### 使用方法

```rust
use ts_derive::TsResponse;

#[derive(TsResponse, Debug)]
#[response(api = "concept")]
struct ConceptListResponse {
    #[ts_field(0)]
    trade_date: String,
    #[ts_field(1)]
    ts_code: String,
    #[ts_field(2)]
    name: String,
    #[ts_field(3)]
    z_t_num: i64,
    #[ts_field(4)]
    up_num: String,
}

// 从响应解析
let json_value = response; // 从 TsEndpoint 请求获取的响应
let items = ConceptListResponse::from_json(&json_value)?;
for item in items {
    println!("{}: {}", item.ts_code, item.name);
}
```

### 属性

- `ts_field(index)` - 指定字段在 Tushare 响应项数组中的索引

## 响应结构

Tushare API 响应遵循以下结构：

```json
{
  "request_id": "585b8590-fa21-4150-a097-cdef7cf34861",
  "code": 0,
  "data": {
    "fields": ["trade_date", "ts_code", "name", "z_t_num", "up_num"],
    "items": [
      ["20250403", "000222.KP", "涨价概念", 2, "19"],
      ["20250403", "000164.KP", "核电", 1, "0"],
      // 更多项...
    ],
    "has_more": false,
    "count": -1
  },
  "msg": ""
}
```

`TsResponse` 过程宏使用 `ts_field` 属性中的索引，将 `items` 数组中的值映射到结构体字段。

## 完整示例

以下是一个完整的示例，展示了如何使用这两个过程宏：

```rust
use serde::Serialize;
use ts_derive::{TsEndpoint, TsResponse};

// 定义请求
#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "concept", desc = "获取概念股列表", resp = ConceptListItem)]
pub struct ConceptListRequest {
    pub trade_date: String,
}

// 定义响应项
#[derive(TsResponse, Debug)]
#[response(api = "concept")]
pub struct ConceptListItem {
    #[ts_field(0)]
    pub trade_date: String,
    #[ts_field(1)]
    pub ts_code: String,
    #[ts_field(2)]
    pub name: String,
    #[ts_field(3)]
    pub z_t_num: i64,
    #[ts_field(4)]
    pub up_num: String,
}

// 使用示例
pub async fn example_usage() -> Result<(), Box<dyn std::error::Error>> {
    // 创建请求
    let request = ConceptListRequest {
        trade_date: "20250403".to_string(),
    };
    
    // 链式调用方式，指定字段并返回类型化响应
    let items = request
        .with_fields(vec!["trade_date", "ts_code", "name", "z_t_num", "up_num"])
        .execute_typed()
        .await?;
    
    println!("获取到 {} 个概念股信息", items.len());
    for item in items {
        println!("{}: {}", item.ts_code, item.name);
    }
    
    Ok(())
} 