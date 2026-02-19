#[cfg(test)]
mod tests {
    use ftservo_sdk::group_sync_write::GroupSyncWrite;

    #[test]
    fn test_group_sync_write_new() {
        let gsw = GroupSyncWrite::new(42, 6);
        assert_eq!(gsw.start_address, 42);
    }

    #[test]
    fn test_add_param() {
        let mut gsw = GroupSyncWrite::new(42, 6);

        let data = vec![1, 2, 3, 4, 5, 6];
        assert!(gsw.add_param(1, data.clone()).is_ok());

        // 尝试添加重复 ID 应该失败
        assert!(gsw.add_param(1, data).is_err());
    }

    #[test]
    fn test_remove_param() {
        let mut gsw = GroupSyncWrite::new(42, 6);

        let data = vec![1, 2, 3, 4, 5, 6];
        gsw.add_param(1, data).unwrap();

        assert!(gsw.remove_param(1).is_ok());
        assert!(gsw.remove_param(1).is_err()); // 再次移除应该失败
    }

    #[test]
    fn test_clear_param() {
        let mut gsw = GroupSyncWrite::new(42, 6);

        gsw.add_param(1, vec![1, 2, 3, 4, 5, 6]).unwrap();
        gsw.add_param(2, vec![7, 8, 9, 10, 11, 12]).unwrap();

        gsw.clear_param();

        // 清除后应该无法移除任何参数
        assert!(gsw.remove_param(1).is_err());
    }

    #[test]
    fn test_change_param() {
        let mut gsw = GroupSyncWrite::new(42, 6);

        gsw.add_param(1, vec![1, 2, 3, 4, 5, 6]).unwrap();

        let new_data = vec![10, 20, 30, 40, 50, 60];
        assert!(gsw.change_param(1, new_data).is_ok());

        // 尝试更改不存在的 ID 应该失败
        assert!(gsw.change_param(2, vec![1, 2, 3, 4, 5, 6]).is_err());
    }
}
