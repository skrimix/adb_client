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
        client.get_device().expect("cannot get device")
    }

    #[test]
    fn test_version() {
        let mut adb = new_client();
        adb.version().unwrap();
    }

    #[test]
    #[ignore]
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
        let inputs = [
            "7a5158f05122195aa       device 1-5 product:gts210vewifixx model:SM_T813 device:gts210vewifi transport_id:4", 
            "1WMHH810H12000         sideload usb:2-3.2 product:hollywood model:Quest_2 device:hollywood transport_id:5",
            "192.168.1.100:5555     device product:hollywood model:Quest_2 device:hollywood transport_id:6",
            "1WMHH810H12000         device 2-3.2 product:hollywood model:Quest_2 device:hollywood transport_id:2",
        ];
        let devices = inputs
            .iter()
            .map(|s| DeviceLong::try_from(s.as_bytes().to_vec()))
            .collect::<Result<Vec<DeviceLong>, _>>()
            .unwrap();
        let expected = [
            DeviceLong {
                identifier: "7a5158f05122195aa".to_string(),
                state: DeviceState::Device,
                usb: "1-5".to_string(),
                product: "gts210vewifixx".to_string(),
                model: "SM_T813".to_string(),
                device: "gts210vewifi".to_string(),
                transport_id: 4,
            },
            DeviceLong {
                identifier: "1WMHH810H12000".to_string(),
                state: DeviceState::Sideload,
                usb: "2-3.2".to_string(),
                product: "hollywood".to_string(),
                model: "Quest_2".to_string(),
                device: "hollywood".to_string(),
                transport_id: 5,
            },
            DeviceLong {
                identifier: "192.168.1.100:5555".to_string(),
                state: DeviceState::Device,
                usb: "Unk".to_string(),
                product: "hollywood".to_string(),
                model: "Quest_2".to_string(),
                device: "hollywood".to_string(),
                transport_id: 6,
            },
            DeviceLong {
                identifier: "1WMHH810H12000".to_string(),
                state: DeviceState::Device,
                usb: "2-3.2".to_string(),
                product: "hollywood".to_string(),
                model: "Quest_2".to_string(),
                device: "hollywood".to_string(),
                transport_id: 2,
            },
        ];
        for i in 0..inputs.len() {
            assert_eq!(devices[i], expected[i]);
        }
    }

    #[test]
    #[ignore]
    fn test_send_recv() {
        // Create random "Reader" in memory
        let mut key = [0u8; 1000];
        rand::thread_rng().fill(&mut key[..]);
        let mut c: Cursor<Vec<u8>> = Cursor::new(key.to_vec());

        let mut device = new_device();

        const TEST_FILENAME: &str = "/data/local/tmp/test_file";
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
    fn multiple_connections() {
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
