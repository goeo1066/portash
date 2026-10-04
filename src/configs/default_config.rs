use crate::configs::{PortashConfig, PortashConfigGeneral};

pub fn load_default() -> PortashConfig {
    PortashConfig {
        config_general: PortashConfigGeneral {
            prompt_header: "Portash>".into(),
            chained_header: "-------|".into(),
            msg_on_not_allowed_command: "No such file or directory".into(),
            msg_on_login: "".into(),
        },
        config_command: vec![],
    }
}
