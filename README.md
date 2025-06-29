


          
# FTServo SDK for Rust

一个用于控制飞特舵机(FTServo)设备的Rust库，支持SMS和STS系列舵机以及SCSCL系列舵机。

## 特性

- 🚀 **高性能**: 使用Rust编写，提供零成本抽象和内存安全
- 🔧 **完整功能**: 支持单个舵机控制、同步控制、批量读取等操作
- 📡 **多协议支持**: 支持SMS/STS和SCSCL两种协议
- 🛡️ **类型安全**: 强类型系统确保运行时安全
- 📚 **易于使用**: 简洁的API设计，丰富的示例代码

## 支持的设备

### SMS/STS 系列舵机
- SMS40系列
- STS3032系列
- STS3215系列
- 其他兼容SMS/STS协议的舵机

### SCSCL 系列舵机
- SCSCL系列数字舵机
- 其他兼容SCSCL协议的舵机

## 安装

在你的 `Cargo.toml` 文件中添加以下依赖：

```toml
[dependencies]
ftservo_sdk = "0.1.0"
```

或者使用 cargo 命令安装：

```bash
cargo add ftservo_sdk
```

## 快速开始

### 基本使用示例

```rust
use ftservo_sdk::{
    create_port_handler, create_sms_sts,
    COMM, Result
};
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    // 创建端口处理器
    let mut port_handler = create_port_handler("/dev/ttyUSB0");
    
    // 设置波特率并打开端口
    port_handler.set_baudrate(1000000)?;
    port_handler.open_port()?;
    
    // 创建SMS/STS舵机控制器
    let mut sms_sts = create_sms_sts(port_handler);
    
    // 测试连接
    let ping_result = sms_sts.ping(1);
    match ping_result {
        COMM::Success => println!("[ID:001] 连接成功"),
        _ => println!("[ID:001] 连接失败: {:?}", ping_result),
    }
    
    // 使能扭矩
    sms_sts.write_torque_enable(1, true);
    
    // 控制舵机移动到指定位置
    let result = sms_sts.write_pos_ex(1, 2048, 1000, 2400);
    println!("位置控制结果: {:?}", result);
    
    // 等待运动完成
    thread::sleep(Duration::from_millis(2000));
    
    // 读取当前位置
    match sms_sts.read_pos(1) {
        Ok(pos) => println!("当前位置: {}", pos),
        Err(e) => println!("读取位置失败: {:?}", e),
    }
    
    Ok(())
}
```

### SCSCL 舵机使用示例

```rust
use ftservo_sdk::{
    create_port_handler, create_scscl,
    COMM, Result
};

fn main() -> Result<()> {
    let mut port_handler = create_port_handler("/dev/ttyUSB0");
    port_handler.set_baudrate(1000000)?;
    port_handler.open_port()?;
    
    let mut scscl = create_scscl(port_handler);
    
    // 控制SCSCL舵机
    scscl.write_pos(1, 2048, 1000, 2400);
    
    // 读取状态
    match scscl.read_pos(1) {
        Ok(pos) => println!("SCSCL位置: {}", pos),
        Err(e) => println!("读取失败: {:?}", e),
    }
    
    Ok(())
}
```

### 同步控制多个舵机

```rust
use ftservo_sdk::{
    create_port_handler, create_sms_sts,
    COMM
};

fn sync_control_example() {
    let port_handler = create_port_handler("/dev/ttyUSB0");
    let mut sms_sts = create_sms_sts(port_handler);
    
    // 同步控制多个舵机
    let ids = vec![1, 2, 3];
    let positions = vec![1024, 2048, 3072];
    let times = vec![1000, 1000, 1000];
    let speeds = vec![2400, 2400, 2400];
    
    let result = sms_sts.sync_write_pos_ex(ids, positions, times, speeds);
    match result {
        COMM::Success => println!("同步控制成功"),
        _ => println!("同步控制失败: {:?}", result),
    }
}
```

## API 文档

### 核心结构体

#### `PortHandler`
串口通信处理器，负责底层串口通信。

```rust
let mut port_handler = PortHandler::new("/dev/ttyUSB0");
port_handler.set_baudrate(1000000)?;
port_handler.open_port()?;
```

#### `SmsSts`
SMS/STS系列舵机控制器。

