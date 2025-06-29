//! 基本舵机控制示例
//! 对应Python SDK中的基本控制功能

use ftservo_sdk::{
    create_port_handler, create_sms_sts, create_scscl,
    COMM, Result
};
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    println!("=== FTServo SDK 基本控制示例 ===");
    
    // 配置串口参数
    let port_name = "/dev/ttyUSB0";  // Linux/macOS
    // let port_name = "COM3";      // Windows
    let baudrate = 1000000;
    
    // 创建端口处理器
    let mut port_handler = create_port_handler(port_name);
    
    // 设置波特率并打开端口
    println!("正在打开串口: {}, 波特率: {}", port_name, baudrate);
    port_handler.set_baudrate(baudrate)?;
    port_handler.open_port()?;
    println!("串口打开成功!");
    
    // 创建SMS/STS舵机控制器
    let mut sms_sts = create_sms_sts(port_handler);
    
    let servo_id = 1;
    
    // 1. 测试连接 (对应Python: sms_sts.ping(servo_id))
    println!("\n--- 测试舵机连接 ---");
    let ping_result = sms_sts.ping(servo_id);
    match ping_result {
        COMM::Success => println!("[ID:{}] 连接成功 ✓", servo_id),
        _ => {
            println!("[ID:{}] 连接失败: {:?}", servo_id, ping_result);
            return Ok(());
        }
    }
    
    // 2. 使能扭矩 (对应Python: sms_sts.EnableTorque(servo_id, 1))
    println!("\n--- 使能扭矩 ---");
    let result = sms_sts.write_torque_enable(servo_id, true);
    match result {
        COMM::Success => println!("[ID:{}] 扭矩使能成功 ✓", servo_id),
        _ => println!("[ID:{}] 扭矩使能失败: {:?}", servo_id, result),
    }
    
    // 3. 写入位置 (对应Python: sms_sts.WritePosEx(servo_id, position, time, speed))
    println!("\n--- 位置控制 ---");
    let target_position = 2048;  // 中间位置
    let move_time = 1000;        // 1秒
    let move_speed = 2400;       // 速度
    
    println!("控制舵机移动到位置: {}", target_position);
    let result = sms_sts.write_pos_ex(servo_id, target_position, move_time, move_speed);
    match result {
        COMM::Success => println!("[ID:{}] 位置控制指令发送成功 ✓", servo_id),
        _ => println!("[ID:{}] 位置控制失败: {:?}", servo_id, result),
    }
    
    // 等待运动完成
    println!("等待舵机运动完成...");
    thread::sleep(Duration::from_millis(1500));
    
    // 4. 读取当前位置 (对应Python: sms_sts.ReadPos(servo_id))
    println!("\n--- 读取状态 ---");
    match sms_sts.read_pos(servo_id) {
        Ok(pos) => println!("[ID:{}] 当前位置: {}", servo_id, pos),
        Err(e) => println!("[ID:{}] 读取位置失败: {:?}", servo_id, e),
    }
    
    // 5. 读取当前速度 (对应Python: sms_sts.ReadSpeed(servo_id))
    match sms_sts.read_speed(servo_id) {
        Ok(speed) => println!("[ID:{}] 当前速度: {}", servo_id, speed),
        Err(e) => println!("[ID:{}] 读取速度失败: {:?}", servo_id, e),
    }
    
    // 6. 读取负载 (对应Python: sms_sts.ReadLoad(servo_id))
    match sms_sts.read_load(servo_id) {
        Ok(load) => println!("[ID:{}] 当前负载: {}", servo_id, load),
        Err(e) => println!("[ID:{}] 读取负载失败: {:?}", servo_id, e),
    }
    
    // 7. 读取电压 (对应Python: sms_sts.ReadVoltage(servo_id))
    match sms_sts.read_voltage(servo_id) {
        Ok(voltage) => println!("[ID:{}] 当前电压: {}V", servo_id, voltage as f32 / 10.0),
        Err(e) => println!("[ID:{}] 读取电压失败: {:?}", servo_id, e),
    }
    
    // 8. 读取温度 (对应Python: sms_sts.ReadTemper(servo_id))
    match sms_sts.read_temperature(servo_id) {
        Ok(temp) => println!("[ID:{}] 当前温度: {}°C", servo_id, temp),
        Err(e) => println!("[ID:{}] 读取温度失败: {:?}", servo_id, e),
    }
    
    // 9. 禁用扭矩 (对应Python: sms_sts.EnableTorque(servo_id, 0))
    println!("\n--- 禁用扭矩 ---");
    let result = sms_sts.write_torque_enable(servo_id, false);
    match result {
        COMM::Success => println!("[ID:{}] 扭矩禁用成功 ✓", servo_id),
        _ => println!("[ID:{}] 扭矩禁用失败: {:?}", servo_id, result),
    }
    
    println!("\n=== 基本控制示例完成 ===");
    Ok(())
}