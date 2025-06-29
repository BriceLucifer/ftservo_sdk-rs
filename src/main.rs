use ftservo_sdk::{
    create_port_handler, create_sms_sts, 
    COMM, Result
};
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    println!("FTServo SDK Rust Demo");
    
    // 创建端口处理器
    let mut port_handler = create_port_handler("/dev/ttyUSB0");
    
    // 设置波特率并打开端口
    port_handler.set_baudrate(1000000)?;
    port_handler.open_port()?;
    
    // 创建SMS/STS舵机控制器
    let mut sms_sts = create_sms_sts(port_handler);
    
    // 测试ping
    let ping_result = sms_sts.ping(1);
    match ping_result {
        COMM::Success => println!("[ID:001] ping成功"),
        _ => println!("[ID:001] ping失败: {:?}", ping_result),
    }
    
    // 使能扭矩
    sms_sts.write_torque_enable(1, true);
    
    // 写入位置
    let result = sms_sts.write_pos_ex(1, 2048, 1000, 2400);
    println!("写入位置结果: {:?}", result);
    
    // 等待运动完成
    thread::sleep(Duration::from_millis(2000));
    
    // 读取当前位置
    match sms_sts.read_pos(1) {
        Ok(pos) => println!("当前位置: {}", pos),
        Err(e) => println!("读取位置失败: {:?}", e),
    }
    
    // 读取当前速度
    match sms_sts.read_speed(1) {
        Ok(speed) => println!("当前速度: {}", speed),
        Err(e) => println!("读取速度失败: {:?}", e),
    }
    
    // 同步控制多个舵机
    let ids = vec![1, 2, 3];
    let positions = vec![1024, 2048, 3072];
    let times = vec![1000, 1000, 1000];
    let speeds = vec![2400, 2400, 2400];
    
    let sync_result = sms_sts.sync_write_pos_ex(ids, positions, times, speeds);
    println!("同步写入结果: {:?}", sync_result);
    
    Ok(())
}
