use std::collections::HashMap;
use std::sync::OnceLock;

pub struct RegistryInfo {
    pub home: &'static str,
    pub registry: &'static str,
}

pub static DEFAULT_REGISTRIES: OnceLock<HashMap<String, String>> = OnceLock::new();
pub static DEFAULT_REGISTRIES_INFO: OnceLock<HashMap<String, RegistryInfo>> = OnceLock::new();

pub fn get_default_registries_info() -> &'static HashMap<String, RegistryInfo> {
    DEFAULT_REGISTRIES_INFO.get_or_init(|| {
        let mut registries: HashMap<String, RegistryInfo> = HashMap::new();
        // Default registries config here
        registries.insert(
            "npm".to_string(),
            RegistryInfo {
                home: "https://www.npmjs.org",
                registry: "https://registry.npmjs.org/",
            },
        );
        registries.insert(
            "yarn".to_string(),
            RegistryInfo {
                home: "https://yarnpkg.com",
                registry: "https://registry.yarnpkg.com/",
            },
        );
        registries.insert(
            "tencent".to_string(),
            RegistryInfo {
                home: "https://mirrors.clound.tencent.com/npm/",
                registry: "https://mirrors.clound.tencent.com/npm/",
            },
        );
        registries.insert(
            "cnpm".to_string(),
            RegistryInfo {
                home: "https://cnpmjs.org",
                registry: "https://r.cnpmjs.org/",
            },
        );
        registries.insert(
            "taobao".to_string(),
            RegistryInfo {
                home: "https://npmmirror.com",
                registry: "https://registry.npmmirror.com/",
            },
        );
        registries.insert(
            "npmMirror".to_string(),
            RegistryInfo {
                home: "https://skimdb.npmjs.com/",
                registry: "https://skimdb.npmjs.com/registry/",
            },
        );
        registries
    })
}

pub fn get_default_registries() -> &'static HashMap<String, String> {
    DEFAULT_REGISTRIES.get_or_init(|| {
        get_default_registries_info()
            .iter()
            .map(
                |(
                    key,
                    RegistryInfo {
                        home: _home,
                        registry,
                    },
                )| (key.clone(), registry.to_string()),
            )
            .collect()
    })
}
