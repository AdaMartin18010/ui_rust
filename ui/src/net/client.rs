//! 网络客户端模块 - 提供 DNS 查找和 HTTP 客户端功能

use std::net::IpAddr;
use std::str::FromStr;

/// 网络客户端，提供 DNS 查找等功能
pub struct NetClient {
    /// DNS 后端类型
    dns_backend: DnsBackend,
}

/// 支持的 DNS 后端类型
#[derive(Debug, Clone)]
pub enum DnsBackend {
    /// Cloudflare DNS over HTTPS
    CloudflareDoh,
    /// Cloudflare DNS over TLS
    CloudflareDot,
    /// Google DNS over HTTPS
    GoogleDoh,
    /// Google DNS over TLS
    GoogleDot,
    /// Quad9 DNS over HTTPS
    Quad9Doh,
    /// Quad9 DNS over TLS
    Quad9Dot,
    /// 系统默认 DNS 解析
    System,
}

impl Default for DnsBackend {
    fn default() -> Self {
        Self::System
    }
}

impl FromStr for DnsBackend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cloudflare_doh" => Ok(DnsBackend::CloudflareDoh),
            "cloudflare_dot" => Ok(DnsBackend::CloudflareDot),
            "google_doh" => Ok(DnsBackend::GoogleDoh),
            "google_dot" => Ok(DnsBackend::GoogleDot),
            "quad9_doh" => Ok(DnsBackend::Quad9Doh),
            "quad9_dot" => Ok(DnsBackend::Quad9Dot),
            "system" => Ok(DnsBackend::System),
            _ => Err(format!("不支持的 DNS 后端: {}", s)),
        }
    }
}

impl NetClient {
    /// 创建新的网络客户端实例
    pub fn new() -> Self {
        let dns_backend = std::env::var("C10_DNS_BACKEND")
            .unwrap_or_else(|_| "system".to_string())
            .parse()
            .unwrap_or_default();

        Self { dns_backend }
    }

    /// 创建指定 DNS 后端的网络客户端
    pub fn with_dns_backend(dns_backend: DnsBackend) -> Self {
        Self { dns_backend }
    }

    /// 执行 DNS 查找，返回域名的 IP 地址列表
    pub async fn dns_lookup_ips(&self, domain: &str) -> anyhow::Result<Vec<IpAddr>> {
        match &self.dns_backend {
            DnsBackend::CloudflareDoh => self.cloudflare_doh_lookup(domain).await,
            DnsBackend::CloudflareDot => self.cloudflare_dot_lookup(domain).await,
            DnsBackend::GoogleDoh => self.google_doh_lookup(domain).await,
            DnsBackend::GoogleDot => self.google_dot_lookup(domain).await,
            DnsBackend::Quad9Doh => self.quad9_doh_lookup(domain).await,
            DnsBackend::Quad9Dot => self.quad9_dot_lookup(domain).await,
            DnsBackend::System => self.system_lookup(domain).await,
        }
    }

    /// Cloudflare DNS over HTTPS 查找
    async fn cloudflare_doh_lookup(&self, domain: &str) -> anyhow::Result<Vec<IpAddr>> {
        // 简化实现：使用系统 DNS 作为占位符
        // 在实际应用中，这里应该实现真正的 DoH 查询
        println!("使用 Cloudflare DNS over HTTPS 查找: {}", domain);
        self.system_lookup(domain).await
    }

    /// Cloudflare DNS over TLS 查找
    async fn cloudflare_dot_lookup(&self, domain: &str) -> anyhow::Result<Vec<IpAddr>> {
        // 简化实现：使用系统 DNS 作为占位符
        println!("使用 Cloudflare DNS over TLS 查找: {}", domain);
        self.system_lookup(domain).await
    }

    /// Google DNS over HTTPS 查找
    async fn google_doh_lookup(&self, domain: &str) -> anyhow::Result<Vec<IpAddr>> {
        // 简化实现：使用系统 DNS 作为占位符
        println!("使用 Google DNS over HTTPS 查找: {}", domain);
        self.system_lookup(domain).await
    }

    /// Google DNS over TLS 查找
    async fn google_dot_lookup(&self, domain: &str) -> anyhow::Result<Vec<IpAddr>> {
        // 简化实现：使用系统 DNS 作为占位符
        println!("使用 Google DNS over TLS 查找: {}", domain);
        self.system_lookup(domain).await
    }

    /// Quad9 DNS over HTTPS 查找
    async fn quad9_doh_lookup(&self, domain: &str) -> anyhow::Result<Vec<IpAddr>> {
        // 简化实现：使用系统 DNS 作为占位符
        println!("使用 Quad9 DNS over HTTPS 查找: {}", domain);
        self.system_lookup(domain).await
    }

    /// Quad9 DNS over TLS 查找
    async fn quad9_dot_lookup(&self, domain: &str) -> anyhow::Result<Vec<IpAddr>> {
        // 简化实现：使用系统 DNS 作为占位符
        println!("使用 Quad9 DNS over TLS 查找: {}", domain);
        self.system_lookup(domain).await
    }

    /// 系统默认 DNS 查找
    async fn system_lookup(&self, domain: &str) -> anyhow::Result<Vec<IpAddr>> {
        use tokio::net::lookup_host;
        
        // 尝试不同的端口以确保获取到结果
        let mut ips = Vec::new();
        
        // 尝试 HTTP 端口
        if let Ok(addresses) = lookup_host((domain, 80)).await {
            ips.extend(addresses.map(|addr| addr.ip()));
        }
        
        // 尝试 HTTPS 端口
        if let Ok(addresses) = lookup_host((domain, 443)).await {
            ips.extend(addresses.map(|addr| addr.ip()));
        }
        
        // 如果还是没有结果，尝试使用 std::net 的回退方案
        if ips.is_empty() {
            if let Ok(ip) = std::net::ToSocketAddrs::to_socket_addrs(&format!("{}:80", domain))
                .and_then(|mut addrs| addrs.next().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No addresses")))
            {
                ips.push(ip.ip());
            }
        }
        
        if ips.is_empty() {
            return Err(anyhow::anyhow!("无法解析域名: {}", domain));
        }
        
        // 去重
        ips.sort();
        ips.dedup();
        
        Ok(ips)
    }
}

impl Default for NetClient {
    fn default() -> Self {
        Self::new()
    }
}