**主要方法：**
- `ping(id)` - 测试舵机连接
- `write_pos_ex(id, pos, time, speed)` - 写入位置（扩展模式）
- `read_pos(id)` - 读取当前位置
- `read_speed(id)` - 读取当前速度
- `write_torque_enable(id, enable)` - 控制扭矩使能
- `sync_write_pos_ex(ids, positions, times, speeds)` - 同步位置控制

#### `Scscl`
SCSCL系列舵机控制器。

**主要方法：**
- `write_pos(id, pos, time, speed)` - 写入位置
- `read_pos(id)` - 读取位置
- `read_load(id)` - 读取负载
- `read_voltage(id)` - 读取电压
- `read_temperature(id)` - 读取温度

### 错误处理

库定义了自定义错误类型 `FtServoError`：

```rust
pub enum FtServoError {
    SerialPort(serialport::Error),
    Communication(COMM),
    InvalidParameter(String),
    Timeout,
    ChecksumError,
    Io(std::io::Error),
}
```

### 通信结果

`COMM` 枚举表示通信结果：

```rust
pub enum COMM {
    Success,        // 成功
    PortBusy,      // 端口忙
    TxFail,        // 发送失败
    RxFail,        // 接收失败
    TxError,       // 发送错误
    RxWaiting,     // 等待接收
    RxTimeout,     // 接收超时
    RxCorrupt,     // 数据损坏
    NotAvailable,  // 功能不可用
}
```

## 示例程序

项目包含多个示例程序，展示不同的使用场景：

### 运行基本控制示例

```bash
cargo run --example basic_control
```

### 运行同步控制示例

```bash
cargo run --example sync_control
```

### 运行状态读取示例

```bash
cargo run --example read_status
```

## 硬件连接

### 串口连接

1. **USB转TTL模块**: 推荐使用CH340、CP2102或FT232等芯片的USB转TTL模块
2. **接线方式**:
   - VCC → 舵机电源正极（通常5V-12V）
   - GND → 舵机电源负极和USB转TTL的GND
   - TX → 舵机数据线
   - RX → 舵机数据线（与TX连接同一根线）

### 波特率设置

常用波特率：
- `1000000` - 1Mbps（推荐）
- `500000` - 500Kbps
- `115200` - 115.2Kbps
- `57600` - 57.6Kbps

## 平台支持

- ✅ **Linux** (Ubuntu, Debian, CentOS等)
- ✅ **macOS**
- ✅ **Windows**
- ✅ **嵌入式Linux** (Raspberry Pi等)

## 故障排除

### 常见问题

1. **无法打开串口**
   ```
   错误: SerialPort(Os { code: 13, kind: PermissionDenied, message: "Permission denied" })
   ```
   **解决方案**: 在Linux上添加用户到dialout组
   ```bash
   sudo usermod -a -G dialout $USER
   # 重新登录或重启
   ```

2. **舵机无响应**
   - 检查接线是否正确
   - 确认波特率设置
   - 验证舵机ID是否正确
   - 检查电源供应

3. **通信超时**
   - 降低波特率重试
   - 检查数据线连接
   - 确认舵机协议类型

### 调试技巧

启用详细日志输出：

```bash
RUST_LOG=debug cargo run --example basic_control
```

## 开发指南

### 构建项目

```bash
# 克隆仓库
git clone https://github.com/BriceLucifer/ftservo_sdk.git
cd ftservo_sdk

# 构建项目
cargo build

# 运行测试
cargo test

# 构建文档
cargo doc --open
```

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

## 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

## 许可证

本项目采用 MIT 或 Apache-2.0 双重许可证。详见 [LICENSE-MIT](LICENSE-MIT) 和 [LICENSE-APACHE](LICENSE-APACHE) 文件。

## 更新日志

### v0.1.0 (2024-01-XX)
- 🎉 初始版本发布
- ✨ 支持SMS/STS和SCSCL协议
- 🚀 完整的舵机控制功能
- 📚 详细的文档和示例

## 联系方式

- **作者**: BriceLucifer
- **邮箱**: 2376671337@qq.com
- **仓库**: https://github.com/BriceLucifer/ftservo_sdk
- **问题反馈**: https://github.com/BriceLucifer/ftservo_sdk/issues

## 致谢

感谢飞特科技提供的舵机产品和技术支持。

---

**注意**: 使用本库时请确保正确连接硬件，错误的接线可能损坏设备。建议在实际应用前先进行充分测试。
        