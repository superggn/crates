# PROST_DEMO

## 概述

说人话

- prost + prost-build
    - 写 proto, build .rs 文件
    - 通过环境变量拿 .rs 文件来开发， 不要把自动生成的 .rs 文件放到 src 里
- prost + prost-types
    - 不用自己写 proto 文件
    - 一些常见的结果， 比如时间戳啥的， 都给定义好了， 直接用即可



当前文件为 tokio 旗下的 prost / prost_build / prost_types 整理了一些例子， 供参考

例子都在 main.rs 里， 直接 cargo run 即可



#### repo

https://github.com/tokio-rs/prost

没错就只有一个 prost repo, 几个 crate 都在这个下面



#### docs

https://docs.rs/prost/latest/prost/

https://docs.rs/prost-build/latest/prost_build/

https://docs.rs/prost-types/latest/prost_types/





##### prost meta actions

https://chatgpt.com/c/67c525dd-f554-8004-acc1-bb2c45201e8a

- dependencies

    - ```toml
        [dependencies]
        prost = "0.13"
        prost-types = "0.13"  # 处理 well-known types，如 `Timestamp`
        
        [build-dependencies]
        prost-build = "0.13"
        
        ```

- proto files

    - 见 demo

- build & include

    - build.rs

    - ```rust
        fn main() {
            prost_build::compile_protos(&["src/items.proto"], &["src"]).unwrap();
        }
        ```

    - include

    - ```rust
        mod my_message {
            include!(concat!(env!("OUT_DIR"), "/items.proto.rs"));
            impl Item {...}
            impl From<i32> for Item {...}
        }
        ```

- serdes

    - customized structs
    - WKT (prost-types)

- 扩展 proto

    - merge - 用得少， 不用管
    - oneof
    - derive macro （兼容 serde 等 trait， ...）



其实 proto 这玩意儿定义的消息结构体是可以和所有表示形式兼容的， 比如 json



同一个消息， 内存里定义完了之后， derive 一个 Serialize / Deserialize， 转 proto 再转回来， 转 json 再转回来， 都一样



