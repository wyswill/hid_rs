use hidapi::HidApi;

fn main() {
    // 初始化 HID API
    let api = HidApi::new().expect("Failed to initialize HID API");

    // Read: [0, 0, 0, 0, 0, 255] down
    // Read: [0, 0, 0, 0, 0, 1] up
    let (VID, PID) = (0x0b05, 0x18dd);
    // let (VID, PID) = (2821, 6365);
    let device = api.open(VID, PID).unwrap();
    // Read data from device
    // let mut buf = [0u8; 8];
    // loop {
    //     let res = device.read(&mut buf[..]).unwrap();
    //     println!("Read: {:?}", &buf[..res]);
    // }

    // Write data to device
    for _i in 0..20 {
        let buf = [0u8, 0, 0, 0, 0, 1];
        let res = device.send_feature_report(&buf).unwrap();
        println!("Wrote: {:?} byte(s)", res);
    }
    drop(device);
}

// fn _sendCmd() {
//     let api = HidApi::new().expect("Failed to initialize HID API");
//     // 查找鼠标设备
//     let device_info = api
//         .device_list()
//         .find(|info| info.vendor_id() == 2821 && info.product_id() == 6365)
//         .expect("Failed to find mouse device");

//     // 打开鼠标设备
//     let device = device_info
//         .open_device(&api)
//         .expect("Failed to open mouse device");

//     // 创建鼠标滚轮滚动事件报文
//     let report_data: [u8; 16] = [
//         0x00, 0x38, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//         0x00,
//     ];
//     // 滚轮变化。0x01表示滚轮向前滚动一格；0xFF表示滚轮向后滚动一格；0x80是个中间值，不滚动
//     // /**
//     //  *  0x38; // 鼠标滚轮滚动事件的标识符
//     //     0x01; // 滚动方向和速度，正值表示向上滚动，负值表示向下滚动
//     //  */
//     for _i in 0..100 {
//         println!("{}", _i);
//         // 发送鼠标滚轮滚动事件报文
//         device
//             .send_feature_report(&report_data)
//             .expect("Failed to send mouse scroll event");
//     }

//     // 关闭鼠标设备
//     drop(device);
// }
