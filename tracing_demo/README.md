# Tracing crate demo

#### general

##### workflow

跑 jaeger container => 跑 rust 程序， 就可以在 jaeger ui 里搜到日志了

https://www.jaegertracing.io/docs/2.4/deployment/

```sh
docker run --rm --name jaeger \
  -p 16686:16686 \
  -p 4317:4317 \
  -p 4318:4318 \
  -p 5778:5778 \
  -p 9411:9411 \
  jaegertracing/jaeger:2.4.0
```

在这里看 jaeger ui

http://localhost:16686/

rust demo 见 examples

#### links

otlp example

https://github.com/open-telemetry/opentelemetry-rust/blob/main/opentelemetry-otlp/src/lib.rs
