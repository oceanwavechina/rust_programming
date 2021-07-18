# 关于 time_wait 状态

## 1. 怎么产生的

首先从头字面上看 time_wait， 它等待的是 "超时时间"， 而不是事件。

>这个超时时间是 2MSL, 即 2 倍的 Maximum Segment Lifetime， 等待2MSL时间主要目的是怕最后一个ACK包对方没收到，那么对方在超时后将重发第三次握手的FIN包，主动关闭端接到重发的FIN包后可以再发一个ACK应答包。在TIME_WAIT状态时两端的端口不能使用，要等到2MSL时间结束才可继续使用。当连接处于2MSL等待阶段时任何迟到的报文段都将被丢弃。

```
Maximum Segment Lifetime

the time a TCP segment can exist in the internetwork system.  

Arbitrarily defined to be 2 minutes.
```

要理解这个东西，先要明白tcp的关闭分为两种：
1. 主动关闭
2. 被动关闭

而 time_wait 就是发生在 **主动关闭** 情况下，根据 rfc793 的 tcp 状态变迁图，可以知道，当主动关闭时，会有如下的状态变迁路径（以当前端为第一视角）

```
ESTABLISHED  
        >>>>    close: send FIN     >>>>   FIN_WAIT_1  

        >>>>    recv ACK            >>>>   FIN_WAIT_2

        >>>>    recv FIN, send ACK  >>>>   TIME_WAIT

        >>>>        wait 2MSL       >>>>   CLOSE
```        

从上边的流程可以知道 主动发起关闭的一方，主要是在等对方的回应。并且最后要回复对方ACK，需要注意的是这个ACK本身并不会得到对方的确认，
也就是不能确保这个ACK一定会被对方收到，为了保证这个连接（四元组）完整的关闭，只有等待一定时间。

问题来了，这个一段时间是怎么确定的。因为 TCP 传输的可靠性依赖超时重传，所以要假设第一次没有传成功，然后重传一次
所以我们假设的最大超时时间就是传输两次的时间，


## 2. 通过对比来理解 MSL

* MSL
  
    他是任何报文在网络上存在的最长时间，超过这个时间报文将被丢弃

* TTL
  
    IP 头中有一个TTL域，TTL是 time to live的缩写，中文可以译为“生存时间”，这个生存时间是由源主机设置初始值但不是存的具体时间，而是存储了一个ip数据报可以经过的最大路由数，每经 过一个处理他的路由器此值就减1，当此值为0则数据报将被丢弃，同时发送ICMP报文通知源主机。

* RTT

    RTT是客户到服务器往返所花时间（round-trip time，简称RTT），TCP含有动态估算RTT的算法。TCP还持续估算一个给定连接的RTT，这是因为RTT受网络传输拥塞程序的变化而变化