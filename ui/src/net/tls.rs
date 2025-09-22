//! TLS 热重载模块

use rustls::ServerConfig;
use std::sync::Arc;
use tokio::sync::RwLock;

/// TLS 配置热重载器
#[derive(Clone)]
pub struct TlsReloader {
    config: Arc<RwLock<ServerConfig>>,
}

impl TlsReloader {
    pub fn new(initial_config: ServerConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(initial_config)),
        }
    }

    pub fn handle(&self) -> Arc<RwLock<ServerConfig>> {
        self.config.clone()
    }

    pub async fn reload(&self, cert_pem: &[u8], key_pem: &[u8]) -> anyhow::Result<()> {
        // 解析证书和私钥
        let certs = pem_to_certs(cert_pem)?;
        let key = pem_to_key(key_pem)?;
        
        // 创建新的 TLS 配置
        let provider = rustls::crypto::ring::default_provider();
        let new_config = ServerConfig::builder_with_provider(provider.into())
            .with_protocol_versions(&[&rustls::version::TLS13])
            .map_err(|e| anyhow::anyhow!("tls proto: {e}"))?
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| anyhow::anyhow!("tls build: {e}"))?;

        // 更新配置
        let mut config = self.config.write().await;
        *config = new_config;
        
        println!("TLS 配置已热重载");
        Ok(())
    }
}

fn pem_to_certs(pem: &[u8]) -> anyhow::Result<Vec<rustls::pki_types::CertificateDer<'static>>> {
    let mut certs = vec![];
    let mut rest = pem;
    while let Some((item, r)) =
        rustls_pemfile::read_one_from_slice(rest).map_err(|e| anyhow::anyhow!("{e:?}"))?
    {
        rest = r;
        if let rustls_pemfile::Item::X509Certificate(der) = item {
            certs.push(rustls::pki_types::CertificateDer::from(der));
        }
    }
    anyhow::ensure!(!certs.is_empty(), "no certs in pem");
    Ok(certs)
}

fn pem_to_key(pem: &[u8]) -> anyhow::Result<rustls::pki_types::PrivateKeyDer<'static>> {
    let mut rest = pem;
    while let Some((item, r)) =
        rustls_pemfile::read_one_from_slice(rest).map_err(|e| anyhow::anyhow!("{e:?}"))?
    {
        rest = r;
        match item {
            rustls_pemfile::Item::Pkcs8Key(der) => {
                return Ok(rustls::pki_types::PrivateKeyDer::from(
                    rustls::pki_types::PrivatePkcs8KeyDer::from(der)
                ));
            }
            rustls_pemfile::Item::Pkcs1Key(der) => {
                return Ok(rustls::pki_types::PrivateKeyDer::from(
                    rustls::pki_types::PrivatePkcs1KeyDer::from(der)
                ));
            }
            rustls_pemfile::Item::Sec1Key(der) => {
                return Ok(rustls::pki_types::PrivateKeyDer::from(
                    rustls::pki_types::PrivateSec1KeyDer::from(der)
                ));
            }
            _ => {}
        }
    }
    anyhow::bail!("no key in pem")
}
