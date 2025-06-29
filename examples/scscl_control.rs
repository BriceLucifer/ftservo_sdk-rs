//! SCSCL系列舵机控制示例
//! 对应Python SDK中的SCSCL功能

use ftservo_sdk::{
    create_port_handler, create_scscl,
    COMM, Result
};
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    println!("=== SCSCL舵机控制示例 ===");
    
    // 配置串口参数
    let port_name = "/dev/ttyUSB0";
    let baudrate = 1000000;
    
    // 创建端口处理器
    let mut port_handler = create_port_handler(port_name);
    port_handler.set_baudrate(baudrate)?;
    port_handler.open_port()?;
    println!("串口打开成功!");
    
    // 创建SCSCL舵机控制器
    let mut scscl = create_scscl(port_handler);
    
    let servo_id = 1;
    
    // 1. 测试连接 (对应Python: scscl.ping(servo_id))
    println!("\n--- 测试SCSCL舵机连接 ---");
    let ping_result = scscl.ping(servo_id);
    match ping_result {
        COMM::Success => println!("[ID:{}] SCSCL舵机连接成功 ✓", servo_id),
        _ => {
            println!("[ID:{}] SCSCL舵机连接失败: {:?}", servo_id, ping_result);
            return Ok(());
        }
    }
    
    // 2. 使能扭矩 (对应Python: scscl.EnableTorque(servo_id, 1))
    println!("\n--- 使能扭矩 ---");
    let result = scscl.write_torque_enable(servo_id, true);
    match result {
        COMM::Success => println!("[ID:{}] 扭矩使能成功 ✓", servo_id),
        _ => println!("[ID:{}] 扭矩使能失败: {:?}", servo_id, result),
    }
    
    // 3. 位置控制 (对应Python: scscl.WritePos(servo_id, position, time, speed))
    println!("\n--- SCSCL位置控制 ---");
    let target_position = 2048;
    let move_time = 1000;
    let move_speed = 2400;
    
    println!("控制SCSCL舵机移动到位置: {}", target_position);
    let result = scscl.write_pos(servo_id, target_position, move_time, move_speed);
    match result {
        COMM::Success => println!("[ID:{}] 位置控制指令发送成功 ✓", servo_id),
        _ => println!("[ID:{}] 位置控制失败: {:?}", servo_id, result),
    }
    
    thread::sleep(Duration::from_millis(1500));
    
    // 4. 读取状态信息
    println!("\n--- 读取SCSCL舵机状态 ---");
    
    // 读取位置 (对应Python: scscl.ReadPos(servo_id))
    match scscl.read_pos(servo_id) {
        Ok(pos) => println!("[ID:{}] 当前位置: {}", servo_id, pos),
        Err(e) => println!("[ID:{}] 读取位置失败: {:?}", servo_id, e),
    }
    
    // 读取速度 (对应Python: scscl.ReadSpeed(servo_id))
    match scscl.read_speed(servo_id) {
        Ok(speed) => println!("[ID:{}] 当前速度: {}", servo_id, speed),
        Err(e) => println!("[ID:{}] 读取速度失败: {:?}", servo_id, e),
    }
    
    // 读取负载 (对应Python: scscl.ReadLoad(servo_id))
    match scscl.read_load(servo_id) {
        Ok(load) => println!("[ID:{}] 当前负载: {}", servo_id, load),
        Err(e) => println!("[ID:{}] 读取负载失败: {:?}", servo_id, e),
    }
    
    // 读取电压 (对应Python: scscl.ReadVoltage(servo_id))
    match scscl.read_voltage(servo_id) {
        Ok(voltage) => println!("[ID:{}] 当前电压: {}V", servo_id, voltage as f32 / 10.0),
        Err(e) => println!("[ID:{}] 读取电压失败: {:?}", servo_id, e),
    }
    
    // 读取温度 (对应Python: scscl.ReadTemper(servo_id))
    match scscl.read_temperature(servo_id) {
        Ok(temp) => println!("[ID:{}] 当前温度: {}°C", servo_id, temp),
        Err(e) => println!("[ID:{}] 读取温度失败: {:?}", servo_id, e),
    }
    
    // 读取型号 (对应Python: scscl.ReadModel(servo_id))
    match scscl.read_model(servo_id) {
        Ok(model) => println!("[ID:{}] 舵机型号: {}", servo_id, model),
        Err(e) => println!("[ID:{}] 读取型号失败: {:?}", servo_id, e),
    }
    
    // 5. 同步控制多个SCSCL舵机
    println!("\n--- SCSCL同步控制 ---");
    let servo_ids = vec![1, 2, 3];
    let positions = vec![1024, 2048, 3072];
    let times = vec![1000, 1000, 1000];
    let speeds = vec![2400, 2400, 2400];
    
    let result = scscl.sync_write_pos(servo_ids, positions, times, speeds);
    match result {
        COMM::Success => println!("SCSCL同步控制指令发送成功 ✓"),
        _ => println!("SCSCL同步控制失败: {:?}", result),
    }
    
    thread::sleep(Duration::from_millis(1500));
    
    // 6. 禁用扭矩
    println!("\n--- 禁用扭矩 ---");
    let result = scscl.write_torque_enable(servo_id, false);
    match result {
        COMM::Success => println!("[ID:{}] 扭矩禁用成功 ✓", servo_id),
        _ => println!("[ID:{}] 扭矩禁用失败: {:?}", servo_id, result),
    }
    
    println!("\n=== SCSCL控制示例完成 ===");
    Ok(())
}