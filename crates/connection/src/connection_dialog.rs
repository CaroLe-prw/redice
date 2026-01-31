mod render_general_tab;

use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
    IndexPath, StyledExt, h_flex,
    input::InputState,
    scroll::ScrollableElement,
    select::{SearchableVec, SelectState},
    v_flex,
};
use theme::ActiveTheme;

/// node address
#[derive(Clone)]
pub(super) struct NodeAddress {
    pub(super) host: Entity<InputState>,
    pub(super) port: Entity<InputState>,
}

/// Dialog view for creating or editing connections
pub struct ConnectionDialog {
    pub active_tab: usize,
    pub config: ConnectionConfig,
}

pub(super) struct ConnectionConfig {
    // ========== General Configuration ==========
    pub(super) general: GeneralConfig,
    // ========== Advanced Configuration ==========
    pub(super) advanced: Option<AdvancedConfig>,
    // ========== Database Aliases ==========
    pub(super) db_aliases: Option<DbAliasesConfig>,
    // ========== SSL/TLS ==========
    pub(super) ssl: Option<SslConfig>,
    // ========== SSH Tunnel ==========
    pub(super) ssh: Option<SshConfig>,
    // ========== Sentinel Mode ==========
    pub(super) sentinel: Option<SentinelConfig>,
    // ========== Cluster Mode ==========
    pub(super) cluster: Option<ClusterConfig>,
    // ========== Proxy Settings ==========
    pub(super) proxy: Option<ProxyConfig>,
}

pub(super) struct GeneralConfig {
    /// Connection name
    pub(super) name: Entity<InputState>,
    /// Group name
    pub(super) group: Entity<SelectState<SearchableVec<&'static str>>>,
    /// Protocol type (TCP / Unix Socket)
    pub(super) protocol: Entity<SelectState<SearchableVec<&'static str>>>,
    /// Host address
    pub(super) host: Entity<InputState>,
    /// Port number
    pub(super) port: Entity<InputState>,
    /// Password
    pub(super) password: Entity<InputState>,
    /// Username
    pub(super) username: Entity<InputState>,
}

pub(super) struct AdvancedConfig {
    /// Default database index
    pub(super) default_db: Entity<SelectState<SearchableVec<&'static str>>>,
    /// Connection timeout (seconds)
    pub(super) connect_timeout: Entity<InputState>,
    /// Execution timeout (seconds)
    pub(super) exec_timeout: Entity<InputState>,
    /// Key separator
    pub(super) key_separator: Entity<InputState>,
    /// Number of keys to load per scan
    pub(super) scan_count: Entity<InputState>,
    /// Enable read-only mode
    pub(super) readonly_mode: bool,
    /// Enable automatic reconnection
    pub(super) auto_reconnect: bool,
    /// Connect automatically on startup
    pub(super) auto_connect: bool,
}

pub(super) struct DbAliasesConfig {
    /// Aliases for db0 to db15
    pub(super) db_aliases: [Entity<InputState>; 16],
}

pub(super) struct SslConfig {
    /// Enable SSL/TLS encryption
    pub(super) ssl_enabled: bool,
    /// Path to CA certificate
    pub(super) ssl_ca_cert: Entity<InputState>,
    /// Path to client certificate
    pub(super) ssl_client_cert: Entity<InputState>,
    /// Path to client private key
    pub(super) ssl_client_key: Entity<InputState>,
    /// Server Name Indication (SNI)
    pub(super) ssl_sni: Entity<InputState>,
    /// Skip certificate verification (Insecure)
    pub(super) ssl_skip_verify: bool,
}

pub(super) struct SshConfig {
    /// Enable SSH tunneling
    pub(super) ssh_enabled: bool,
    /// SSH host address
    pub(super) ssh_host: Entity<InputState>,
    /// SSH port
    pub(super) ssh_port: Entity<InputState>,
    /// SSH username
    pub(super) ssh_username: Entity<InputState>,
    /// SSH authentication method (Password / Private Key / Key + Password)
    pub(super) ssh_auth_method: Entity<SelectState<SearchableVec<&'static str>>>,
    /// SSH password
    pub(super) ssh_password: Entity<InputState>,
    /// Path to SSH private key file
    pub(super) ssh_key_file: Entity<InputState>,
    /// Passphrase for SSH private key
    pub(super) ssh_key_passphrase: Entity<InputState>,
}

pub(super) struct SentinelConfig {
    /// Enable Redis Sentinel mode
    pub(super) sentinel_enabled: bool,
    /// Sentinel master name
    pub(super) sentinel_master: Entity<InputState>,
    /// Sentinel password
    pub(super) sentinel_password: Entity<InputState>,
    /// List of Sentinel nodes
    pub(super) sentinel_nodes: Vec<NodeAddress>,
}

