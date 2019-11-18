use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct KubernetesConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub context: SegmentConfig<'a>,
    pub namespace: SegmentConfig<'a>,
    pub style: Style,
    pub display: Vec<KubernetesDisplayConfig<'a>>,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for KubernetesConfig<'a> {
    fn new() -> Self {
        KubernetesConfig {
            symbol: SegmentConfig::new("☸ "),
            context: SegmentConfig::default(),
            namespace: SegmentConfig::default(),
            style: Color::Cyan.bold(),
            display: vec![KubernetesDisplayConfig {
                name: "prod",
                style: Color::Red.bold(),
            }],
            disabled: true,
        }
    }
}

#[derive(Clone, ModuleConfig)]
pub struct KubernetesDisplayConfig<'a> {
    pub name: &'a str,
    pub style: Style,
}
