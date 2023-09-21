use std::vec::Vec;

pub struct Provider {
    pub name: String,
    pub endpoints: Vec<ProviderEndpoint>,
}

pub struct ProviderEndpoint {
    pub url: String,
    pub auth_token: Option<String>,
}

fn provider(name: &str, endpoints: Vec<ProviderEndpoint>) -> Provider {
    Provider {
        name: name.to_string(),
        endpoints,
    }
}

fn endpoint(url: &str, auth_token: Option<&str>) -> ProviderEndpoint {
    ProviderEndpoint {
        url: url.to_string(),
        auth_token: auth_token.map(|s| s.to_string()),
    }
}

pub fn get_provider(name: &str) -> Option<Provider> {
    get_providers().into_iter().find(|p| p.name == name)
}

fn get_providers() -> Vec<Provider> {
    vec![
        provider(
            "xdai_mainnet",
            vec![
                endpoint("https://primary.gnosis-chain.rpc.hoprtech.net", None),
                endpoint("https://secondary.gnosis-chain.rpc.hoprtech.net", None),
            ],
        ),
        provider(
            "gnosis",
            vec![
                endpoint(
                    "https://rpc.eu-central-2.gateway.fm/v4/gnosis/non-archival/mainnet",
                    Some("WlpVWYtSXZkKShofH6G6zjk7ydfWEsuz.fZupbyX3PHu6OD0g"),
                ),
                endpoint("https://primary.gnosis-chain.rpc.hoprtech.net", None),
                endpoint("https://secondary.gnosis-chain.rpc.hoprtech.net", None),
            ],
        ),
        provider(
            "gnosis-ext",
            vec![endpoint(
                "https://rpc.eu-central-2.gateway.fm/v4/gnosis/non-archival/mainnet",
                Some("WlpVWYtSXZkKShofH6G6zjk7ydfWEsuz.fZupbyX3PHu6OD0g"),
            )],
        ),
    ]
}