pub(super) struct ClusterConfig {
    /// Enable Redis Cluster mode
    pub(super) cluster_enabled: bool,
    /// List of cluster nodes
    pub(super) cluster_nodes: Vec<NodeAddress>,
    /// Enable read-only access to replicas
    pub(super) cluster_readonly_replica: bool,
}

pub(super) struct ProxyConfig {
    /// Enable network proxy
    pub(super) proxy_enabled: bool,
    /// Proxy type (SOCKS5 / SOCKS4 / HTTP)
    pub(super) proxy_type: Entity<SelectState<SearchableVec<&'static str>>>,
    /// Proxy host address
    pub(super) proxy_host: Entity<InputState>,
    /// Proxy port
    pub(super) proxy_port: Entity<InputState>,
    /// Proxy username
    pub(super) proxy_username: Entity<InputState>,
    /// Proxy password
    pub(super) proxy_password: Entity<InputState>,
}

impl ConnectionDialog {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            active_tab: 0,
            config: ConnectionConfig::new(window, cx),
        }
    }
}

impl ConnectionConfig {
    pub(super) fn new(window: &mut Window, cx: &mut Context<ConnectionDialog>) -> Self {
        Self {
            general: GeneralConfig::new(window, cx),
            advanced: None,
            db_aliases: None,
            ssl: None,
            ssh: None,
            sentinel: None,
            cluster: None,
            proxy: None,
        }
    }

    pub(super) fn ensure_for_tab(
        &mut self,
        tab: usize,
        window: &mut Window,
        cx: &mut Context<ConnectionDialog>,
    ) {
        match tab {
            0 => {}
            1 => {
                self.advanced
                    .get_or_insert_with(|| AdvancedConfig::new(window, cx));
            }
            2 => {
                self.db_aliases
                    .get_or_insert_with(|| DbAliasesConfig::new(window, cx));
            }
            3 => {
                self.ssl.get_or_insert_with(|| SslConfig::new(window, cx));
            }
            4 => {
                self.ssh.get_or_insert_with(|| SshConfig::new(window, cx));
            }
            5 => {
                self.sentinel
                    .get_or_insert_with(|| SentinelConfig::new(window, cx));
            }
            6 => {
                self.cluster
                    .get_or_insert_with(|| ClusterConfig::new(window, cx));
            }
            7 => {
                self.proxy
                    .get_or_insert_with(|| ProxyConfig::new(window, cx));
            }
            _ => {}
        }
    }
}

impl GeneralConfig {
    fn new(window: &mut Window, cx: &mut Context<ConnectionDialog>) -> Self {
        let groups = SearchableVec::new(vec!["无分组"]);
        let protocols = SearchableVec::new(vec!["TCP", "Unix Socket"]);

        Self {
            name: cx.new(|cx| InputState::new(window, cx).placeholder("连接名")),
            group: cx.new(|cx| SelectState::new(groups, Some(IndexPath::default()), window, cx)),
            protocol: cx
                .new(|cx| SelectState::new(protocols, Some(IndexPath::default()), window, cx)),
            host: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("127.0.0.1")
                    .default_value("127.0.0.1")
            }),
            port: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("6379")
                    .default_value("6379")
            }),
            password: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("(可选) Redis服务授权密码")
                    .masked(true)
            }),
            username: cx
                .new(|cx| InputState::new(window, cx).placeholder("(可选) Redis服务授权用户名")),
        }
    }
}

impl AdvancedConfig {
    fn new(window: &mut Window, cx: &mut Context<ConnectionDialog>) -> Self {
        let db_options = SearchableVec::new(vec![
            "db0", "db1", "db2", "db3", "db4", "db5", "db6", "db7", "db8", "db9", "db10", "db11",
            "db12", "db13", "db14", "db15",
        ]);

        Self {
            default_db: cx
                .new(|cx| SelectState::new(db_options, Some(IndexPath::default()), window, cx)),
            connect_timeout: cx.new(|cx| InputState::new(window, cx).default_value("10")),
            exec_timeout: cx.new(|cx| InputState::new(window, cx).default_value("60")),
            key_separator: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder(":")
                    .default_value(":")
            }),
            scan_count: cx.new(|cx| InputState::new(window, cx).default_value("500")),
            readonly_mode: false,
            auto_reconnect: true,
            auto_connect: false,
        }
    }
}

impl DbAliasesConfig {
    fn new(window: &mut Window, cx: &mut Context<ConnectionDialog>) -> Self {
        let db_aliases: [Entity<InputState>; 16] =
            std::array::from_fn(|_| cx.new(|cx| InputState::new(window, cx).placeholder("Alias")));

        Self { db_aliases }
    }
}

