pub mod init {

    use crate::idenity::device_identity::Identity;
    use crate::state::device_state::State;
    use std::io::Result as SingleResult;
    use std::path::Path;
    pub struct AxonInit;

    impl AxonInit {
        pub fn fs_exists() -> SingleResult<bool> {
            Ok(Path::new(&crate::PARENT_PATH.to_string()).exists())
        }

        pub fn init_fs() -> SingleResult<()> {
            match Self::fs_exists() {
                Ok(true) => Ok(()),
                Ok(false) => {
                    State::init_state(&String::from(crate::PARENT_PATH), &String::from(crate::STATE_PATH))?;
                    Identity::create_identity(&crate::IDENTITY_PATH.to_string())?;
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
    }
}
