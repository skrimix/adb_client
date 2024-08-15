#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use adb_client::{ADBServer, ADBServerDevice, DeviceLong, DeviceState};
    use rand::Rng;

    fn new_client() -> ADBServer {
        ADBServer::default()
    }

    fn new_device() -> ADBServerDevice {
        let mut client = new_client();
        return client.get_device().expect("cannot get device");
    }

    #[test]
    fn test_version() {
        let mut adb = new_client();
        adb.version().unwrap();
    }

    #[test]
    fn test_shell() {
        let mut device = new_device();

        device.shell_command(vec!["ls"]).unwrap();
        device.shell_command(vec!["pwd"]).unwrap();
    }

    #[test]
    fn test_devices() {
        let mut adb = new_client();
        adb.devices().unwrap();
    }

    #[test]
    fn test_devices_long() {
        let mut adb = new_client();
        adb.devices_long().unwrap();
    }

    #[test]
    fn test_static_devices_long() {
        let inputs = ["7a5158f05122195aa       device 1-5 product:gts210vewifixx model:SM_T813 device:gts210vewifi transport_id:4", 
        "1WMHH810H12203         device usb:2-3.2 product:hollywood model:Quest_2 device:hollywood transport_id:5"];
        let device1 = DeviceLong::try_from(inputs[0].as_bytes().to_vec()).unwrap_or_else(|_| panic!("cannot parse input: '{}'", inputs[0]));
        let device2 = DeviceLong::try_from(inputs[1].as_bytes().to_vec()).unwrap_or_else(|_| panic!("cannot parse input: '{}'", inputs[1]));

        assert_eq!(device1.identifier, "7a5158f05122195aa");
        assert!(matches!(device1.state, DeviceState::Device));
        assert_eq!(device1.product, "gts210vewifixx");
        assert_eq!(device1.model, "SM_T813");
        assert_eq!(device1.device, "gts210vewifi");
        assert_eq!(device1.transport_id, 4);

        assert_eq!(device2.identifier, "1WMHH810H12203");
        assert!(matches!(device2.state, DeviceState::Device));
        assert_eq!(device2.product, "hollywood");
        assert_eq!(device2.model, "Quest_2");
        assert_eq!(device2.device, "hollywood");
        assert_eq!(device2.transport_id, 5);
    }

    #[test]
    fn test_send_recv() {
        // Create random "Reader" in memory
        let mut key = [0u8; 1000];
        rand::thread_rng().fill(&mut key[..]);
        let mut c: Cursor<Vec<u8>> = Cursor::new(key.to_vec());

        let mut device = new_device();

        const TEST_FILENAME: &'static str = "/data/local/tmp/test_file";
        // Send it
        device
            .send(&mut c, TEST_FILENAME)
            .expect("cannot send file");

        // Pull it to memory
        let mut res = vec![];
        device
            .recv(TEST_FILENAME, &mut res)
            .expect("cannot recv file");

        // diff
        assert_eq!(c.get_ref(), &res);

        device
            .shell_command::<&str>([format!("rm {TEST_FILENAME}").as_str()])
            .expect("cannot remove test file");
    }

    #[test]
    fn multiple_connexions() {
        let mut connection = new_client();

        for _ in 0..2 {
            let _ = connection.devices().expect("cannot get version");
        }
    }

    // #[test]
    // fn command_emulator() {
    //     let mut connection = new_client();
    //     let mut emulator = connection
    //         .get_emulator_device()
    //         .expect("no emulator running");
    //     emulator.hello().expect("cannot hello");
    // }
}