impl SslConfig {
    fn new(window: &mut Window, cx: &mut Context<ConnectionDialog>) -> Self {
        Self {
            ssl_enabled: false,
            ssl_ca_cert: cx
                .new(|cx| InputState::new(window, cx).placeholder("(Optional) Path to CA cert")),
            ssl_client_cert: cx.new(|cx| {
                InputState::new(window, cx).placeholder("(Optional) Path to client cert")
            }),
            ssl_client_key: cx
                .new(|cx| InputState::new(window, cx).placeholder("(Optional) Path to client key")),
            ssl_sni: cx.new(|cx| {
                InputState::new(window, cx).placeholder("(Optional) TLS SNI server name")
            }),
            ssl_skip_verify: false,
        }
    }
}

impl SshConfig {
    fn new(window: &mut Window, cx: &mut Context<ConnectionDialog>) -> Self {
        let ssh_auth_methods =
            SearchableVec::new(vec!["Password", "Private Key", "Private Key + Password"]);

        Self {
            ssh_enabled: false,
            ssh_host: cx.new(|cx| InputState::new(window, cx).placeholder("SSH host address")),
            ssh_port: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("22")
                    .default_value("22")
            }),
            ssh_username: cx.new(|cx| InputState::new(window, cx).placeholder("SSH username")),
            ssh_auth_method: cx.new(|cx| {
                SelectState::new(ssh_auth_methods, Some(IndexPath::default()), window, cx)
            }),
            ssh_password: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("SSH password")
                    .masked(true)
            }),
            ssh_key_file: cx.new(|cx| {
                InputState::new(window, cx).placeholder("(Optional) Path to SSH private key")
            }),
            ssh_key_passphrase: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("(Optional) Private key passphrase")
                    .masked(true)
            }),
        }
    }
}

impl SentinelConfig {
    fn new(window: &mut Window, cx: &mut Context<ConnectionDialog>) -> Self {
        Self {
            sentinel_enabled: false,
            sentinel_master: cx.new(|cx| InputState::new(window, cx).placeholder("mymaster")),
            sentinel_password: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("(Optional) Sentinel password")
                    .masked(true)
            }),
            sentinel_nodes: vec![],
        }
    }
}

impl ClusterConfig {
    fn new(_window: &mut Window, _cx: &mut Context<ConnectionDialog>) -> Self {
        Self {
            cluster_enabled: false,
            cluster_nodes: vec![],
            cluster_readonly_replica: true,
        }
    }
}

impl ProxyConfig {
    fn new(window: &mut Window, cx: &mut Context<ConnectionDialog>) -> Self {
        let proxy_types = SearchableVec::new(vec!["SOCKS5", "SOCKS4", "HTTP"]);

        Self {
            proxy_enabled: false,
            proxy_type: cx
                .new(|cx| SelectState::new(proxy_types, Some(IndexPath::default()), window, cx)),
            proxy_host: cx.new(|cx| InputState::new(window, cx).placeholder("127.0.0.1")),
            proxy_port: cx.new(|cx| InputState::new(window, cx).placeholder("1080")),
            proxy_username: cx
                .new(|cx| InputState::new(window, cx).placeholder("(Optional) Proxy username")),
            proxy_password: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("(Optional) Proxy password")
                    .masked(true)
            }),
        }
    }
}

impl Render for ConnectionDialog {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let border_color = cx.theme().border;
        v_flex()
            .size_full()
            .mt_6()
            // top tabs
            .child(self.render_top_tabs(cx))
            // content area with top border
            .child(
                div()
                    .flex_1()
                    .border_t_1()
                    .border_b_1()
                    .border_color(border_color)
                    .child(self.render_tab_content(window, cx)),
            )
    }
}

impl ConnectionDialog {
    fn render_top_tabs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let tabs = [
            "常规配置",
            "高级配置",
            "数据库别名",
            "SSL/TLS",
            "SSH隧道",
            "哨兵模式",
            "集群模式",
            "网络代理",
        ];

        h_flex()
            .px_6()
            .gap_6()
            .children(tabs.iter().enumerate().map(|(index, label)| {
                let is_active = self.active_tab == index;
                div()
                    .id(SharedString::from(format!("tab-{}", index)))
                    .py_2()
                    .text_sm()
                    .cursor_pointer()
                    .border_b_2()
                    .when(is_active, |el| {
                        el.border_color(theme.danger)
                            .text_color(theme.danger)
                            .font_medium()
                    })
                    .when(!is_active, |el| {
                        el.border_color(gpui::transparent_black())
                            .text_color(theme.muted_foreground)
                    })
                    .hover(|el| el.text_color(theme.foreground))
                    .child(*label)
                    .on_click(cx.listener(move |view, _, window, cx| {
                        view.active_tab = index;
                        view.config.ensure_for_tab(index, window, cx);
                        cx.notify();
                    }))
            }))
    }

    fn render_tab_content(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_6()
            .overflow_y_scrollbar()
            .child(match self.active_tab {
                0 => self.render_general_tab(window, cx).into_any_element(),
                _ => div().into_any_element(),
            })
    }
}
