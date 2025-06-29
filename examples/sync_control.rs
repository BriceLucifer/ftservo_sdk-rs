//! 同步控制多个舵机示例
//! 对应Python SDK中的SyncWrite功能

use ftservo_sdk::{
    create_port_handler, create_sms_sts,
    COMM, Result
};
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    println!("=== FTServo SDK 同步控制示例 ===");
    
    // 配置串口参数
    let port_name = "/dev/ttyUSB0";
    let baudrate = 1000000;
    
    // 创建端口处理器
    let mut port_handler = create_port_handler(port_name);
    port_handler.set_baudrate(baudrate)?;
    port_handler.open_port()?;
    println!("串口打开成功!");
    
    // 创建SMS/STS舵机控制器
    let mut sms_sts = create_sms_sts(port_handler);
    
    // 定义要控制的舵机
    let servo_ids = vec![1, 2, 3];
    
    // 1. 测试所有舵机连接
    println!("\n--- 测试舵机连接 ---");
    for &id in &servo_ids {
        let ping_result = sms_sts.ping(id);
        match ping_result {
            COMM::Success => println!("[ID:{}] 连接成功 ✓", id),
            _ => println!("[ID:{}] 连接失败: {:?}", id, ping_result),
        }
    }
    
    // 2. 使能所有舵机扭矩
    println!("\n--- 使能扭矩 ---");
    for &id in &servo_ids {
        let result = sms_sts.write_torque_enable(id, true);
        match result {
            COMM::Success => println!("[ID:{}] 扭矩使能成功 ✓", id),
            _ => println!("[ID:{}] 扭矩使能失败: {:?}", id, result),
        }
    }
    
    // 3. 同步位置控制 - 动作1
    // 对应Python: sms_sts.SyncWritePosEx(servo_ids, positions, times, speeds)
    println!("\n--- 同步控制 - 动作1 ---");
    let positions1 = vec![1024, 2048, 3072];  // 不同位置
    let times1 = vec![1000, 1000, 1000];     // 1秒完成
    let speeds1 = vec![2400, 2400, 2400];    // 相同速度
    
    println!("执行动作1: 舵机移动到不同位置");
    let result = sms_sts.sync_write_pos_ex(
        servo_ids.clone(), 
        positions1.clone(), 
        times1.clone(), 
        speeds1.clone()
    );
    match result {
        COMM::Success => println!("同步控制指令发送成功 ✓"),
        _ => println!("同步控制失败: {:?}", result),
    }
    
    // 等待动作完成
    thread::sleep(Duration::from_millis(1500));
    
    // 读取所有舵机位置
    println!("\n--- 读取当前位置 ---");
    for &id in &servo_ids {
        match sms_sts.read_pos(id) {
            Ok(pos) => println!("[ID:{}] 当前位置: {}", id, pos),
            Err(e) => println!("[ID:{}] 读取位置失败: {:?}", id, e),
        }
    }
    
    // 4. 同步位置控制 - 动作2
    println!("\n--- 同步控制 - 动作2 ---");
    let positions2 = vec![3072, 1024, 2048];  // 交换位置
    let times2 = vec![1500, 1500, 1500];     // 1.5秒完成
    let speeds2 = vec![1800, 1800, 1800];    // 较慢速度
    
    println!("执行动作2: 舵机交换位置");
    let result = sms_sts.sync_write_pos_ex(
        servo_ids.clone(), 
        positions2, 
        times2, 
        speeds2
    );
    match result {
        COMM::Success => println!("同步控制指令发送成功 ✓"),
        _ => println!("同步控制失败: {:?}", result),
    }
    
    thread::sleep(Duration::from_millis(2000));
    
    // 5. 回到初始位置
    println!("\n--- 回到初始位置 ---");
    let home_positions = vec![2048, 2048, 2048];  // 中间位置
    let home_times = vec![2000, 2000, 2000];      // 2秒完成
    let home_speeds = vec![1200, 1200, 1200];     // 慢速
    
    let result = sms_sts.sync_write_pos_ex(
        servo_ids.clone(), 
        home_positions, 
        home_times, 
        home_speeds
    );
    match result {
        COMM::Success => println!("回到初始位置指令发送成功 ✓"),
        _ => println!("回到初始位置失败: {:?}", result),
    }
    
    thread::sleep(Duration::from_millis(2500));
    
    // 6. 禁用所有舵机扭矩
    println!("\n--- 禁用扭矩 ---");
    for &id in &servo_ids {
        let result = sms_sts.write_torque_enable(id, false);
        match result {
            COMM::Success => println!("[ID:{}] 扭矩禁用成功 ✓", id),
            _ => println!("[ID:{}] 扭矩禁用失败: {:?}", id, result),
        }
    }
    
    println!("\n=== 同步控制示例完成 ===");
    Ok(())
}