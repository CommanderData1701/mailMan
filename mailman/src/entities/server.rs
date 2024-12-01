pub struct ImapServer {
    hostname: String,
    port: u32,
    connection_security: ConnectionSecurity,
    authentication_method: AuthenticationMethod,
}

impl ImapServer {
    pub fn get_hostname(&self) -> String {
        self.hostname
    }
}

pub struct ImapServerBuilder {
    hostname: Option<String>,
    authentication_method: AuthenticationMethod,
    connection_security: ConnectionSecurity,
    port: Option<u32>,
}

impl Default for ImapServerBuilder {
    fn default() -> Self {
        ImapServerBuilder{
            hostname: None,
            authentication_method: AuthenticationMethod::NormalPassword,
            connection_security: ConnectionSecurity::NoSecure,
            port: None
        }
    }
}

impl ImapServerBuilder {
    pub fn build(&self) -> Result<ImapServer, String> {
        Ok (ImapServer {
            hostname: match self.hostname {
                None => return Err("Hostname not set.".to_string()),
                Some(hostname) => hostname
            },
            authentication_method: self.authentication_method,
            connection_security: self.connection_security,
            port: match self.port {
                None => self.connection_security.get_default_port(),
                Some(port) => port
            }
        })
    }

    pub fn hostname(&mut self, hostname: String) -> &mut Self {
        self.hostname = Some(hostname);
        self
    }

    pub fn authentication_method(&mut self, auth_meth: AuthenticationMethod)
        -> &mut Self {
        self.authentication_method = auth_meth;
        self
    }

    pub fn connection_security(&mut self, conn_sec: ConnectionSecurity)
        -> &mut Self {
        self.connection_security = conn_sec;
        self
    }

    pub fn port(&mut self, port: u32) -> &mut Self {
        self.port = Some(port);
        self
    }
}

pub enum ConnectionSecurity {
    NoSecure,
    SSLTLSS
}

impl ConnectionSecurity {
    fn get_default_port(&self) -> u32 {
        match &self {
            Self::NoSecure => 143,
            Self::SSLTLSS => 993,
        }
    }
}

pub enum AuthenticationMethod {
    NormalPassword,
}
