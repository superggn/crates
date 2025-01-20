use bytes::BytesMut;

use futures::{ready, FutureExt, Sink, Stream};
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{read_frame, FrameCoder, KvError};

/// 处理 KV server prost frame 的 stream
pub struct ProstStream<S, In, Out> {
    // innner stream
    stream: S,
    // 写缓存
    wbuf: BytesMut,
    // 读缓存
    rbuf: BytesMut,

    // 类型占位符
    _in: PhantomData<In>,
    _out: PhantomData<Out>,
}

impl<S, In, Out> Stream for ProstStream<S, In, Out>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    In: Unpin + Send + FrameCoder,
    Out: Unpin + Send,
{
    /// 当调用 next() 时，得到 Result<In, KvError>
    type Item = Result<In, KvError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // 上一次调用结束后 rbuf 应该为空
        assert!(self.rbuf.len() == 0);

        // 从 rbuf 中分离出 rest（摆脱对 self 的引用）
        let mut rest = self.rbuf.split_off(0);

        // 使用 read_frame 来获取数据
        let fut = read_frame(&mut self.stream, &mut rest);
        // let mut a = Box::pin(fut);
        ready!(Box::pin(fut).poll_unpin(cx))?;

        // 拿到一个 frame 的数据，把 buffer 合并回去
        self.rbuf.unsplit(rest);

        // 调用 decode_frame 获取解包后的数据
        Poll::Ready(Some(In::decode_frame(&mut self.rbuf)))
    }
}

/// 当调用 send() 时，会把 Out 发出去
impl<S, In, Out> Sink<Out> for ProstStream<S, In, Out>
where
    S: AsyncRead + AsyncWrite + Unpin,
    In: Unpin + Send,
    Out: Unpin + Send + FrameCoder,
{
    /// 如果发送出错，会返回 KvError
    type Error = KvError;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn start_send(self: Pin<&mut Self>, item: Out) -> Result<(), Self::Error> {
        todo!()
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }
}
