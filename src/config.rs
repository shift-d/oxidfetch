// TODO(thiserror): proper handling
use std::convert::From;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    logo: Logo,
    components: Vec<Component>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Logo {
    Os,
    Custom(String),
    Disabled,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Component {
    name: String,
    icon: Option<String>,
    content: String,
}

type MsgPack = Vec<u8>;

impl From<MsgPack> for Config {
    fn from(buf: MsgPack) -> Self {
        rmp_serde::from_read_ref::<&[u8], Self>(&&buf[..]).unwrap()
    }
}

impl From<Config> for MsgPack {
    fn from(config: Config) -> Self {
        rmp_serde::to_vec(&config).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn msgpack_to_config() {
        let comp = Component {
            name: "OS".into(),
            icon: Some("!".into()),
            content: "Some OS".into(),
        };

        let expected_cfg = Config {
            logo: Logo::Disabled,
            components: vec![comp],
        };

        let buf: MsgPack = vec![
            0x92, 0x81, 0x2, 0xc0, 0x91, 0x93, 0xa2, 0x4f, 0x53, 0xa1, 0x21, 0xa7, 0x53,
            0x6f, 0x6d, 0x65, 0x20, 0x4f, 0x53,
        ];
        let cfg: Config = buf.into();

        assert_eq!(cfg, expected_cfg);
    }

    #[test]
    fn msgpack_from_config() {
        let comp = Component {
            name: "OS".into(),
            icon: Some("!".into()),
            content: "Some OS".into(),
        };

        let cfg = Config {
            logo: Logo::Disabled,
            components: vec![comp],
        };

        let buf: MsgPack = cfg.into();
        let expected_buf: MsgPack = vec![
            0x92, 0x81, 0x2, 0xc0, 0x91, 0x93, 0xa2, 0x4f, 0x53, 0xa1, 0x21, 0xa7, 0x53,
            0x6f, 0x6d, 0x65, 0x20, 0x4f, 0x53,
        ];

        assert_eq!(buf, expected_buf);
    }
}