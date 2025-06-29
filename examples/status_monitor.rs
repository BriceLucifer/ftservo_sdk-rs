//! 舵机状态监控示例
//! 对应Python SDK中的状态读取功能

use ftservo_sdk::{
    create_port_handler, create_sms_sts,
    COMM, Result
};
use std::time::Duration;
use std::thread;

fn main() -> Result<()> {
    println!("=== FTServo SDK 状态监控示例 ===");
    
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
    
    let servo_id = 1;
    
    // 测试连接
    let ping_result = sms_sts.ping(servo_id);
    match ping_result {
        COMM::Success => println!("[ID:{}] 舵机连接成功 ✓", servo_id),
        _ => {
            println!("[ID:{}] 舵机连接失败: {:?}", servo_id, ping_result);
            return Ok(());
        }
    }
    
    // 使能扭矩
    sms_sts.write_torque_enable(servo_id, true);
    
    // 开始监控循环
    println!("\n开始状态监控 (按Ctrl+C退出)...");
    println!("时间\t\t位置\t速度\t负载\t电压\t温度\t运动状态");
    println!("{}","-".repeat(70));
    
    for i in 0..30 {  // 监控30次
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // 读取各种状态
        let position = sms_sts.read_pos(servo_id).unwrap_or(-1);
        let speed = sms_sts.read_speed(servo_id).unwrap_or(-1);
        let load = sms_sts.read_load(servo_id).unwrap_or(-1);
        let voltage = sms_sts.read_voltage(servo_id).unwrap_or(0);
        let temperature = sms_sts.read_temperature(servo_id).unwrap_or(0);
        let is_moving = sms_sts.read_moving(servo_id).unwrap_or(false);
        
        println!(
            "{}\t{}\t{}\t{}\t{:.1}V\t{}°C\t{}",
            timestamp,
            position,
            speed,
            load,
            voltage as f32 / 10.0,
            temperature,
            if is_moving { "运动中" } else { "静止" }
        );
        
        // 每5次发送一个新的位置指令
        if i % 5 == 0 {
            let target_pos = if (i / 5) % 2 == 0 { 1024 } else { 3072 };
            sms_sts.write_pos_ex(servo_id, target_pos, 2000, 1800);
            println!(">>> 发送新位置指令: {}", target_pos);
        }
        
        thread::sleep(Duration::from_millis(500));
    }
    
    // 禁用扭矩
    sms_sts.write_torque_enable(servo_id, false);
    println!("\n=== 状态监控示例完成 ===");
    Ok(())
}