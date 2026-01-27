-- ============================================================================
-- 连接分组表
-- ============================================================================
CREATE TABLE IF NOT EXISTS connection_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,          -- 主键ID
    name TEXT NOT NULL UNIQUE,                     -- 分组名称（唯一）
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))  -- 创建时间（Unix时间戳）
);

-- ============================================================================
-- 主连接配置表
-- ============================================================================
CREATE TABLE IF NOT EXISTS connections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,          -- 主键ID
    name TEXT NOT NULL,                            -- 连接名称
    group_id INTEGER REFERENCES connection_groups(id) ON DELETE SET NULL,  -- 所属分组ID

    -- ========== 常规配置 ==========
    connection_type TEXT NOT NULL DEFAULT 'tcp' CHECK (connection_type IN ('tcp', 'unix')),  -- 连接类型：tcp/unix
    host TEXT NOT NULL DEFAULT '127.0.0.1',        -- 主机地址
    port INTEGER NOT NULL DEFAULT 6379,            -- 端口号
    username TEXT,                                 -- Redis 用户名（可选）
    password TEXT,                                 -- Redis 密码（可选）

    -- ========== 高级配置 ==========
    default_db INTEGER NOT NULL DEFAULT 0,         -- 默认数据库索引 (0-15)
    connection_timeout INTEGER NOT NULL DEFAULT 10,   -- 连接超时（秒）
    execution_timeout INTEGER NOT NULL DEFAULT 60,    -- 执行超时（秒）
    key_separator TEXT NOT NULL DEFAULT ':',       -- 键分隔符（用于键浏览器分组）
    scan_count INTEGER NOT NULL DEFAULT 500,       -- SCAN 每次迭代返回的键数量
    readonly_mode INTEGER NOT NULL DEFAULT 0,      -- 只读模式：0=关闭, 1=启用
    auto_reconnect INTEGER NOT NULL DEFAULT 1,     -- 自动重连：0=关闭, 1=启用
    auto_connect_on_startup INTEGER NOT NULL DEFAULT 0,  -- 启动时自动连接：0=关闭, 1=启用

    -- ========== SSL/TLS 配置 ==========
    ssl_enabled INTEGER NOT NULL DEFAULT 0,        -- 启用 SSL/TLS：0=关闭, 1=启用
    ssl_ca_cert_path TEXT,                         -- CA 证书文件路径
    ssl_client_cert_path TEXT,                     -- 客户端证书文件路径
    ssl_client_key_path TEXT,                      -- 客户端私钥文件路径
    ssl_sni_server_name TEXT,                      -- TLS SNI 服务器名称
    ssl_skip_verify INTEGER NOT NULL DEFAULT 0,    -- 跳过证书验证：0=关闭, 1=启用（仅测试环境）

    -- ========== SSH 隧道配置 ==========
    ssh_enabled INTEGER NOT NULL DEFAULT 0,        -- 启用 SSH 隧道：0=关闭, 1=启用
    ssh_host TEXT,                                 -- SSH 服务器地址
    ssh_port INTEGER DEFAULT 22,                   -- SSH 端口号
    ssh_username TEXT,                             -- SSH 用户名
    ssh_auth_method TEXT DEFAULT 'password' CHECK (ssh_auth_method IN ('password', 'key', 'key_password')),  -- 认证方式：password/key/key_password
    ssh_password TEXT,                             -- SSH 密码
    ssh_private_key_path TEXT,                     -- SSH 私钥文件路径
    ssh_private_key_passphrase TEXT,               -- SSH 私钥密码

    -- ========== 哨兵模式配置 ==========
    sentinel_enabled INTEGER NOT NULL DEFAULT 0,   -- 启用哨兵模式：0=关闭, 1=启用
    sentinel_master_name TEXT,                     -- Master 名称
    sentinel_password TEXT,                        -- Sentinel 认证密码

    -- ========== 集群模式配置 ==========
    cluster_enabled INTEGER NOT NULL DEFAULT 0,    -- 启用集群模式：0=关闭, 1=启用
    cluster_readonly_replicas INTEGER NOT NULL DEFAULT 1,  -- 允许从只读副本读取：0=关闭, 1=启用

    -- ========== 网络代理配置 ==========
    proxy_enabled INTEGER NOT NULL DEFAULT 0,      -- 启用网络代理：0=关闭, 1=启用
    proxy_type TEXT DEFAULT 'socks5' CHECK (proxy_type IN ('socks5', 'socks4', 'http')),  -- 代理类型
    proxy_host TEXT,                               -- 代理服务器地址
    proxy_port INTEGER,                            -- 代理端口号
    proxy_username TEXT,                           -- 代理用户名（可选）
    proxy_password TEXT,                           -- 代理密码（可选）

    -- ========== 时间戳 ==========
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),  -- 创建时间（Unix时间戳）
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))   -- 更新时间（Unix时间戳）
);

-- ============================================================================
-- 数据库别名表 (db0-db15)
-- ============================================================================
CREATE TABLE IF NOT EXISTS db_aliases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,          -- 主键ID
    connection_id INTEGER NOT NULL REFERENCES connections(id) ON DELETE CASCADE,  -- 关联的连接ID
    db_index INTEGER NOT NULL CHECK (db_index >= 0 AND db_index <= 15),  -- 数据库索引 (0-15)
    alias TEXT NOT NULL,                           -- 数据库别名
    UNIQUE (connection_id, db_index)               -- 每个连接的每个数据库只能有一个别名
);

-- ============================================================================
-- 哨兵节点表
-- ============================================================================
CREATE TABLE IF NOT EXISTS sentinel_nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,          -- 主键ID
    connection_id INTEGER NOT NULL REFERENCES connections(id) ON DELETE CASCADE,  -- 关联的连接ID
    host TEXT NOT NULL,                            -- 节点主机地址
    port INTEGER NOT NULL DEFAULT 26379            -- 节点端口号
);

-- ============================================================================
-- 集群节点表
-- ============================================================================
CREATE TABLE IF NOT EXISTS cluster_nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,          -- 主键ID
    connection_id INTEGER NOT NULL REFERENCES connections(id) ON DELETE CASCADE,  -- 关联的连接ID
    host TEXT NOT NULL,                            -- 节点主机地址
    port INTEGER NOT NULL DEFAULT 7000             -- 节点端口号
);

-- ============================================================================
-- 索引
-- ============================================================================
CREATE INDEX IF NOT EXISTS idx_connections_group_id ON connections(group_id);  -- 按分组查询连接
CREATE INDEX IF NOT EXISTS idx_db_aliases_connection_id ON db_aliases(connection_id);  -- 按连接查询别名
CREATE INDEX IF NOT EXISTS idx_sentinel_nodes_connection_id ON sentinel_nodes(connection_id);  -- 按连接查询哨兵节点
CREATE INDEX IF NOT EXISTS idx_cluster_nodes_connection_id ON cluster_nodes(connection_id);  -- 按连接查询集群节点
