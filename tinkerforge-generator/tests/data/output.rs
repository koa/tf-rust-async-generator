pub mod master {
    #[allow(unused_imports)]
    use tinkerforge_base::byte_converter::{FromByteSlice, ToBytes};
    #[allow(unused_imports)]
    use tokio_stream::StreamExt;
    #[allow(unused_imports)]
    use std::convert::TryInto;
    #[derive(Clone, Debug)]
    pub struct MasterBrick {
        device: tinkerforge_base::device::Device,
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum ExtensionType {
        Chibi,
        Rs485,
        Wifi,
        Ethernet,
        Wifi2,
    }
    impl Into<u32> for ExtensionType {
        fn into(self) -> u32 {
            match self {
                ExtensionType::Chibi => 1u32,
                ExtensionType::Rs485 => 2u32,
                ExtensionType::Wifi => 3u32,
                ExtensionType::Ethernet => 4u32,
                ExtensionType::Wifi2 => 5u32,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for ExtensionType {
        fn write_to_slice(self, target: &mut [u8]) {
            <ExtensionType as Into<u32>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for ExtensionType {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u32::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            4usize
        }
    }
    impl std::convert::TryInto<ExtensionType> for u32 {
        type Error = ();
        fn try_into(self) -> Result<ExtensionType, Self::Error> {
            match self {
                1u32 => Ok(ExtensionType::Chibi),
                2u32 => Ok(ExtensionType::Rs485),
                3u32 => Ok(ExtensionType::Wifi),
                4u32 => Ok(ExtensionType::Ethernet),
                5u32 => Ok(ExtensionType::Wifi2),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetExtensionTypeRequest {
        pub extension: u8,
        pub exttype: crate::bindings::master::ExtensionType,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for SetExtensionTypeRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let extension = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let exttype = crate::bindings::master::ExtensionType::from_le_byte_slice(
                &bytes[1usize..5usize],
            );
            Self { extension, exttype }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetExtensionTypeRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.extension.write_to_slice(&mut target[0usize..1usize]);
            self.exttype.write_to_slice(&mut target[1usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetChibiSlaveAddressRequest {
        pub num: u8,
        pub address: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetChibiSlaveAddressRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let num = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let address = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            Self { num, address }
        }
        fn bytes_expected() -> usize {
            2usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetChibiSlaveAddressRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.num.write_to_slice(&mut target[0usize..1usize]);
            self.address.write_to_slice(&mut target[1usize..2usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetChibiErrorLogResponse {
        pub underrun: u16,
        pub crc_error: u16,
        pub no_ack: u16,
        pub overflow: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetChibiErrorLogResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let underrun = u16::from_le_byte_slice(&bytes[0usize..2usize]);
            let crc_error = u16::from_le_byte_slice(&bytes[2usize..4usize]);
            let no_ack = u16::from_le_byte_slice(&bytes[4usize..6usize]);
            let overflow = u16::from_le_byte_slice(&bytes[6usize..8usize]);
            Self {
                underrun,
                crc_error,
                no_ack,
                overflow,
            }
        }
        fn bytes_expected() -> usize {
            8usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetChibiErrorLogResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.underrun.write_to_slice(&mut target[0usize..2usize]);
            self.crc_error.write_to_slice(&mut target[2usize..4usize]);
            self.no_ack.write_to_slice(&mut target[4usize..6usize]);
            self.overflow.write_to_slice(&mut target[6usize..8usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum ChibiFrequency {
        Oqpsk868MHz,
        Oqpsk915MHz,
        Oqpsk780MHz,
        Bpsk40915MHz,
    }
    impl Into<u8> for ChibiFrequency {
        fn into(self) -> u8 {
            match self {
                ChibiFrequency::Oqpsk868MHz => 0u8,
                ChibiFrequency::Oqpsk915MHz => 1u8,
                ChibiFrequency::Oqpsk780MHz => 2u8,
                ChibiFrequency::Bpsk40915MHz => 3u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for ChibiFrequency {
        fn write_to_slice(self, target: &mut [u8]) {
            <ChibiFrequency as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for ChibiFrequency {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<ChibiFrequency> for u8 {
        type Error = ();
        fn try_into(self) -> Result<ChibiFrequency, Self::Error> {
            match self {
                0u8 => Ok(ChibiFrequency::Oqpsk868MHz),
                1u8 => Ok(ChibiFrequency::Oqpsk915MHz),
                2u8 => Ok(ChibiFrequency::Oqpsk780MHz),
                3u8 => Ok(ChibiFrequency::Bpsk40915MHz),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetRs485SlaveAddressRequest {
        pub num: u8,
        pub address: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetRs485SlaveAddressRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let num = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let address = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            Self { num, address }
        }
        fn bytes_expected() -> usize {
            2usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetRs485SlaveAddressRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.num.write_to_slice(&mut target[0usize..1usize]);
            self.address.write_to_slice(&mut target[1usize..2usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Rs485Parity {
        None,
        Even,
        Odd,
    }
    impl Into<char> for Rs485Parity {
        fn into(self) -> char {
            match self {
                Rs485Parity::None => 'n',
                Rs485Parity::Even => 'e',
                Rs485Parity::Odd => 'o',
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for Rs485Parity {
        fn write_to_slice(self, target: &mut [u8]) {
            <Rs485Parity as Into<char>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for Rs485Parity {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            char::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<Rs485Parity> for char {
        type Error = ();
        fn try_into(self) -> Result<Rs485Parity, Self::Error> {
            match self {
                'n' => Ok(Rs485Parity::None),
                'e' => Ok(Rs485Parity::Even),
                'o' => Ok(Rs485Parity::Odd),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetRs485ConfigurationRequest {
        pub speed: u32,
        pub parity: crate::bindings::master::Rs485Parity,
        pub stopbits: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetRs485ConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let speed = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let parity = crate::bindings::master::Rs485Parity::from_le_byte_slice(
                &bytes[4usize..5usize],
            );
            let stopbits = u8::from_le_byte_slice(&bytes[5usize..6usize]);
            Self { speed, parity, stopbits }
        }
        fn bytes_expected() -> usize {
            6usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetRs485ConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.speed.write_to_slice(&mut target[0usize..4usize]);
            self.parity.write_to_slice(&mut target[4usize..5usize]);
            self.stopbits.write_to_slice(&mut target[5usize..6usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetRs485ConfigurationResponse {
        pub speed: u32,
        pub parity: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::Rs485Parity,
            char,
        >,
        pub stopbits: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetRs485ConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let speed = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let parity = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::Rs485Parity,
                char,
            >::from_le_byte_slice(&bytes[4usize..5usize]);
            let stopbits = u8::from_le_byte_slice(&bytes[5usize..6usize]);
            Self { speed, parity, stopbits }
        }
        fn bytes_expected() -> usize {
            6usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetRs485ConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.speed.write_to_slice(&mut target[0usize..4usize]);
            self.parity.write_to_slice(&mut target[4usize..5usize]);
            self.stopbits.write_to_slice(&mut target[5usize..6usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum WifiConnection {
        Dhcp,
        StaticIp,
        AccessPointDhcp,
        AccessPointStaticIp,
        AdHocDhcp,
        AdHocStaticIp,
    }
    impl Into<u8> for WifiConnection {
        fn into(self) -> u8 {
            match self {
                WifiConnection::Dhcp => 0u8,
                WifiConnection::StaticIp => 1u8,
                WifiConnection::AccessPointDhcp => 2u8,
                WifiConnection::AccessPointStaticIp => 3u8,
                WifiConnection::AdHocDhcp => 4u8,
                WifiConnection::AdHocStaticIp => 5u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for WifiConnection {
        fn write_to_slice(self, target: &mut [u8]) {
            <WifiConnection as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for WifiConnection {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<WifiConnection> for u8 {
        type Error = ();
        fn try_into(self) -> Result<WifiConnection, Self::Error> {
            match self {
                0u8 => Ok(WifiConnection::Dhcp),
                1u8 => Ok(WifiConnection::StaticIp),
                2u8 => Ok(WifiConnection::AccessPointDhcp),
                3u8 => Ok(WifiConnection::AccessPointStaticIp),
                4u8 => Ok(WifiConnection::AdHocDhcp),
                5u8 => Ok(WifiConnection::AdHocStaticIp),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetWifiConfigurationRequest {
        pub ssid: [char; 32usize],
        pub connection: crate::bindings::master::WifiConnection,
        pub ip: [u8; 4usize],
        pub subnet_mask: [u8; 4usize],
        pub gateway: [u8; 4usize],
        pub port: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetWifiConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let ssid = <[char; 32usize]>::from_le_byte_slice(&bytes[0usize..32usize]);
            let connection = crate::bindings::master::WifiConnection::from_le_byte_slice(
                &bytes[32usize..33usize],
            );
            let ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[33usize..37usize]);
            let subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[37usize..41usize],
            );
            let gateway = <[u8; 4usize]>::from_le_byte_slice(&bytes[41usize..45usize]);
            let port = u16::from_le_byte_slice(&bytes[45usize..47usize]);
            Self {
                ssid,
                connection,
                ip,
                subnet_mask,
                gateway,
                port,
            }
        }
        fn bytes_expected() -> usize {
            47usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetWifiConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.ssid.write_to_slice(&mut target[0usize..32usize]);
            self.connection.write_to_slice(&mut target[32usize..33usize]);
            self.ip.write_to_slice(&mut target[33usize..37usize]);
            self.subnet_mask.write_to_slice(&mut target[37usize..41usize]);
            self.gateway.write_to_slice(&mut target[41usize..45usize]);
            self.port.write_to_slice(&mut target[45usize..47usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifiConfigurationResponse {
        pub ssid: [char; 32usize],
        pub connection: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::WifiConnection,
            u8,
        >,
        pub ip: [u8; 4usize],
        pub subnet_mask: [u8; 4usize],
        pub gateway: [u8; 4usize],
        pub port: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetWifiConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let ssid = <[char; 32usize]>::from_le_byte_slice(&bytes[0usize..32usize]);
            let connection = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::WifiConnection,
                u8,
            >::from_le_byte_slice(&bytes[32usize..33usize]);
            let ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[33usize..37usize]);
            let subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[37usize..41usize],
            );
            let gateway = <[u8; 4usize]>::from_le_byte_slice(&bytes[41usize..45usize]);
            let port = u16::from_le_byte_slice(&bytes[45usize..47usize]);
            Self {
                ssid,
                connection,
                ip,
                subnet_mask,
                gateway,
                port,
            }
        }
        fn bytes_expected() -> usize {
            47usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetWifiConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.ssid.write_to_slice(&mut target[0usize..32usize]);
            self.connection.write_to_slice(&mut target[32usize..33usize]);
            self.ip.write_to_slice(&mut target[33usize..37usize]);
            self.subnet_mask.write_to_slice(&mut target[37usize..41usize]);
            self.gateway.write_to_slice(&mut target[41usize..45usize]);
            self.port.write_to_slice(&mut target[45usize..47usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum WifiEncryption {
        WpaWpa2,
        WpaEnterprise,
        Wep,
        NoEncryption,
    }
    impl Into<u8> for WifiEncryption {
        fn into(self) -> u8 {
            match self {
                WifiEncryption::WpaWpa2 => 0u8,
                WifiEncryption::WpaEnterprise => 1u8,
                WifiEncryption::Wep => 2u8,
                WifiEncryption::NoEncryption => 3u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for WifiEncryption {
        fn write_to_slice(self, target: &mut [u8]) {
            <WifiEncryption as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for WifiEncryption {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<WifiEncryption> for u8 {
        type Error = ();
        fn try_into(self) -> Result<WifiEncryption, Self::Error> {
            match self {
                0u8 => Ok(WifiEncryption::WpaWpa2),
                1u8 => Ok(WifiEncryption::WpaEnterprise),
                2u8 => Ok(WifiEncryption::Wep),
                3u8 => Ok(WifiEncryption::NoEncryption),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum WifiEapOption {
        OuterAuthEapFast,
        OuterAuthEapTls,
        OuterAuthEapTtls,
        OuterAuthEapPeap,
        InnerAuthEapMschap,
        InnerAuthEapGtc,
        CertTypeCaCert,
        CertTypeClientCert,
        CertTypePrivateKey,
    }
    impl Into<u8> for WifiEapOption {
        fn into(self) -> u8 {
            match self {
                WifiEapOption::OuterAuthEapFast => 0u8,
                WifiEapOption::OuterAuthEapTls => 1u8,
                WifiEapOption::OuterAuthEapTtls => 2u8,
                WifiEapOption::OuterAuthEapPeap => 3u8,
                WifiEapOption::InnerAuthEapMschap => 0u8,
                WifiEapOption::InnerAuthEapGtc => 4u8,
                WifiEapOption::CertTypeCaCert => 0u8,
                WifiEapOption::CertTypeClientCert => 8u8,
                WifiEapOption::CertTypePrivateKey => 16u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for WifiEapOption {
        fn write_to_slice(self, target: &mut [u8]) {
            <WifiEapOption as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for WifiEapOption {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<WifiEapOption> for u8 {
        type Error = ();
        fn try_into(self) -> Result<WifiEapOption, Self::Error> {
            match self {
                0u8 => Ok(WifiEapOption::OuterAuthEapFast),
                1u8 => Ok(WifiEapOption::OuterAuthEapTls),
                2u8 => Ok(WifiEapOption::OuterAuthEapTtls),
                3u8 => Ok(WifiEapOption::OuterAuthEapPeap),
                0u8 => Ok(WifiEapOption::InnerAuthEapMschap),
                4u8 => Ok(WifiEapOption::InnerAuthEapGtc),
                0u8 => Ok(WifiEapOption::CertTypeCaCert),
                8u8 => Ok(WifiEapOption::CertTypeClientCert),
                16u8 => Ok(WifiEapOption::CertTypePrivateKey),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetWifiEncryptionRequest {
        pub encryption: crate::bindings::master::WifiEncryption,
        pub key: [char; 50usize],
        pub key_index: u8,
        pub eap_options: crate::bindings::master::WifiEapOption,
        pub ca_certificate_length: u16,
        pub client_certificate_length: u16,
        pub private_key_length: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for SetWifiEncryptionRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let encryption = crate::bindings::master::WifiEncryption::from_le_byte_slice(
                &bytes[0usize..1usize],
            );
            let key = <[char; 50usize]>::from_le_byte_slice(&bytes[1usize..51usize]);
            let key_index = u8::from_le_byte_slice(&bytes[51usize..52usize]);
            let eap_options = crate::bindings::master::WifiEapOption::from_le_byte_slice(
                &bytes[52usize..53usize],
            );
            let ca_certificate_length = u16::from_le_byte_slice(
                &bytes[53usize..55usize],
            );
            let client_certificate_length = u16::from_le_byte_slice(
                &bytes[55usize..57usize],
            );
            let private_key_length = u16::from_le_byte_slice(&bytes[57usize..59usize]);
            Self {
                encryption,
                key,
                key_index,
                eap_options,
                ca_certificate_length,
                client_certificate_length,
                private_key_length,
            }
        }
        fn bytes_expected() -> usize {
            59usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetWifiEncryptionRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.encryption.write_to_slice(&mut target[0usize..1usize]);
            self.key.write_to_slice(&mut target[1usize..51usize]);
            self.key_index.write_to_slice(&mut target[51usize..52usize]);
            self.eap_options.write_to_slice(&mut target[52usize..53usize]);
            self.ca_certificate_length.write_to_slice(&mut target[53usize..55usize]);
            self.client_certificate_length.write_to_slice(&mut target[55usize..57usize]);
            self.private_key_length.write_to_slice(&mut target[57usize..59usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifiEncryptionResponse {
        pub encryption: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::WifiEncryption,
            u8,
        >,
        pub key: [char; 50usize],
        pub key_index: u8,
        pub eap_options: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::WifiEapOption,
            u8,
        >,
        pub ca_certificate_length: u16,
        pub client_certificate_length: u16,
        pub private_key_length: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetWifiEncryptionResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let encryption = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::WifiEncryption,
                u8,
            >::from_le_byte_slice(&bytes[0usize..1usize]);
            let key = <[char; 50usize]>::from_le_byte_slice(&bytes[1usize..51usize]);
            let key_index = u8::from_le_byte_slice(&bytes[51usize..52usize]);
            let eap_options = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::WifiEapOption,
                u8,
            >::from_le_byte_slice(&bytes[52usize..53usize]);
            let ca_certificate_length = u16::from_le_byte_slice(
                &bytes[53usize..55usize],
            );
            let client_certificate_length = u16::from_le_byte_slice(
                &bytes[55usize..57usize],
            );
            let private_key_length = u16::from_le_byte_slice(&bytes[57usize..59usize]);
            Self {
                encryption,
                key,
                key_index,
                eap_options,
                ca_certificate_length,
                client_certificate_length,
                private_key_length,
            }
        }
        fn bytes_expected() -> usize {
            59usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetWifiEncryptionResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.encryption.write_to_slice(&mut target[0usize..1usize]);
            self.key.write_to_slice(&mut target[1usize..51usize]);
            self.key_index.write_to_slice(&mut target[51usize..52usize]);
            self.eap_options.write_to_slice(&mut target[52usize..53usize]);
            self.ca_certificate_length.write_to_slice(&mut target[53usize..55usize]);
            self.client_certificate_length.write_to_slice(&mut target[55usize..57usize]);
            self.private_key_length.write_to_slice(&mut target[57usize..59usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum WifiState {
        Disassociated,
        Associated,
        Associating,
        Error,
        NotInitializedYet,
    }
    impl Into<u8> for WifiState {
        fn into(self) -> u8 {
            match self {
                WifiState::Disassociated => 0u8,
                WifiState::Associated => 1u8,
                WifiState::Associating => 2u8,
                WifiState::Error => 3u8,
                WifiState::NotInitializedYet => 255u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for WifiState {
        fn write_to_slice(self, target: &mut [u8]) {
            <WifiState as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for WifiState {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<WifiState> for u8 {
        type Error = ();
        fn try_into(self) -> Result<WifiState, Self::Error> {
            match self {
                0u8 => Ok(WifiState::Disassociated),
                1u8 => Ok(WifiState::Associated),
                2u8 => Ok(WifiState::Associating),
                3u8 => Ok(WifiState::Error),
                255u8 => Ok(WifiState::NotInitializedYet),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifiStatusResponse {
        pub mac_address: [u8; 6usize],
        pub bssid: [u8; 6usize],
        pub channel: u8,
        pub rssi: i16,
        pub ip: [u8; 4usize],
        pub subnet_mask: [u8; 4usize],
        pub gateway: [u8; 4usize],
        pub rx_count: u32,
        pub tx_count: u32,
        pub state: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::WifiState,
            u8,
        >,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetWifiStatusResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let mac_address = <[u8; 6usize]>::from_le_byte_slice(&bytes[0usize..6usize]);
            let bssid = <[u8; 6usize]>::from_le_byte_slice(&bytes[6usize..12usize]);
            let channel = u8::from_le_byte_slice(&bytes[12usize..13usize]);
            let rssi = i16::from_le_byte_slice(&bytes[13usize..15usize]);
            let ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[15usize..19usize]);
            let subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[19usize..23usize],
            );
            let gateway = <[u8; 4usize]>::from_le_byte_slice(&bytes[23usize..27usize]);
            let rx_count = u32::from_le_byte_slice(&bytes[27usize..31usize]);
            let tx_count = u32::from_le_byte_slice(&bytes[31usize..35usize]);
            let state = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::WifiState,
                u8,
            >::from_le_byte_slice(&bytes[35usize..36usize]);
            Self {
                mac_address,
                bssid,
                channel,
                rssi,
                ip,
                subnet_mask,
                gateway,
                rx_count,
                tx_count,
                state,
            }
        }
        fn bytes_expected() -> usize {
            36usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetWifiStatusResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.mac_address.write_to_slice(&mut target[0usize..6usize]);
            self.bssid.write_to_slice(&mut target[6usize..12usize]);
            self.channel.write_to_slice(&mut target[12usize..13usize]);
            self.rssi.write_to_slice(&mut target[13usize..15usize]);
            self.ip.write_to_slice(&mut target[15usize..19usize]);
            self.subnet_mask.write_to_slice(&mut target[19usize..23usize]);
            self.gateway.write_to_slice(&mut target[23usize..27usize]);
            self.rx_count.write_to_slice(&mut target[27usize..31usize]);
            self.tx_count.write_to_slice(&mut target[31usize..35usize]);
            self.state.write_to_slice(&mut target[35usize..36usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetWifiCertificateRequest {
        pub index: u16,
        pub data: [u8; 32usize],
        pub data_length: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for SetWifiCertificateRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let index = u16::from_le_byte_slice(&bytes[0usize..2usize]);
            let data = <[u8; 32usize]>::from_le_byte_slice(&bytes[2usize..34usize]);
            let data_length = u8::from_le_byte_slice(&bytes[34usize..35usize]);
            Self { index, data, data_length }
        }
        fn bytes_expected() -> usize {
            35usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetWifiCertificateRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.index.write_to_slice(&mut target[0usize..2usize]);
            self.data.write_to_slice(&mut target[2usize..34usize]);
            self.data_length.write_to_slice(&mut target[34usize..35usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifiCertificateResponse {
        pub data: [u8; 32usize],
        pub data_length: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetWifiCertificateResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let data = <[u8; 32usize]>::from_le_byte_slice(&bytes[0usize..32usize]);
            let data_length = u8::from_le_byte_slice(&bytes[32usize..33usize]);
            Self { data, data_length }
        }
        fn bytes_expected() -> usize {
            33usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetWifiCertificateResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.data.write_to_slice(&mut target[0usize..32usize]);
            self.data_length.write_to_slice(&mut target[32usize..33usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum WifiPowerMode {
        FullSpeed,
        LowPower,
    }
    impl Into<u8> for WifiPowerMode {
        fn into(self) -> u8 {
            match self {
                WifiPowerMode::FullSpeed => 0u8,
                WifiPowerMode::LowPower => 1u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for WifiPowerMode {
        fn write_to_slice(self, target: &mut [u8]) {
            <WifiPowerMode as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for WifiPowerMode {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<WifiPowerMode> for u8 {
        type Error = ();
        fn try_into(self) -> Result<WifiPowerMode, Self::Error> {
            match self {
                0u8 => Ok(WifiPowerMode::FullSpeed),
                1u8 => Ok(WifiPowerMode::LowPower),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifiBufferInfoResponse {
        pub overflow: u32,
        pub low_watermark: u16,
        pub used: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetWifiBufferInfoResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let overflow = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let low_watermark = u16::from_le_byte_slice(&bytes[4usize..6usize]);
            let used = u16::from_le_byte_slice(&bytes[6usize..8usize]);
            Self {
                overflow,
                low_watermark,
                used,
            }
        }
        fn bytes_expected() -> usize {
            8usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetWifiBufferInfoResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.overflow.write_to_slice(&mut target[0usize..4usize]);
            self.low_watermark.write_to_slice(&mut target[4usize..6usize]);
            self.used.write_to_slice(&mut target[6usize..8usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum WifiDomain {
        Channel1To11,
        Channel1To13,
        Channel1To14,
    }
    impl Into<u8> for WifiDomain {
        fn into(self) -> u8 {
            match self {
                WifiDomain::Channel1To11 => 0u8,
                WifiDomain::Channel1To13 => 1u8,
                WifiDomain::Channel1To14 => 2u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for WifiDomain {
        fn write_to_slice(self, target: &mut [u8]) {
            <WifiDomain as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for WifiDomain {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<WifiDomain> for u8 {
        type Error = ();
        fn try_into(self) -> Result<WifiDomain, Self::Error> {
            match self {
                0u8 => Ok(WifiDomain::Channel1To11),
                1u8 => Ok(WifiDomain::Channel1To13),
                2u8 => Ok(WifiDomain::Channel1To14),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum ThresholdOption {
        Off,
        Outside,
        Inside,
        Smaller,
        Greater,
    }
    impl Into<char> for ThresholdOption {
        fn into(self) -> char {
            match self {
                ThresholdOption::Off => 'x',
                ThresholdOption::Outside => 'o',
                ThresholdOption::Inside => 'i',
                ThresholdOption::Smaller => '<',
                ThresholdOption::Greater => '>',
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for ThresholdOption {
        fn write_to_slice(self, target: &mut [u8]) {
            <ThresholdOption as Into<char>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for ThresholdOption {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            char::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<ThresholdOption> for char {
        type Error = ();
        fn try_into(self) -> Result<ThresholdOption, Self::Error> {
            match self {
                'x' => Ok(ThresholdOption::Off),
                'o' => Ok(ThresholdOption::Outside),
                'i' => Ok(ThresholdOption::Inside),
                '<' => Ok(ThresholdOption::Smaller),
                '>' => Ok(ThresholdOption::Greater),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetStackCurrentCallbackThresholdRequest {
        pub option: crate::bindings::master::ThresholdOption,
        pub min: u16,
        pub max: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetStackCurrentCallbackThresholdRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let option = crate::bindings::master::ThresholdOption::from_le_byte_slice(
                &bytes[0usize..1usize],
            );
            let min = u16::from_le_byte_slice(&bytes[1usize..3usize]);
            let max = u16::from_le_byte_slice(&bytes[3usize..5usize]);
            Self { option, min, max }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for SetStackCurrentCallbackThresholdRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.option.write_to_slice(&mut target[0usize..1usize]);
            self.min.write_to_slice(&mut target[1usize..3usize]);
            self.max.write_to_slice(&mut target[3usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetStackCurrentCallbackThresholdResponse {
        pub option: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::ThresholdOption,
            char,
        >,
        pub min: u16,
        pub max: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetStackCurrentCallbackThresholdResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let option = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::ThresholdOption,
                char,
            >::from_le_byte_slice(&bytes[0usize..1usize]);
            let min = u16::from_le_byte_slice(&bytes[1usize..3usize]);
            let max = u16::from_le_byte_slice(&bytes[3usize..5usize]);
            Self { option, min, max }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for GetStackCurrentCallbackThresholdResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.option.write_to_slice(&mut target[0usize..1usize]);
            self.min.write_to_slice(&mut target[1usize..3usize]);
            self.max.write_to_slice(&mut target[3usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetStackVoltageCallbackThresholdRequest {
        pub option: crate::bindings::master::ThresholdOption,
        pub min: u16,
        pub max: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetStackVoltageCallbackThresholdRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let option = crate::bindings::master::ThresholdOption::from_le_byte_slice(
                &bytes[0usize..1usize],
            );
            let min = u16::from_le_byte_slice(&bytes[1usize..3usize]);
            let max = u16::from_le_byte_slice(&bytes[3usize..5usize]);
            Self { option, min, max }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for SetStackVoltageCallbackThresholdRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.option.write_to_slice(&mut target[0usize..1usize]);
            self.min.write_to_slice(&mut target[1usize..3usize]);
            self.max.write_to_slice(&mut target[3usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetStackVoltageCallbackThresholdResponse {
        pub option: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::ThresholdOption,
            char,
        >,
        pub min: u16,
        pub max: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetStackVoltageCallbackThresholdResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let option = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::ThresholdOption,
                char,
            >::from_le_byte_slice(&bytes[0usize..1usize]);
            let min = u16::from_le_byte_slice(&bytes[1usize..3usize]);
            let max = u16::from_le_byte_slice(&bytes[3usize..5usize]);
            Self { option, min, max }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for GetStackVoltageCallbackThresholdResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.option.write_to_slice(&mut target[0usize..1usize]);
            self.min.write_to_slice(&mut target[1usize..3usize]);
            self.max.write_to_slice(&mut target[3usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetUsbVoltageCallbackThresholdRequest {
        pub option: crate::bindings::master::ThresholdOption,
        pub min: u16,
        pub max: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetUsbVoltageCallbackThresholdRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let option = crate::bindings::master::ThresholdOption::from_le_byte_slice(
                &bytes[0usize..1usize],
            );
            let min = u16::from_le_byte_slice(&bytes[1usize..3usize]);
            let max = u16::from_le_byte_slice(&bytes[3usize..5usize]);
            Self { option, min, max }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for SetUsbVoltageCallbackThresholdRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.option.write_to_slice(&mut target[0usize..1usize]);
            self.min.write_to_slice(&mut target[1usize..3usize]);
            self.max.write_to_slice(&mut target[3usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetUsbVoltageCallbackThresholdResponse {
        pub option: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::ThresholdOption,
            char,
        >,
        pub min: u16,
        pub max: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetUsbVoltageCallbackThresholdResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let option = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::ThresholdOption,
                char,
            >::from_le_byte_slice(&bytes[0usize..1usize]);
            let min = u16::from_le_byte_slice(&bytes[1usize..3usize]);
            let max = u16::from_le_byte_slice(&bytes[3usize..5usize]);
            Self { option, min, max }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for GetUsbVoltageCallbackThresholdResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.option.write_to_slice(&mut target[0usize..1usize]);
            self.min.write_to_slice(&mut target[1usize..3usize]);
            self.max.write_to_slice(&mut target[3usize..5usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum EthernetConnection {
        Dhcp,
        StaticIp,
    }
    impl Into<u8> for EthernetConnection {
        fn into(self) -> u8 {
            match self {
                EthernetConnection::Dhcp => 0u8,
                EthernetConnection::StaticIp => 1u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for EthernetConnection {
        fn write_to_slice(self, target: &mut [u8]) {
            <EthernetConnection as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for EthernetConnection {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<EthernetConnection> for u8 {
        type Error = ();
        fn try_into(self) -> Result<EthernetConnection, Self::Error> {
            match self {
                0u8 => Ok(EthernetConnection::Dhcp),
                1u8 => Ok(EthernetConnection::StaticIp),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetEthernetConfigurationRequest {
        pub connection: crate::bindings::master::EthernetConnection,
        pub ip: [u8; 4usize],
        pub subnet_mask: [u8; 4usize],
        pub gateway: [u8; 4usize],
        pub port: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetEthernetConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let connection = crate::bindings::master::EthernetConnection::from_le_byte_slice(
                &bytes[0usize..1usize],
            );
            let ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[1usize..5usize]);
            let subnet_mask = <[u8; 4usize]>::from_le_byte_slice(&bytes[5usize..9usize]);
            let gateway = <[u8; 4usize]>::from_le_byte_slice(&bytes[9usize..13usize]);
            let port = u16::from_le_byte_slice(&bytes[13usize..15usize]);
            Self {
                connection,
                ip,
                subnet_mask,
                gateway,
                port,
            }
        }
        fn bytes_expected() -> usize {
            15usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetEthernetConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.connection.write_to_slice(&mut target[0usize..1usize]);
            self.ip.write_to_slice(&mut target[1usize..5usize]);
            self.subnet_mask.write_to_slice(&mut target[5usize..9usize]);
            self.gateway.write_to_slice(&mut target[9usize..13usize]);
            self.port.write_to_slice(&mut target[13usize..15usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetEthernetConfigurationResponse {
        pub connection: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::EthernetConnection,
            u8,
        >,
        pub ip: [u8; 4usize],
        pub subnet_mask: [u8; 4usize],
        pub gateway: [u8; 4usize],
        pub port: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetEthernetConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let connection = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::EthernetConnection,
                u8,
            >::from_le_byte_slice(&bytes[0usize..1usize]);
            let ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[1usize..5usize]);
            let subnet_mask = <[u8; 4usize]>::from_le_byte_slice(&bytes[5usize..9usize]);
            let gateway = <[u8; 4usize]>::from_le_byte_slice(&bytes[9usize..13usize]);
            let port = u16::from_le_byte_slice(&bytes[13usize..15usize]);
            Self {
                connection,
                ip,
                subnet_mask,
                gateway,
                port,
            }
        }
        fn bytes_expected() -> usize {
            15usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetEthernetConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.connection.write_to_slice(&mut target[0usize..1usize]);
            self.ip.write_to_slice(&mut target[1usize..5usize]);
            self.subnet_mask.write_to_slice(&mut target[5usize..9usize]);
            self.gateway.write_to_slice(&mut target[9usize..13usize]);
            self.port.write_to_slice(&mut target[13usize..15usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetEthernetStatusResponse {
        pub mac_address: [u8; 6usize],
        pub ip: [u8; 4usize],
        pub subnet_mask: [u8; 4usize],
        pub gateway: [u8; 4usize],
        pub rx_count: u32,
        pub tx_count: u32,
        pub hostname: [char; 32usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetEthernetStatusResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let mac_address = <[u8; 6usize]>::from_le_byte_slice(&bytes[0usize..6usize]);
            let ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[6usize..10usize]);
            let subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[10usize..14usize],
            );
            let gateway = <[u8; 4usize]>::from_le_byte_slice(&bytes[14usize..18usize]);
            let rx_count = u32::from_le_byte_slice(&bytes[18usize..22usize]);
            let tx_count = u32::from_le_byte_slice(&bytes[22usize..26usize]);
            let hostname = <[char; 32usize]>::from_le_byte_slice(
                &bytes[26usize..58usize],
            );
            Self {
                mac_address,
                ip,
                subnet_mask,
                gateway,
                rx_count,
                tx_count,
                hostname,
            }
        }
        fn bytes_expected() -> usize {
            58usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetEthernetStatusResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.mac_address.write_to_slice(&mut target[0usize..6usize]);
            self.ip.write_to_slice(&mut target[6usize..10usize]);
            self.subnet_mask.write_to_slice(&mut target[10usize..14usize]);
            self.gateway.write_to_slice(&mut target[14usize..18usize]);
            self.rx_count.write_to_slice(&mut target[18usize..22usize]);
            self.tx_count.write_to_slice(&mut target[22usize..26usize]);
            self.hostname.write_to_slice(&mut target[26usize..58usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetEthernetWebsocketConfigurationRequest {
        pub sockets: u8,
        pub port: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetEthernetWebsocketConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let sockets = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let port = u16::from_le_byte_slice(&bytes[1usize..3usize]);
            Self { sockets, port }
        }
        fn bytes_expected() -> usize {
            3usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for SetEthernetWebsocketConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.sockets.write_to_slice(&mut target[0usize..1usize]);
            self.port.write_to_slice(&mut target[1usize..3usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetEthernetWebsocketConfigurationResponse {
        pub sockets: u8,
        pub port: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetEthernetWebsocketConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let sockets = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let port = u16::from_le_byte_slice(&bytes[1usize..3usize]);
            Self { sockets, port }
        }
        fn bytes_expected() -> usize {
            3usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for GetEthernetWebsocketConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.sockets.write_to_slice(&mut target[0usize..1usize]);
            self.port.write_to_slice(&mut target[1usize..3usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum ConnectionType {
        None,
        Usb,
        SpiStack,
        Chibi,
        Rs485,
        Wifi,
        Ethernet,
        Wifi2,
    }
    impl Into<u8> for ConnectionType {
        fn into(self) -> u8 {
            match self {
                ConnectionType::None => 0u8,
                ConnectionType::Usb => 1u8,
                ConnectionType::SpiStack => 2u8,
                ConnectionType::Chibi => 3u8,
                ConnectionType::Rs485 => 4u8,
                ConnectionType::Wifi => 5u8,
                ConnectionType::Ethernet => 6u8,
                ConnectionType::Wifi2 => 7u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for ConnectionType {
        fn write_to_slice(self, target: &mut [u8]) {
            <ConnectionType as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for ConnectionType {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<ConnectionType> for u8 {
        type Error = ();
        fn try_into(self) -> Result<ConnectionType, Self::Error> {
            match self {
                0u8 => Ok(ConnectionType::None),
                1u8 => Ok(ConnectionType::Usb),
                2u8 => Ok(ConnectionType::SpiStack),
                3u8 => Ok(ConnectionType::Chibi),
                4u8 => Ok(ConnectionType::Rs485),
                5u8 => Ok(ConnectionType::Wifi),
                6u8 => Ok(ConnectionType::Ethernet),
                7u8 => Ok(ConnectionType::Wifi2),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct WriteWifi2SerialPortRequest {
        pub data: [u8; 60usize],
        pub length: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for WriteWifi2SerialPortRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let data = <[u8; 60usize]>::from_le_byte_slice(&bytes[0usize..60usize]);
            let length = u8::from_le_byte_slice(&bytes[60usize..61usize]);
            Self { data, length }
        }
        fn bytes_expected() -> usize {
            61usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for WriteWifi2SerialPortRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.data.write_to_slice(&mut target[0usize..60usize]);
            self.length.write_to_slice(&mut target[60usize..61usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct ReadWifi2SerialPortResponse {
        pub data: [u8; 60usize],
        pub result: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for ReadWifi2SerialPortResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let data = <[u8; 60usize]>::from_le_byte_slice(&bytes[0usize..60usize]);
            let result = u8::from_le_byte_slice(&bytes[60usize..61usize]);
            Self { data, result }
        }
        fn bytes_expected() -> usize {
            61usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for ReadWifi2SerialPortResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.data.write_to_slice(&mut target[0usize..60usize]);
            self.result.write_to_slice(&mut target[60usize..61usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Wifi2PhyMode {
        B,
        G,
        N,
    }
    impl Into<u8> for Wifi2PhyMode {
        fn into(self) -> u8 {
            match self {
                Wifi2PhyMode::B => 0u8,
                Wifi2PhyMode::G => 1u8,
                Wifi2PhyMode::N => 2u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for Wifi2PhyMode {
        fn write_to_slice(self, target: &mut [u8]) {
            <Wifi2PhyMode as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for Wifi2PhyMode {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<Wifi2PhyMode> for u8 {
        type Error = ();
        fn try_into(self) -> Result<Wifi2PhyMode, Self::Error> {
            match self {
                0u8 => Ok(Wifi2PhyMode::B),
                1u8 => Ok(Wifi2PhyMode::G),
                2u8 => Ok(Wifi2PhyMode::N),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetWifi2ConfigurationRequest {
        pub port: u16,
        pub websocket_port: u16,
        pub website_port: u16,
        pub phy_mode: crate::bindings::master::Wifi2PhyMode,
        pub sleep_mode: u8,
        pub website: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetWifi2ConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let port = u16::from_le_byte_slice(&bytes[0usize..2usize]);
            let websocket_port = u16::from_le_byte_slice(&bytes[2usize..4usize]);
            let website_port = u16::from_le_byte_slice(&bytes[4usize..6usize]);
            let phy_mode = crate::bindings::master::Wifi2PhyMode::from_le_byte_slice(
                &bytes[6usize..7usize],
            );
            let sleep_mode = u8::from_le_byte_slice(&bytes[7usize..8usize]);
            let website = u8::from_le_byte_slice(&bytes[8usize..9usize]);
            Self {
                port,
                websocket_port,
                website_port,
                phy_mode,
                sleep_mode,
                website,
            }
        }
        fn bytes_expected() -> usize {
            9usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetWifi2ConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.port.write_to_slice(&mut target[0usize..2usize]);
            self.websocket_port.write_to_slice(&mut target[2usize..4usize]);
            self.website_port.write_to_slice(&mut target[4usize..6usize]);
            self.phy_mode.write_to_slice(&mut target[6usize..7usize]);
            self.sleep_mode.write_to_slice(&mut target[7usize..8usize]);
            self.website.write_to_slice(&mut target[8usize..9usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifi2ConfigurationResponse {
        pub port: u16,
        pub websocket_port: u16,
        pub website_port: u16,
        pub phy_mode: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::Wifi2PhyMode,
            u8,
        >,
        pub sleep_mode: u8,
        pub website: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetWifi2ConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let port = u16::from_le_byte_slice(&bytes[0usize..2usize]);
            let websocket_port = u16::from_le_byte_slice(&bytes[2usize..4usize]);
            let website_port = u16::from_le_byte_slice(&bytes[4usize..6usize]);
            let phy_mode = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::Wifi2PhyMode,
                u8,
            >::from_le_byte_slice(&bytes[6usize..7usize]);
            let sleep_mode = u8::from_le_byte_slice(&bytes[7usize..8usize]);
            let website = u8::from_le_byte_slice(&bytes[8usize..9usize]);
            Self {
                port,
                websocket_port,
                website_port,
                phy_mode,
                sleep_mode,
                website,
            }
        }
        fn bytes_expected() -> usize {
            9usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetWifi2ConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.port.write_to_slice(&mut target[0usize..2usize]);
            self.websocket_port.write_to_slice(&mut target[2usize..4usize]);
            self.website_port.write_to_slice(&mut target[4usize..6usize]);
            self.phy_mode.write_to_slice(&mut target[6usize..7usize]);
            self.sleep_mode.write_to_slice(&mut target[7usize..8usize]);
            self.website.write_to_slice(&mut target[8usize..9usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Wifi2ClientStatus {
        Idle,
        Connecting,
        WrongPassword,
        NoApFound,
        ConnectFailed,
        GotIp,
        Unknown,
    }
    impl Into<u8> for Wifi2ClientStatus {
        fn into(self) -> u8 {
            match self {
                Wifi2ClientStatus::Idle => 0u8,
                Wifi2ClientStatus::Connecting => 1u8,
                Wifi2ClientStatus::WrongPassword => 2u8,
                Wifi2ClientStatus::NoApFound => 3u8,
                Wifi2ClientStatus::ConnectFailed => 4u8,
                Wifi2ClientStatus::GotIp => 5u8,
                Wifi2ClientStatus::Unknown => 255u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for Wifi2ClientStatus {
        fn write_to_slice(self, target: &mut [u8]) {
            <Wifi2ClientStatus as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for Wifi2ClientStatus {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<Wifi2ClientStatus> for u8 {
        type Error = ();
        fn try_into(self) -> Result<Wifi2ClientStatus, Self::Error> {
            match self {
                0u8 => Ok(Wifi2ClientStatus::Idle),
                1u8 => Ok(Wifi2ClientStatus::Connecting),
                2u8 => Ok(Wifi2ClientStatus::WrongPassword),
                3u8 => Ok(Wifi2ClientStatus::NoApFound),
                4u8 => Ok(Wifi2ClientStatus::ConnectFailed),
                5u8 => Ok(Wifi2ClientStatus::GotIp),
                255u8 => Ok(Wifi2ClientStatus::Unknown),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifi2StatusResponse {
        pub client_enabled: bool,
        pub client_status: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::Wifi2ClientStatus,
            u8,
        >,
        pub client_ip: [u8; 4usize],
        pub client_subnet_mask: [u8; 4usize],
        pub client_gateway: [u8; 4usize],
        pub client_mac_address: [u8; 6usize],
        pub client_rx_count: u32,
        pub client_tx_count: u32,
        pub client_rssi: i8,
        pub ap_enabled: bool,
        pub ap_ip: [u8; 4usize],
        pub ap_subnet_mask: [u8; 4usize],
        pub ap_gateway: [u8; 4usize],
        pub ap_mac_address: [u8; 6usize],
        pub ap_rx_count: u32,
        pub ap_tx_count: u32,
        pub ap_connected_count: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetWifi2StatusResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let client_enabled = bool::from_le_byte_slice(&bytes[0usize..1usize]);
            let client_status = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::Wifi2ClientStatus,
                u8,
            >::from_le_byte_slice(&bytes[1usize..2usize]);
            let client_ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[2usize..6usize]);
            let client_subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[6usize..10usize],
            );
            let client_gateway = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[10usize..14usize],
            );
            let client_mac_address = <[u8; 6usize]>::from_le_byte_slice(
                &bytes[14usize..20usize],
            );
            let client_rx_count = u32::from_le_byte_slice(&bytes[20usize..24usize]);
            let client_tx_count = u32::from_le_byte_slice(&bytes[24usize..28usize]);
            let client_rssi = i8::from_le_byte_slice(&bytes[28usize..29usize]);
            let ap_enabled = bool::from_le_byte_slice(&bytes[29usize..30usize]);
            let ap_ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[30usize..34usize]);
            let ap_subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[34usize..38usize],
            );
            let ap_gateway = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[38usize..42usize],
            );
            let ap_mac_address = <[u8; 6usize]>::from_le_byte_slice(
                &bytes[42usize..48usize],
            );
            let ap_rx_count = u32::from_le_byte_slice(&bytes[48usize..52usize]);
            let ap_tx_count = u32::from_le_byte_slice(&bytes[52usize..56usize]);
            let ap_connected_count = u8::from_le_byte_slice(&bytes[56usize..57usize]);
            Self {
                client_enabled,
                client_status,
                client_ip,
                client_subnet_mask,
                client_gateway,
                client_mac_address,
                client_rx_count,
                client_tx_count,
                client_rssi,
                ap_enabled,
                ap_ip,
                ap_subnet_mask,
                ap_gateway,
                ap_mac_address,
                ap_rx_count,
                ap_tx_count,
                ap_connected_count,
            }
        }
        fn bytes_expected() -> usize {
            57usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetWifi2StatusResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.client_enabled.write_to_slice(&mut target[0usize..1usize]);
            self.client_status.write_to_slice(&mut target[1usize..2usize]);
            self.client_ip.write_to_slice(&mut target[2usize..6usize]);
            self.client_subnet_mask.write_to_slice(&mut target[6usize..10usize]);
            self.client_gateway.write_to_slice(&mut target[10usize..14usize]);
            self.client_mac_address.write_to_slice(&mut target[14usize..20usize]);
            self.client_rx_count.write_to_slice(&mut target[20usize..24usize]);
            self.client_tx_count.write_to_slice(&mut target[24usize..28usize]);
            self.client_rssi.write_to_slice(&mut target[28usize..29usize]);
            self.ap_enabled.write_to_slice(&mut target[29usize..30usize]);
            self.ap_ip.write_to_slice(&mut target[30usize..34usize]);
            self.ap_subnet_mask.write_to_slice(&mut target[34usize..38usize]);
            self.ap_gateway.write_to_slice(&mut target[38usize..42usize]);
            self.ap_mac_address.write_to_slice(&mut target[42usize..48usize]);
            self.ap_rx_count.write_to_slice(&mut target[48usize..52usize]);
            self.ap_tx_count.write_to_slice(&mut target[52usize..56usize]);
            self.ap_connected_count.write_to_slice(&mut target[56usize..57usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetWifi2ClientConfigurationRequest {
        pub enable: bool,
        pub ssid: [char; 32usize],
        pub ip: [u8; 4usize],
        pub subnet_mask: [u8; 4usize],
        pub gateway: [u8; 4usize],
        pub mac_address: [u8; 6usize],
        pub bssid: [u8; 6usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetWifi2ClientConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let enable = bool::from_le_byte_slice(&bytes[0usize..1usize]);
            let ssid = <[char; 32usize]>::from_le_byte_slice(&bytes[1usize..33usize]);
            let ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[33usize..37usize]);
            let subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[37usize..41usize],
            );
            let gateway = <[u8; 4usize]>::from_le_byte_slice(&bytes[41usize..45usize]);
            let mac_address = <[u8; 6usize]>::from_le_byte_slice(
                &bytes[45usize..51usize],
            );
            let bssid = <[u8; 6usize]>::from_le_byte_slice(&bytes[51usize..57usize]);
            Self {
                enable,
                ssid,
                ip,
                subnet_mask,
                gateway,
                mac_address,
                bssid,
            }
        }
        fn bytes_expected() -> usize {
            57usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for SetWifi2ClientConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.enable.write_to_slice(&mut target[0usize..1usize]);
            self.ssid.write_to_slice(&mut target[1usize..33usize]);
            self.ip.write_to_slice(&mut target[33usize..37usize]);
            self.subnet_mask.write_to_slice(&mut target[37usize..41usize]);
            self.gateway.write_to_slice(&mut target[41usize..45usize]);
            self.mac_address.write_to_slice(&mut target[45usize..51usize]);
            self.bssid.write_to_slice(&mut target[51usize..57usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifi2ClientConfigurationResponse {
        pub enable: bool,
        pub ssid: [char; 32usize],
        pub ip: [u8; 4usize],
        pub subnet_mask: [u8; 4usize],
        pub gateway: [u8; 4usize],
        pub mac_address: [u8; 6usize],
        pub bssid: [u8; 6usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetWifi2ClientConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let enable = bool::from_le_byte_slice(&bytes[0usize..1usize]);
            let ssid = <[char; 32usize]>::from_le_byte_slice(&bytes[1usize..33usize]);
            let ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[33usize..37usize]);
            let subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[37usize..41usize],
            );
            let gateway = <[u8; 4usize]>::from_le_byte_slice(&bytes[41usize..45usize]);
            let mac_address = <[u8; 6usize]>::from_le_byte_slice(
                &bytes[45usize..51usize],
            );
            let bssid = <[u8; 6usize]>::from_le_byte_slice(&bytes[51usize..57usize]);
            Self {
                enable,
                ssid,
                ip,
                subnet_mask,
                gateway,
                mac_address,
                bssid,
            }
        }
        fn bytes_expected() -> usize {
            57usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for GetWifi2ClientConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.enable.write_to_slice(&mut target[0usize..1usize]);
            self.ssid.write_to_slice(&mut target[1usize..33usize]);
            self.ip.write_to_slice(&mut target[33usize..37usize]);
            self.subnet_mask.write_to_slice(&mut target[37usize..41usize]);
            self.gateway.write_to_slice(&mut target[41usize..45usize]);
            self.mac_address.write_to_slice(&mut target[45usize..51usize]);
            self.bssid.write_to_slice(&mut target[51usize..57usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Wifi2ApEncryption {
        Open,
        Wep,
        WpaPsk,
        Wpa2Psk,
        WpaWpa2Psk,
    }
    impl Into<u8> for Wifi2ApEncryption {
        fn into(self) -> u8 {
            match self {
                Wifi2ApEncryption::Open => 0u8,
                Wifi2ApEncryption::Wep => 1u8,
                Wifi2ApEncryption::WpaPsk => 2u8,
                Wifi2ApEncryption::Wpa2Psk => 3u8,
                Wifi2ApEncryption::WpaWpa2Psk => 4u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for Wifi2ApEncryption {
        fn write_to_slice(self, target: &mut [u8]) {
            <Wifi2ApEncryption as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for Wifi2ApEncryption {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<Wifi2ApEncryption> for u8 {
        type Error = ();
        fn try_into(self) -> Result<Wifi2ApEncryption, Self::Error> {
            match self {
                0u8 => Ok(Wifi2ApEncryption::Open),
                1u8 => Ok(Wifi2ApEncryption::Wep),
                2u8 => Ok(Wifi2ApEncryption::WpaPsk),
                3u8 => Ok(Wifi2ApEncryption::Wpa2Psk),
                4u8 => Ok(Wifi2ApEncryption::WpaWpa2Psk),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetWifi2ApConfigurationRequest {
        pub enable: bool,
        pub ssid: [char; 32usize],
        pub ip: [u8; 4usize],
        pub subnet_mask: [u8; 4usize],
        pub gateway: [u8; 4usize],
        pub encryption: crate::bindings::master::Wifi2ApEncryption,
        pub hidden: bool,
        pub channel: u8,
        pub mac_address: [u8; 6usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetWifi2ApConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let enable = bool::from_le_byte_slice(&bytes[0usize..1usize]);
            let ssid = <[char; 32usize]>::from_le_byte_slice(&bytes[1usize..33usize]);
            let ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[33usize..37usize]);
            let subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[37usize..41usize],
            );
            let gateway = <[u8; 4usize]>::from_le_byte_slice(&bytes[41usize..45usize]);
            let encryption = crate::bindings::master::Wifi2ApEncryption::from_le_byte_slice(
                &bytes[45usize..46usize],
            );
            let hidden = bool::from_le_byte_slice(&bytes[46usize..47usize]);
            let channel = u8::from_le_byte_slice(&bytes[47usize..48usize]);
            let mac_address = <[u8; 6usize]>::from_le_byte_slice(
                &bytes[48usize..54usize],
            );
            Self {
                enable,
                ssid,
                ip,
                subnet_mask,
                gateway,
                encryption,
                hidden,
                channel,
                mac_address,
            }
        }
        fn bytes_expected() -> usize {
            54usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetWifi2ApConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.enable.write_to_slice(&mut target[0usize..1usize]);
            self.ssid.write_to_slice(&mut target[1usize..33usize]);
            self.ip.write_to_slice(&mut target[33usize..37usize]);
            self.subnet_mask.write_to_slice(&mut target[37usize..41usize]);
            self.gateway.write_to_slice(&mut target[41usize..45usize]);
            self.encryption.write_to_slice(&mut target[45usize..46usize]);
            self.hidden.write_to_slice(&mut target[46usize..47usize]);
            self.channel.write_to_slice(&mut target[47usize..48usize]);
            self.mac_address.write_to_slice(&mut target[48usize..54usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifi2ApConfigurationResponse {
        pub enable: bool,
        pub ssid: [char; 32usize],
        pub ip: [u8; 4usize],
        pub subnet_mask: [u8; 4usize],
        pub gateway: [u8; 4usize],
        pub encryption: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::Wifi2ApEncryption,
            u8,
        >,
        pub hidden: bool,
        pub channel: u8,
        pub mac_address: [u8; 6usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetWifi2ApConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let enable = bool::from_le_byte_slice(&bytes[0usize..1usize]);
            let ssid = <[char; 32usize]>::from_le_byte_slice(&bytes[1usize..33usize]);
            let ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[33usize..37usize]);
            let subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[37usize..41usize],
            );
            let gateway = <[u8; 4usize]>::from_le_byte_slice(&bytes[41usize..45usize]);
            let encryption = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::Wifi2ApEncryption,
                u8,
            >::from_le_byte_slice(&bytes[45usize..46usize]);
            let hidden = bool::from_le_byte_slice(&bytes[46usize..47usize]);
            let channel = u8::from_le_byte_slice(&bytes[47usize..48usize]);
            let mac_address = <[u8; 6usize]>::from_le_byte_slice(
                &bytes[48usize..54usize],
            );
            Self {
                enable,
                ssid,
                ip,
                subnet_mask,
                gateway,
                encryption,
                hidden,
                channel,
                mac_address,
            }
        }
        fn bytes_expected() -> usize {
            54usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetWifi2ApConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.enable.write_to_slice(&mut target[0usize..1usize]);
            self.ssid.write_to_slice(&mut target[1usize..33usize]);
            self.ip.write_to_slice(&mut target[33usize..37usize]);
            self.subnet_mask.write_to_slice(&mut target[37usize..41usize]);
            self.gateway.write_to_slice(&mut target[41usize..45usize]);
            self.encryption.write_to_slice(&mut target[45usize..46usize]);
            self.hidden.write_to_slice(&mut target[46usize..47usize]);
            self.channel.write_to_slice(&mut target[47usize..48usize]);
            self.mac_address.write_to_slice(&mut target[48usize..54usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifi2FirmwareVersionResponse {
        pub firmware_version_major: u8,
        pub firmware_version_minor: u8,
        pub firmware_version_revision: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetWifi2FirmwareVersionResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let firmware_version_major = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let firmware_version_minor = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let firmware_version_revision = u8::from_le_byte_slice(
                &bytes[2usize..3usize],
            );
            Self {
                firmware_version_major,
                firmware_version_minor,
                firmware_version_revision,
            }
        }
        fn bytes_expected() -> usize {
            3usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetWifi2FirmwareVersionResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.firmware_version_major.write_to_slice(&mut target[0usize..1usize]);
            self.firmware_version_minor.write_to_slice(&mut target[1usize..2usize]);
            self.firmware_version_revision.write_to_slice(&mut target[2usize..3usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetWifi2MeshConfigurationRequest {
        pub enable: bool,
        pub root_ip: [u8; 4usize],
        pub root_subnet_mask: [u8; 4usize],
        pub root_gateway: [u8; 4usize],
        pub router_bssid: [u8; 6usize],
        pub group_id: [u8; 6usize],
        pub group_ssid_prefix: [char; 16usize],
        pub gateway_ip: [u8; 4usize],
        pub gateway_port: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetWifi2MeshConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let enable = bool::from_le_byte_slice(&bytes[0usize..1usize]);
            let root_ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[1usize..5usize]);
            let root_subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[5usize..9usize],
            );
            let root_gateway = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[9usize..13usize],
            );
            let router_bssid = <[u8; 6usize]>::from_le_byte_slice(
                &bytes[13usize..19usize],
            );
            let group_id = <[u8; 6usize]>::from_le_byte_slice(&bytes[19usize..25usize]);
            let group_ssid_prefix = <[char; 16usize]>::from_le_byte_slice(
                &bytes[25usize..41usize],
            );
            let gateway_ip = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[41usize..45usize],
            );
            let gateway_port = u16::from_le_byte_slice(&bytes[45usize..47usize]);
            Self {
                enable,
                root_ip,
                root_subnet_mask,
                root_gateway,
                router_bssid,
                group_id,
                group_ssid_prefix,
                gateway_ip,
                gateway_port,
            }
        }
        fn bytes_expected() -> usize {
            47usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetWifi2MeshConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.enable.write_to_slice(&mut target[0usize..1usize]);
            self.root_ip.write_to_slice(&mut target[1usize..5usize]);
            self.root_subnet_mask.write_to_slice(&mut target[5usize..9usize]);
            self.root_gateway.write_to_slice(&mut target[9usize..13usize]);
            self.router_bssid.write_to_slice(&mut target[13usize..19usize]);
            self.group_id.write_to_slice(&mut target[19usize..25usize]);
            self.group_ssid_prefix.write_to_slice(&mut target[25usize..41usize]);
            self.gateway_ip.write_to_slice(&mut target[41usize..45usize]);
            self.gateway_port.write_to_slice(&mut target[45usize..47usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifi2MeshConfigurationResponse {
        pub enable: bool,
        pub root_ip: [u8; 4usize],
        pub root_subnet_mask: [u8; 4usize],
        pub root_gateway: [u8; 4usize],
        pub router_bssid: [u8; 6usize],
        pub group_id: [u8; 6usize],
        pub group_ssid_prefix: [char; 16usize],
        pub gateway_ip: [u8; 4usize],
        pub gateway_port: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetWifi2MeshConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let enable = bool::from_le_byte_slice(&bytes[0usize..1usize]);
            let root_ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[1usize..5usize]);
            let root_subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[5usize..9usize],
            );
            let root_gateway = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[9usize..13usize],
            );
            let router_bssid = <[u8; 6usize]>::from_le_byte_slice(
                &bytes[13usize..19usize],
            );
            let group_id = <[u8; 6usize]>::from_le_byte_slice(&bytes[19usize..25usize]);
            let group_ssid_prefix = <[char; 16usize]>::from_le_byte_slice(
                &bytes[25usize..41usize],
            );
            let gateway_ip = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[41usize..45usize],
            );
            let gateway_port = u16::from_le_byte_slice(&bytes[45usize..47usize]);
            Self {
                enable,
                root_ip,
                root_subnet_mask,
                root_gateway,
                router_bssid,
                group_id,
                group_ssid_prefix,
                gateway_ip,
                gateway_port,
            }
        }
        fn bytes_expected() -> usize {
            47usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for GetWifi2MeshConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.enable.write_to_slice(&mut target[0usize..1usize]);
            self.root_ip.write_to_slice(&mut target[1usize..5usize]);
            self.root_subnet_mask.write_to_slice(&mut target[5usize..9usize]);
            self.root_gateway.write_to_slice(&mut target[9usize..13usize]);
            self.router_bssid.write_to_slice(&mut target[13usize..19usize]);
            self.group_id.write_to_slice(&mut target[19usize..25usize]);
            self.group_ssid_prefix.write_to_slice(&mut target[25usize..41usize]);
            self.gateway_ip.write_to_slice(&mut target[41usize..45usize]);
            self.gateway_port.write_to_slice(&mut target[45usize..47usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Wifi2MeshStatus {
        Disabled,
        WifiConnecting,
        GotIp,
        MeshLocal,
        MeshOnline,
        ApAvailable,
        ApSetup,
        LeafAvailable,
    }
    impl Into<u8> for Wifi2MeshStatus {
        fn into(self) -> u8 {
            match self {
                Wifi2MeshStatus::Disabled => 0u8,
                Wifi2MeshStatus::WifiConnecting => 1u8,
                Wifi2MeshStatus::GotIp => 2u8,
                Wifi2MeshStatus::MeshLocal => 3u8,
                Wifi2MeshStatus::MeshOnline => 4u8,
                Wifi2MeshStatus::ApAvailable => 5u8,
                Wifi2MeshStatus::ApSetup => 6u8,
                Wifi2MeshStatus::LeafAvailable => 7u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for Wifi2MeshStatus {
        fn write_to_slice(self, target: &mut [u8]) {
            <Wifi2MeshStatus as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for Wifi2MeshStatus {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<Wifi2MeshStatus> for u8 {
        type Error = ();
        fn try_into(self) -> Result<Wifi2MeshStatus, Self::Error> {
            match self {
                0u8 => Ok(Wifi2MeshStatus::Disabled),
                1u8 => Ok(Wifi2MeshStatus::WifiConnecting),
                2u8 => Ok(Wifi2MeshStatus::GotIp),
                3u8 => Ok(Wifi2MeshStatus::MeshLocal),
                4u8 => Ok(Wifi2MeshStatus::MeshOnline),
                5u8 => Ok(Wifi2MeshStatus::ApAvailable),
                6u8 => Ok(Wifi2MeshStatus::ApSetup),
                7u8 => Ok(Wifi2MeshStatus::LeafAvailable),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifi2MeshCommonStatusResponse {
        pub status: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::master::Wifi2MeshStatus,
            u8,
        >,
        pub root_node: bool,
        pub root_candidate: bool,
        pub connected_nodes: u16,
        pub rx_count: u32,
        pub tx_count: u32,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetWifi2MeshCommonStatusResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let status = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::master::Wifi2MeshStatus,
                u8,
            >::from_le_byte_slice(&bytes[0usize..1usize]);
            let root_node = bool::from_le_byte_slice(&bytes[1usize..2usize]);
            let root_candidate = bool::from_le_byte_slice(&bytes[2usize..3usize]);
            let connected_nodes = u16::from_le_byte_slice(&bytes[3usize..5usize]);
            let rx_count = u32::from_le_byte_slice(&bytes[5usize..9usize]);
            let tx_count = u32::from_le_byte_slice(&bytes[9usize..13usize]);
            Self {
                status,
                root_node,
                root_candidate,
                connected_nodes,
                rx_count,
                tx_count,
            }
        }
        fn bytes_expected() -> usize {
            13usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetWifi2MeshCommonStatusResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.status.write_to_slice(&mut target[0usize..1usize]);
            self.root_node.write_to_slice(&mut target[1usize..2usize]);
            self.root_candidate.write_to_slice(&mut target[2usize..3usize]);
            self.connected_nodes.write_to_slice(&mut target[3usize..5usize]);
            self.rx_count.write_to_slice(&mut target[5usize..9usize]);
            self.tx_count.write_to_slice(&mut target[9usize..13usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifi2MeshClientStatusResponse {
        pub hostname: [char; 32usize],
        pub ip: [u8; 4usize],
        pub subnet_mask: [u8; 4usize],
        pub gateway: [u8; 4usize],
        pub mac_address: [u8; 6usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetWifi2MeshClientStatusResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let hostname = <[char; 32usize]>::from_le_byte_slice(
                &bytes[0usize..32usize],
            );
            let ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[32usize..36usize]);
            let subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[36usize..40usize],
            );
            let gateway = <[u8; 4usize]>::from_le_byte_slice(&bytes[40usize..44usize]);
            let mac_address = <[u8; 6usize]>::from_le_byte_slice(
                &bytes[44usize..50usize],
            );
            Self {
                hostname,
                ip,
                subnet_mask,
                gateway,
                mac_address,
            }
        }
        fn bytes_expected() -> usize {
            50usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetWifi2MeshClientStatusResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.hostname.write_to_slice(&mut target[0usize..32usize]);
            self.ip.write_to_slice(&mut target[32usize..36usize]);
            self.subnet_mask.write_to_slice(&mut target[36usize..40usize]);
            self.gateway.write_to_slice(&mut target[40usize..44usize]);
            self.mac_address.write_to_slice(&mut target[44usize..50usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetWifi2MeshApStatusResponse {
        pub ssid: [char; 32usize],
        pub ip: [u8; 4usize],
        pub subnet_mask: [u8; 4usize],
        pub gateway: [u8; 4usize],
        pub mac_address: [u8; 6usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetWifi2MeshApStatusResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let ssid = <[char; 32usize]>::from_le_byte_slice(&bytes[0usize..32usize]);
            let ip = <[u8; 4usize]>::from_le_byte_slice(&bytes[32usize..36usize]);
            let subnet_mask = <[u8; 4usize]>::from_le_byte_slice(
                &bytes[36usize..40usize],
            );
            let gateway = <[u8; 4usize]>::from_le_byte_slice(&bytes[40usize..44usize]);
            let mac_address = <[u8; 6usize]>::from_le_byte_slice(
                &bytes[44usize..50usize],
            );
            Self {
                ssid,
                ip,
                subnet_mask,
                gateway,
                mac_address,
            }
        }
        fn bytes_expected() -> usize {
            50usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetWifi2MeshApStatusResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.ssid.write_to_slice(&mut target[0usize..32usize]);
            self.ip.write_to_slice(&mut target[32usize..36usize]);
            self.subnet_mask.write_to_slice(&mut target[36usize..40usize]);
            self.gateway.write_to_slice(&mut target[40usize..44usize]);
            self.mac_address.write_to_slice(&mut target[44usize..50usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetBrickletXmcFlashConfigRequest {
        pub config: u32,
        pub parameter_1: u32,
        pub parameter_2: u32,
        pub data: [u8; 52usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetBrickletXmcFlashConfigRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let config = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let parameter_1 = u32::from_le_byte_slice(&bytes[4usize..8usize]);
            let parameter_2 = u32::from_le_byte_slice(&bytes[8usize..12usize]);
            let data = <[u8; 52usize]>::from_le_byte_slice(&bytes[12usize..64usize]);
            Self {
                config,
                parameter_1,
                parameter_2,
                data,
            }
        }
        fn bytes_expected() -> usize {
            64usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetBrickletXmcFlashConfigRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.config.write_to_slice(&mut target[0usize..4usize]);
            self.parameter_1.write_to_slice(&mut target[4usize..8usize]);
            self.parameter_2.write_to_slice(&mut target[8usize..12usize]);
            self.data.write_to_slice(&mut target[12usize..64usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetBrickletXmcFlashConfigResponse {
        pub return_value: u32,
        pub return_data: [u8; 60usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetBrickletXmcFlashConfigResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let return_value = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let return_data = <[u8; 60usize]>::from_le_byte_slice(
                &bytes[4usize..64usize],
            );
            Self { return_value, return_data }
        }
        fn bytes_expected() -> usize {
            64usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for SetBrickletXmcFlashConfigResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.return_value.write_to_slice(&mut target[0usize..4usize]);
            self.return_data.write_to_slice(&mut target[4usize..64usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetSpitfpBaudrateConfigRequest {
        pub enable_dynamic_baudrate: bool,
        pub minimum_dynamic_baudrate: u32,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetSpitfpBaudrateConfigRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let enable_dynamic_baudrate = bool::from_le_byte_slice(
                &bytes[0usize..1usize],
            );
            let minimum_dynamic_baudrate = u32::from_le_byte_slice(
                &bytes[1usize..5usize],
            );
            Self {
                enable_dynamic_baudrate,
                minimum_dynamic_baudrate,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetSpitfpBaudrateConfigRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.enable_dynamic_baudrate.write_to_slice(&mut target[0usize..1usize]);
            self.minimum_dynamic_baudrate.write_to_slice(&mut target[1usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetSpitfpBaudrateConfigResponse {
        pub enable_dynamic_baudrate: bool,
        pub minimum_dynamic_baudrate: u32,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetSpitfpBaudrateConfigResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let enable_dynamic_baudrate = bool::from_le_byte_slice(
                &bytes[0usize..1usize],
            );
            let minimum_dynamic_baudrate = u32::from_le_byte_slice(
                &bytes[1usize..5usize],
            );
            Self {
                enable_dynamic_baudrate,
                minimum_dynamic_baudrate,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetSpitfpBaudrateConfigResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.enable_dynamic_baudrate.write_to_slice(&mut target[0usize..1usize]);
            self.minimum_dynamic_baudrate.write_to_slice(&mut target[1usize..5usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum CommunicationMethod {
        None,
        Usb,
        SpiStack,
        Chibi,
        Rs485,
        Wifi,
        Ethernet,
        WifiV2,
    }
    impl Into<u8> for CommunicationMethod {
        fn into(self) -> u8 {
            match self {
                CommunicationMethod::None => 0u8,
                CommunicationMethod::Usb => 1u8,
                CommunicationMethod::SpiStack => 2u8,
                CommunicationMethod::Chibi => 3u8,
                CommunicationMethod::Rs485 => 4u8,
                CommunicationMethod::Wifi => 5u8,
                CommunicationMethod::Ethernet => 6u8,
                CommunicationMethod::WifiV2 => 7u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for CommunicationMethod {
        fn write_to_slice(self, target: &mut [u8]) {
            <CommunicationMethod as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for CommunicationMethod {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<CommunicationMethod> for u8 {
        type Error = ();
        fn try_into(self) -> Result<CommunicationMethod, Self::Error> {
            match self {
                0u8 => Ok(CommunicationMethod::None),
                1u8 => Ok(CommunicationMethod::Usb),
                2u8 => Ok(CommunicationMethod::SpiStack),
                3u8 => Ok(CommunicationMethod::Chibi),
                4u8 => Ok(CommunicationMethod::Rs485),
                5u8 => Ok(CommunicationMethod::Wifi),
                6u8 => Ok(CommunicationMethod::Ethernet),
                7u8 => Ok(CommunicationMethod::WifiV2),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetSpitfpBaudrateRequest {
        pub bricklet_port: char,
        pub baudrate: u32,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for SetSpitfpBaudrateRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let bricklet_port = char::from_le_byte_slice(&bytes[0usize..1usize]);
            let baudrate = u32::from_le_byte_slice(&bytes[1usize..5usize]);
            Self { bricklet_port, baudrate }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetSpitfpBaudrateRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.bricklet_port.write_to_slice(&mut target[0usize..1usize]);
            self.baudrate.write_to_slice(&mut target[1usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetSpitfpErrorCountResponse {
        pub error_count_ack_checksum: u32,
        pub error_count_message_checksum: u32,
        pub error_count_frame: u32,
        pub error_count_overflow: u32,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetSpitfpErrorCountResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let error_count_ack_checksum = u32::from_le_byte_slice(
                &bytes[0usize..4usize],
            );
            let error_count_message_checksum = u32::from_le_byte_slice(
                &bytes[4usize..8usize],
            );
            let error_count_frame = u32::from_le_byte_slice(&bytes[8usize..12usize]);
            let error_count_overflow = u32::from_le_byte_slice(&bytes[12usize..16usize]);
            Self {
                error_count_ack_checksum,
                error_count_message_checksum,
                error_count_frame,
                error_count_overflow,
            }
        }
        fn bytes_expected() -> usize {
            16usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetSpitfpErrorCountResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.error_count_ack_checksum.write_to_slice(&mut target[0usize..4usize]);
            self.error_count_message_checksum
                .write_to_slice(&mut target[4usize..8usize]);
            self.error_count_frame.write_to_slice(&mut target[8usize..12usize]);
            self.error_count_overflow.write_to_slice(&mut target[12usize..16usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetProtocol1BrickletNameResponse {
        pub protocol_version: u8,
        pub firmware_version_major: u8,
        pub firmware_version_minor: u8,
        pub firmware_version_revision: u8,
        pub name: [char; 40usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetProtocol1BrickletNameResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let protocol_version = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let firmware_version_major = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let firmware_version_minor = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let firmware_version_revision = u8::from_le_byte_slice(
                &bytes[3usize..4usize],
            );
            let name = <[char; 40usize]>::from_le_byte_slice(&bytes[4usize..44usize]);
            Self {
                protocol_version,
                firmware_version_major,
                firmware_version_minor,
                firmware_version_revision,
                name,
            }
        }
        fn bytes_expected() -> usize {
            44usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetProtocol1BrickletNameResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.protocol_version.write_to_slice(&mut target[0usize..1usize]);
            self.firmware_version_major.write_to_slice(&mut target[1usize..2usize]);
            self.firmware_version_minor.write_to_slice(&mut target[2usize..3usize]);
            self.firmware_version_revision.write_to_slice(&mut target[3usize..4usize]);
            self.name.write_to_slice(&mut target[4usize..44usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct WriteBrickletPluginRequest {
        pub port: char,
        pub offset: u8,
        pub chunk: [u8; 32usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for WriteBrickletPluginRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let port = char::from_le_byte_slice(&bytes[0usize..1usize]);
            let offset = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let chunk = <[u8; 32usize]>::from_le_byte_slice(&bytes[2usize..34usize]);
            Self { port, offset, chunk }
        }
        fn bytes_expected() -> usize {
            34usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for WriteBrickletPluginRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.port.write_to_slice(&mut target[0usize..1usize]);
            self.offset.write_to_slice(&mut target[1usize..2usize]);
            self.chunk.write_to_slice(&mut target[2usize..34usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct ReadBrickletPluginRequest {
        pub port: char,
        pub offset: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for ReadBrickletPluginRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let port = char::from_le_byte_slice(&bytes[0usize..1usize]);
            let offset = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            Self { port, offset }
        }
        fn bytes_expected() -> usize {
            2usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for ReadBrickletPluginRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.port.write_to_slice(&mut target[0usize..1usize]);
            self.offset.write_to_slice(&mut target[1usize..2usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetIdentityResponse {
        pub uid: [char; 8usize],
        pub connected_uid: [char; 8usize],
        pub position: char,
        pub hardware_version_major: u8,
        pub hardware_version_minor: u8,
        pub hardware_version_revision: u8,
        pub firmware_version_major: u8,
        pub firmware_version_minor: u8,
        pub firmware_version_revision: u8,
        pub device_identifier: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetIdentityResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let uid = <[char; 8usize]>::from_le_byte_slice(&bytes[0usize..8usize]);
            let connected_uid = <[char; 8usize]>::from_le_byte_slice(
                &bytes[8usize..16usize],
            );
            let position = char::from_le_byte_slice(&bytes[16usize..17usize]);
            let hardware_version_major = u8::from_le_byte_slice(
                &bytes[17usize..18usize],
            );
            let hardware_version_minor = u8::from_le_byte_slice(
                &bytes[18usize..19usize],
            );
            let hardware_version_revision = u8::from_le_byte_slice(
                &bytes[19usize..20usize],
            );
            let firmware_version_major = u8::from_le_byte_slice(
                &bytes[20usize..21usize],
            );
            let firmware_version_minor = u8::from_le_byte_slice(
                &bytes[21usize..22usize],
            );
            let firmware_version_revision = u8::from_le_byte_slice(
                &bytes[22usize..23usize],
            );
            let device_identifier = u16::from_le_byte_slice(&bytes[23usize..25usize]);
            Self {
                uid,
                connected_uid,
                position,
                hardware_version_major,
                hardware_version_minor,
                hardware_version_revision,
                firmware_version_major,
                firmware_version_minor,
                firmware_version_revision,
                device_identifier,
            }
        }
        fn bytes_expected() -> usize {
            25usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetIdentityResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.uid.write_to_slice(&mut target[0usize..8usize]);
            self.connected_uid.write_to_slice(&mut target[8usize..16usize]);
            self.position.write_to_slice(&mut target[16usize..17usize]);
            self.hardware_version_major.write_to_slice(&mut target[17usize..18usize]);
            self.hardware_version_minor.write_to_slice(&mut target[18usize..19usize]);
            self.hardware_version_revision.write_to_slice(&mut target[19usize..20usize]);
            self.firmware_version_major.write_to_slice(&mut target[20usize..21usize]);
            self.firmware_version_minor.write_to_slice(&mut target[21usize..22usize]);
            self.firmware_version_revision.write_to_slice(&mut target[22usize..23usize]);
            self.device_identifier.write_to_slice(&mut target[23usize..25usize]);
        }
    }
    impl MasterBrick {
        pub fn new(
            uid: impl Into<tinkerforge_base::base58::Uid>,
            connection: tinkerforge_base::ip_connection::async_io::AsyncIpConnection,
        ) -> MasterBrick {
            Self {
                device: tinkerforge_base::device::Device::new(
                    uid.into(),
                    connection,
                    "Master",
                ),
            }
        }
        pub fn uid(&self) -> tinkerforge_base::base58::Uid {
            self.device.uid()
        }
        /**
Gibt die Spannung des Stapels zurück. Diese Spannung wird über
den Stapel verteilt und kann zum Beispiel über eine Step-Down oder
Step-Up Power Supply eingespeist werden.

.. note::
 Es ist mit dieser Funktion nicht möglich, Spannungen, die per PoE oder USB eingespeist werden, zu messen.
*/
        pub async fn get_stack_voltage(
            &mut self,
        ) -> Result<u16, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(1u8, &payload).await?;
            Ok(u16::from_le_byte_slice(&result.body()[0..2usize]))
        }
        /**
Gibt den Stromverbrauch des Stapels zurück. Der angegebene Strom
bezieht sich auf den Stromverbrauch der am Stapel angeschlossenen Verbraucher.
Die Speisung kann z.B. über eine Step-Down oder Step-Up Power Supply erfolgen.

.. note::
 Es ist mit dieser Funktion nicht möglich, den Stromverbrauch über PoE oder USB zu messen.
*/
        pub async fn get_stack_current(
            &mut self,
        ) -> Result<u16, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(2u8, &payload).await?;
            Ok(u16::from_le_byte_slice(&result.body()[0..2usize]))
        }
        /**
Schreibt den Typ der Extension in den EEPROM der angegebenen Extension.
Die Extension kann entweder 0 oder 1 sein (0 ist die untere, 1
die obere, wenn nur eine Extension verfügbar ist, ist 0 zu verwenden)

Mögliche Extensiontypen:

.. csv-table::
 :header: "Typ", "Beschreibung"
 :widths: 10, 100

 "1",    "Chibi"
 "2",    "RS485"
 "3",    "WIFI"
 "4",    "Ethernet"
 "5",    "WIFI 2.0"

Der Typ der Extension ist schon gesetzt beim Erwerb der Extension und kann
über den Brick Viewer gesetzt werden. Daher ist es unwahrscheinlich, dass
diese Funktion benötigt wird.
*/
        pub async fn set_extension_type(
            &mut self,
            request: crate::bindings::master::SetExtensionTypeRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 5usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(3u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt den Typ der angegebenen Extension zurück, wie von :func:`Set Extension Type` gesetzt.
*/
        pub async fn get_extension_type(
            &mut self,
            request: u8,
        ) -> Result<
            tinkerforge_base::byte_converter::ParsedOrRaw<
                crate::bindings::master::ExtensionType,
                u32,
            >,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(4u8, &payload).await?;
            Ok(
                tinkerforge_base::byte_converter::ParsedOrRaw::<
                    crate::bindings::master::ExtensionType,
                    u32,
                >::from_le_byte_slice(&result.body()[0..4usize]),
            )
        }
        /**
Gibt *true* zurück, wenn der Master Brick an Position 0 im Stapel und eine
Chibi Extension verfügbar ist.
*/
        pub async fn is_chibi_present(
            &mut self,
        ) -> Result<bool, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(5u8, &payload).await?;
            Ok(bool::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Setzt die zugehörige Adresse der Chibi Extension.

Es ist möglich die Adresse mit dem Brick Viewer zu setzen und diese
wird im EEPROM der Chibi Extension abgespeichert. Ein Setzen bei
jedem Start ist daher nicht notwendig.
*/
        pub async fn set_chibi_address(
            &mut self,
            request: u8,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(6u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Adresse zurück, wie von :func:`Set Chibi Address` gesetzt.
*/
        pub async fn get_chibi_address(
            &mut self,
        ) -> Result<u8, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(7u8, &payload).await?;
            Ok(u8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Setzt die Adresse des Chibi Master. Diese Adresse wird verwendet
wenn die Chibi Extension als Slave verwendet wird (z.B. wenn keine USB-Verbindung
besteht).

Es ist möglich die Adresse mit dem Brick Viewer zu setzen und diese wird im
EEPROM der Chibi Extension abgespeichert. Ein Setzen bei
jedem Start ist daher nicht notwendig.
*/
        pub async fn set_chibi_master_address(
            &mut self,
            request: u8,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(8u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Adresse zurück, wie von :func:`Set Chibi Master Address` gesetzt.
*/
        pub async fn get_chibi_master_address(
            &mut self,
        ) -> Result<u8, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(9u8, &payload).await?;
            Ok(u8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Setzt bis zu 254 Slave Adressen. 0 hat eine
besondere Bedeutung, sie wird zur Terminierung der Liste verwendet und ist nicht
als normale Slave Adresse erlaubt.
Die Adressnummerierung (mittels :param:`num` Parameter) muss aufsteigend ab
0 erfolgen. Beispiel: Wenn die Chibi Extension im Master Modus verwendet wird
(z.B. wenn der Stapel eine USB-Verbindung hat) und es soll mit drei weiteren
Chibi Stapeln kommuniziert werden, mit den Adressen 17, 23 und 42, sollten die
Aufrufe ``(0, 17)``, ``(1, 23)``, ``(2, 42)`` und ``(3, 0)`` sein. Der letzte
Aufruf mit ``(3, 0)`` dient der Terminierung der Liste und zeigt an, dass die
Chibi Slave Adressliste in diesem Fall 3 Einträge beinhaltet.

Es ist möglich die Adressen mit dem Brick Viewer zu setzen, dieser kümmert sich
dann um korrekte Adressnummerierung und Terminierung der Liste.

Die Slave Adresse werden im EEPROM der Chibi Extension abgespeichert. Ein
Setzen bei jedem Start ist daher nicht notwendig.
*/
        pub async fn set_chibi_slave_address(
            &mut self,
            request: crate::bindings::master::SetChibiSlaveAddressRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 2usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(10u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Slave Adresse für eine Adressnummerierung (mittels :param:`num` Parameter)
zurück, wie von :func:`Set Chibi Slave Address` gesetzt.
*/
        pub async fn get_chibi_slave_address(
            &mut self,
            request: u8,
        ) -> Result<u8, tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(11u8, &payload).await?;
            Ok(u8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Gibt die Signalstärke in dBm zurück. Die Aktualisierung der Signalstärke
wird bei jedem Empfang eines Paketes durchgeführt.
*/
        pub async fn get_chibi_signal_strength(
            &mut self,
        ) -> Result<u8, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(12u8, &payload).await?;
            Ok(u8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Gibt folgende Fehlerzähler der Chibi Kommunikation zurück: Underrun, CRC Fehler,
kein ACK und Overflow. Bei Anstieg dieser Fehlerzähler ist es wahrscheinlich, dass
entweder die Entfernung zwischen zwei Chibi Stapeln zu groß wird oder Störungen
vorliegen.
*/
        pub async fn get_chibi_error_log(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetChibiErrorLogResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(13u8, &payload).await?;
            Ok(
                crate::bindings::master::GetChibiErrorLogResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt den Chibi Frequenzbereich der Chibi Extension. Mögliche Werte sind:

.. csv-table::
 :header: "Typ", "Beschreibung"
 :widths: 10, 100

 "0",    "OQPSK 868MHz (Europe)"
 "1",    "OQPSK 915MHz (US)"
 "2",    "OQPSK 780MHz (China)"
 "3",    "BPSK40 915MHz"

Es ist möglich den Frequenzbereich mit dem Brick Viewer zu setzen und dieser wird
im EEPROM der Chibi Extension abgespeichert. Ein Setzen bei
jedem Start ist daher nicht notwendig.
*/
        pub async fn set_chibi_frequency(
            &mut self,
            request: crate::bindings::master::ChibiFrequency,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(14u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt den Frequenzbereich zurück, wie von :func:`Set Chibi Frequency` gesetzt.
*/
        pub async fn get_chibi_frequency(
            &mut self,
        ) -> Result<
            tinkerforge_base::byte_converter::ParsedOrRaw<
                crate::bindings::master::ChibiFrequency,
                u8,
            >,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(15u8, &payload).await?;
            Ok(
                tinkerforge_base::byte_converter::ParsedOrRaw::<
                    crate::bindings::master::ChibiFrequency,
                    u8,
                >::from_le_byte_slice(&result.body()[0..1usize]),
            )
        }
        /**
Setzt den verwendeten Kanal der Chibi Extension. Die möglichen Kanäle sind
abhängig vom verwendeten Frequenzbereich:

.. csv-table::
 :header: "Frequenzbereich", "Mögliche Kanäle"
 :widths: 40, 60

 "OQPSK 868MHz (Europe)", "0"
 "OQPSK 915MHz (US)",     "1, 2, 3, 4, 5, 6, 7, 8, 9, 10"
 "OQPSK 780MHz (China)",  "0, 1, 2, 3"
 "BPSK40 915MHz",         "1, 2, 3, 4, 5, 6, 7, 8, 9, 10"

Es ist möglich den Kanal mit dem Brick Viewer zu setzen und dieser wird
im EEPROM der Chibi Extension abgespeichert. Ein Setzen bei
jedem Start ist daher nicht notwendig.
*/
        pub async fn set_chibi_channel(
            &mut self,
            request: u8,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(16u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt den Kanal zurück, wie von :func:`Set Chibi Channel` gesetzt.
*/
        pub async fn get_chibi_channel(
            &mut self,
        ) -> Result<u8, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(17u8, &payload).await?;
            Ok(u8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Gibt *true* zurück, wenn der Master Brick an Position 0 im Stapel und eine
RS485 Extension verfügbar ist.
*/
        pub async fn is_rs_485_present(
            &mut self,
        ) -> Result<bool, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(18u8, &payload).await?;
            Ok(bool::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Setzt die zugehörige Adresse (0-255) der RS485 Extension.

Um eine RS485 Extension als RS485 Master (z.B. verbunden mit einem PC über
USB) zu betreiben muss die Adresse auf 0 gesetzt werden.

Es ist möglich die Adresse mit dem Brick Viewer zu setzen und diese wird im
EEPROM der RS485 Extension abgespeichert. Ein Setzen bei
jedem Start ist daher nicht notwendig.
*/
        pub async fn set_rs_485_address(
            &mut self,
            request: u8,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(19u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Adresse zurück, wie von :func:`Set RS485 Address` gesetzt.
*/
        pub async fn get_rs_485_address(
            &mut self,
        ) -> Result<u8, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(20u8, &payload).await?;
            Ok(u8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Setzt bis zu 255 Slave Adressen. Gültige Adressen sind 1-255. 0 hat eine
besondere Bedeutung, sie wird zur Terminierung der Liste verwendet und ist nicht
als normale Slave Adresse erlaubt.
Die Adressnummerierung (mittels ``num`` Parameter) muss aufsteigend ab
0 erfolgen. Beispiel: Wenn die RS485 Extension im Master Modus verwendet wird
(z.B. wenn der Stapel eine USB-Verbindung hat) und es soll mit drei weiteren
RS485 Stapeln kommuniziert werden, mit den Adressen 17, 23 und 42, sollten die
Aufrufe ``(0, 17)``, ``(1, 23)``, ``(2, 42)`` und ``(3, 0)`` sein. Der letzte
Aufruf mit ``(3, 0)`` dient der Terminierung der Liste und zeigt an, dass die
RS485 Slave Adressliste in diesem Fall 3 Einträge beinhaltet.

Es ist möglich die Adressen mit dem Brick Viewer zu setzen, dieser kümmert sich
dann um korrekte Adressnummerierung und Terminierung der Liste.

Die Slave Adresse werden im EEPROM der RS485 Extension abgespeichert. Ein
Setzen bei jedem Start ist daher nicht notwendig.
*/
        pub async fn set_rs_485_slave_address(
            &mut self,
            request: crate::bindings::master::SetRs485SlaveAddressRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 2usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(21u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Slave Adresse für eine Adressnummerierung (mittels ``num`` Parameter)
zurück, wie von :func:`Set RS485 Slave Address` gesetzt.
*/
        pub async fn get_rs_485_slave_address(
            &mut self,
            request: u8,
        ) -> Result<u8, tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(22u8, &payload).await?;
            Ok(u8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Gibt den CRC Fehlerzähler der RS485 Kommunikation zurück. Wenn dieser Zähler
ansteigt ist es wahrscheinlich, dass der Abstand zwischen zwei RS485-Teilnehmern
zu groß ist oder es Störungen gibt.
*/
        pub async fn get_rs_485_error_log(
            &mut self,
        ) -> Result<u16, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(23u8, &payload).await?;
            Ok(u16::from_le_byte_slice(&result.body()[0..2usize]))
        }
        /**
Setzt die Schnittstellenkonfiguration der RS485 Extension.
Der Master Brick versucht die vorgegebene Baudrate so
genau wie möglich zu erreichen. Die maximale empfohlene Baudrate ist 2000000
(2MBd). Mögliche Werte für die Parität sind 'n' (keine), 'e' (gerade) und
'o' (ungerade).

Wenn die RS485 Kommunikation instabil ist (verlorene Nachrichten etc.), sollte
zuerst die Baudrate verringert werden. Sehr lange Busleitungen (z.B. 1km)
sollten möglichst Werte im Bereich von 100000 (100kBd) verwenden.

Die Werte sind im EEPROM gespeichert und werden nur beim Start angewandt. Dass
bedeutet, der Master Brick muss nach einer Konfiguration neu gestartet werden.
*/
        pub async fn set_rs_485_configuration(
            &mut self,
            request: crate::bindings::master::SetRs485ConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 6usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(24u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Schnittstellenkonfiguration zurück, wie von :func:`Set RS485 Configuration` gesetzt.
*/
        pub async fn get_rs_485_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetRs485ConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(25u8, &payload).await?;
            Ok(
                crate::bindings::master::GetRs485ConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Gibt *true* zurück, wenn der Master Brick an Position 0 im Stapel und eine
WIFI Extension verfügbar ist.
*/
        pub async fn is_wifi_present(
            &mut self,
        ) -> Result<bool, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(26u8, &payload).await?;
            Ok(bool::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Setzt die Konfiguration der WIFI Extension. Die ``ssid`` darf eine maximale
Länge von 32 Zeichen haben. Mögliche Werte für ``connection`` sind:

.. csv-table::
 :header: "Wert", "Beschreibung"
 :widths: 10, 90

 "0", "DHCP"
 "1", "Statische IP"
 "2", "Access Point: DHCP"
 "3", "Access Point: Statische IP"
 "4", "Ad Hoc: DHCP"
 "5", "Ad Hoc: Statische IP"

Wenn ``connection`` auf eine der statische IP Optionen gesetzt wird, dann müssen
``ip``, ``subnet_mask`` und ``gateway`` als ein Array der Größe 4 angegeben
werden. Dabei ist das erste Element im Array das niederwertigste Byte. Falls
``connection`` auf eine der DHCP Optionen gesetzt ist, werden ``ip``,
``subnet_mask`` und ``gateway`` ignoriert.

Der letzte Parameter ist der Port auf den das Anwendungsprogramm sich
verbindet.

Die Werte sind im EEPROM gespeichert und werden nur beim Start angewandt. Dass
bedeutet, der Master Brick muss nach einer Konfiguration neu gestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die WIFI Extension zu
konfigurieren.
*/
        pub async fn set_wifi_configuration(
            &mut self,
            request: crate::bindings::master::SetWifiConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 47usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(27u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Konfiguration zurück, wie von :func:`Set Wifi Configuration` gesetzt.
*/
        pub async fn get_wifi_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifiConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(28u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifiConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt die Verschlüsselung der WIFI Extension. Der erste Parameter ist der
Typ der Verschlüsselung. Mögliche Werte sind:

.. csv-table::
 :header: "Wert", "Beschreibung"
 :widths: 10, 90

 "0", "WPA/WPA2"
 "1", "WPA Enterprise (EAP-FAST, EAP-TLS, EAP-TTLS, PEAP)"
 "2", "WEP"
 "3", "Keine Verschlüsselung"

Der ``key`` hat eine maximale Länge von 50 Zeichen und wird benutzt
falls ``encryption`` auf 0 oder 2 (WPA/WPA2 oder WEP) gesetzt ist. Andernfalls
wird dieser Parameter ignoriert.

Für WPA/WPA2 muss der Schlüssel mindestens 8 Zeichen lang sein. Wenn ein
Schlüssel mit mehr als 50 Zeichen gesetzt werden soll, kann
:func:`Set Long Wifi Key` genutzt werden.

Für WEP muss der Schlüssel entweder 10 oder 26 hexadezimale Zeichen lang sein.
Es ist möglich den ``key_index`` zu setzen (1-4). Fall der ``key_index``
unbekannt ist, ist er wahrscheinlich 1.

Wenn WPA Enterprise als ``encryption`` gewählt wird, müssen ``eap_options`` und
die Länge der Zertifikate gesetzt werden. Die Zertifikate selbst können mit
:func:`Set Wifi Certificate` übertragen
werden. Die ``eap_options`` bestehen aus Outer Authentication (Bits 1-2),
Inner Authentication (Bit 3) und Certificate Type (Bits 4-5):

.. csv-table::
 :header: "Option", "Bits", "Beschreibung"
 :widths: 10, 10, 80

 "Outer Authentication", "1-2", "0=EAP-FAST, 1=EAP-TLS, 2=EAP-TTLS, 3=EAP-PEAP"
 "Inner Authentication", "3", "0=EAP-MSCHAP, 1=EAP-GTC"
 "Certificate Type", "4-5", "0=CA Certificate, 1=Client Certificate, 2=Private Key"

Beispiel für EAP-TTLS + EAP-GTC + Private Key: ``option = 2 | (1 << 2) | (2 << 3)``.

Die Werte sind im EEPROM gespeichert und werden nur beim Start angewandt.
Das bedeutet der Master Brick muss nach einer Konfiguration neu gestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die WLAN Verschlüsselung
zu konfigurieren.
*/
        pub async fn set_wifi_encryption(
            &mut self,
            request: crate::bindings::master::SetWifiEncryptionRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 59usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(29u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Verschlüsselungseinstellungen zurück, wie von
:func:`Set Wifi Encryption` gesetzt.

.. note::
 Seit Master Brick Firmware Version 2.4.4 wird der Schlüssel nicht mehr
 zurückgegeben.
*/
        pub async fn get_wifi_encryption(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifiEncryptionResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(30u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifiEncryptionResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Gibt den Status der WIFI Extension zurück. ``state`` wird automatisch
aktualisiert, alle anderen Parameter werden nur beim Starten und nach jedem
Aufruf von :func:`Refresh Wifi Status` aktualisiert.

Mögliche Werte für *state* sind:

.. csv-table::
 :header: "State", "Beschreibung"
 :widths: 10, 90

 "0", "Getrennt"
 "1", "Verbunden"
 "2", "Verbindung wird aufgebaut"
 "3", "Fehler"
 "255", "Noch nicht initialisiert"
*/
        pub async fn get_wifi_status(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifiStatusResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(31u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifiStatusResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Aktualisiert den WLAN Status (siehe :func:`Get Wifi Status`). Um den Status
vom WLAN Modul zu lesen, muss der Master Brick vom Datenmodus in den
Kommandomodus und wieder zurück wechseln. Dieser Wechsel und das eigentliche
Auslesen ist leider zeitaufwändig. Dass heißt, es dauert ein paar ms bis der
Stapel mit aufgesteckter WIFI Extension wieder reagiert nachdem die
Funktion aufgerufen wurde.
*/
        pub async fn refresh_wifi_status(
            &mut self,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            self.device
                .set(32u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Diese Funktion kann benutzt werden um sowohl das Zertifikat als auch
Benutzername und Passwort für WPA Enterprise zu setzen. Für den Benutzernamen
muss Index 0xFFFF und für das Password Index 0xFFFE genutzt werden.
Die maximale Länge für beide ist 32.

Das Zertifikat wird in Chunks der Größe 32 geschrieben und der Index
gibt den Index des Chunk an. ``data_length`` sollte fast immer auf 32 gesetzt
werden. Nur beim letzten Chunk ist eine Länge ungleich 32 möglich.

Der Startindex für CA Certificate ist 0, für Client Certificate 10000 und
für Private Key 20000. Die Maximalen Dateigrößen sind jeweils 1312, 1312 und
4320 Byte.

Die Werte sind im EEPROM gespeichert und werden nur beim Start angewandt.
Das bedeutet der Master Brick muss nach einer Konfiguration neu gestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die Zertifikate, Benutzernamen
und Passwort zu konfigurieren.
*/
        pub async fn set_wifi_certificate(
            &mut self,
            request: crate::bindings::master::SetWifiCertificateRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 35usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(33u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt das Zertifikat für einen Index zurück, wie von
:func:`Set Wifi Certificate` gesetzt.
*/
        pub async fn get_wifi_certificate(
            &mut self,
            request: u16,
        ) -> Result<
            crate::bindings::master::GetWifiCertificateResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 2usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(34u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifiCertificateResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt den Stromsparmodus für die WIFI Extension. Mögliche Werte sind:

.. csv-table::
 :header: "Mode", "Beschreibung"
 :widths: 10, 90

 "0", "Full Speed (hoher Stromverbrauch, hoher Durchsatz)"
 "1", "Low Power (geringer Stromverbrauch, geringer Durchsatz)"
*/
        pub async fn set_wifi_power_mode(
            &mut self,
            request: crate::bindings::master::WifiPowerMode,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(35u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt den Stromsparmodus zurück, wie von :func:`Set Wifi Power Mode` gesetzt.
*/
        pub async fn get_wifi_power_mode(
            &mut self,
        ) -> Result<
            tinkerforge_base::byte_converter::ParsedOrRaw<
                crate::bindings::master::WifiPowerMode,
                u8,
            >,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(36u8, &payload).await?;
            Ok(
                tinkerforge_base::byte_converter::ParsedOrRaw::<
                    crate::bindings::master::WifiPowerMode,
                    u8,
                >::from_le_byte_slice(&result.body()[0..1usize]),
            )
        }
        /**
Gibt Informationen über denn WLAN Empfangsbuffer zurück. Der WLAN
Empfangsbuffer hat eine maximale Größe von 1500 Byte und falls zu viele
Daten übertragen werden, kann er überlaufen.

Die Rückgabewerte sind die Anzahl der Overflows, die Low-Watermark
(d.h. die kleinste Anzahl an Byte die je noch frei waren im Buffer) und
die Anzahl der im Moment verwendeten Bytes im Buffer.

Es sollte immer versucht werden den Buffer leer zu halten, andernfalls
ist mit einer permanenten Latenz zu rechnen. Eine gute Daumenregel ist,
nicht mehr als 1000 Nachrichten pro Sekunde zu verschicken.

Dabei sollten am besten nie mehr als 50 Nachrichten auf einmal ohne
Pausen gesendet werden.
*/
        pub async fn get_wifi_buffer_info(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifiBufferInfoResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(37u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifiBufferInfoResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt den Geltungsbereich der WIFI Extension. Mögliche Werte sind:

.. csv-table::
 :header: "Geltungsbereich", "Beschreibung"
 :widths: 10, 90

 "0", "FCC: Kanal 1-11 (N/S Amerika, Australien, Neuseeland)"
 "1", "ETSI: Kanal 1-13 (Europa, Mittlerer Osten, Afrika)"
 "2", "TELEC: Kanal 1-14 (Japan)"
*/
        pub async fn set_wifi_regulatory_domain(
            &mut self,
            request: crate::bindings::master::WifiDomain,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(38u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt den Geltungsbereich zurück, wie von :func:`Set Wifi Regulatory Domain` gesetzt.
*/
        pub async fn get_wifi_regulatory_domain(
            &mut self,
        ) -> Result<
            tinkerforge_base::byte_converter::ParsedOrRaw<
                crate::bindings::master::WifiDomain,
                u8,
            >,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(39u8, &payload).await?;
            Ok(
                tinkerforge_base::byte_converter::ParsedOrRaw::<
                    crate::bindings::master::WifiDomain,
                    u8,
                >::from_le_byte_slice(&result.body()[0..1usize]),
            )
        }
        /**
Gibt die USB Spannung zurück. Funktioniert nicht mit Hardware Version 2.1 oder neuer.
*/
        pub async fn get_usb_voltage(
            &mut self,
        ) -> Result<u16, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(40u8, &payload).await?;
            Ok(u16::from_le_byte_slice(&result.body()[0..2usize]))
        }
        /**
Setzt einen langen WLAN Schlüssel (bis zu 63 Zeichen, mindestens 8 Zeichen) für
WPA Verschlüsselung. Dieser Schlüssel wird genutzt, wenn der Schlüssel in
:func:`Set Wifi Encryption` auf "-" gesetzt wird. Im alten Protokoll war
ein Payload der Größe 63 nicht möglich, dadurch wurde die maximale
Schlüssellänge auf 50 gesetzt.

Mit dem neuen Protokoll ist die volle
Schlüssellänge möglich. Da wir keine API brechen wollten, wurde diese
Funktion zusätzlich hinzugefügt.
*/
        pub async fn set_long_wifi_key(
            &mut self,
            request: [char; 64usize],
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 64usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(41u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt den Verschlüsselungsschlüssel zurück, wie von
:func:`Set Long Wifi Key` gesetzt.

.. note::
 Seit Master Brick Firmware Version 2.4.4 wird der Schlüssel nicht mehr
 zurückgegeben.
*/
        pub async fn get_long_wifi_key(
            &mut self,
        ) -> Result<[char; 64usize], tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(42u8, &payload).await?;
            Ok(<[char; 64usize]>::from_le_byte_slice(&result.body()[0..64usize]))
        }
        /**
Setzt den Hostnamen der WIFI Extension. Der Hostname wird von
Access Points als Hostname in der DHCP Client Tabelle angezeigt.

Das Setzen eines leeren Strings stellt den voreingestellten Hostnamen
wieder her.
*/
        pub async fn set_wifi_hostname(
            &mut self,
            request: [char; 16usize],
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 16usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(43u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt den Hostnamen zurück, wie von :func:`Set Wifi Hostname` gesetzt.

Ein leerer String bedeutet, dass der voreingestellte Hostname
genutzt wird.
*/
        pub async fn get_wifi_hostname(
            &mut self,
        ) -> Result<[char; 16usize], tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(44u8, &payload).await?;
            Ok(<[char; 16usize]>::from_le_byte_slice(&result.body()[0..16usize]))
        }
        /**
Setzt die Periode mit welcher der :cb:`Stack Current` Callback ausgelöst
wird. Ein Wert von 0 deaktiviert den Callback.

Der :cb:`Stack Current` Callback wird nur ausgelöst, wenn sich die Stromstärke
seit der letzten Auslösung geändert hat.
*/
        pub async fn set_stack_current_callback_period(
            &mut self,
            request: u32,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 4usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(45u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Periode zurück, wie von :func:`Set Stack Current Callback Period` gesetzt
*/
        pub async fn get_stack_current_callback_period(
            &mut self,
        ) -> Result<u32, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(46u8, &payload).await?;
            Ok(u32::from_le_byte_slice(&result.body()[0..4usize]))
        }
        /**
Setzt die Periode mit welcher der :cb:`Stack Voltage` Callback ausgelöst
wird. Ein Wert von 0 deaktiviert den Callback.

Der :cb:`Stack Voltage` Callback wird nur ausgelöst, wenn sich die Spannung seit
der letzten Auslösung geändert hat.
*/
        pub async fn set_stack_voltage_callback_period(
            &mut self,
            request: u32,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 4usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(47u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Periode zurück, wie von :func:`Set Stack Voltage Callback Period` gesetzt
*/
        pub async fn get_stack_voltage_callback_period(
            &mut self,
        ) -> Result<u32, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(48u8, &payload).await?;
            Ok(u32::from_le_byte_slice(&result.body()[0..4usize]))
        }
        /**
Setzt die Periode mit welcher der :cb:`USB Voltage` Callback ausgelöst
wird. Ein Wert von 0 deaktiviert den Callback.

Der :cb:`USB Voltage` Callback wird nur ausgelöst, wenn sich die Spannung seit
der letzten Auslösung geändert hat.
*/
        pub async fn set_usb_voltage_callback_period(
            &mut self,
            request: u32,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 4usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(49u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Periode zurück, wie von :func:`Set USB Voltage Callback Period` gesetzt
*/
        pub async fn get_usb_voltage_callback_period(
            &mut self,
        ) -> Result<u32, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(50u8, &payload).await?;
            Ok(u32::from_le_byte_slice(&result.body()[0..4usize]))
        }
        /**
Setzt den Schwellwert für den :cb:`Stack Current Reached` Callback.

Die folgenden Optionen sind möglich:

.. csv-table::
 :header: "Option", "Beschreibung"
 :widths: 10, 100

 "'x'",    "Callback ist inaktiv"
 "'o'",    "Callback wird ausgelöst, wenn die Stromstärke *außerhalb* des min und max Wertes ist"
 "'i'",    "Callback wird ausgelöst, wenn die Stromstärke *innerhalb* des min und max Wertes ist"
 "'<'",    "Callback wird ausgelöst, wenn die Stromstärke kleiner als der min Wert ist (max wird ignoriert)"
 "'>'",    "Callback wird ausgelöst, wenn die Stromstärke größer als der min Wert ist (max wird ignoriert)"
*/
        pub async fn set_stack_current_callback_threshold(
            &mut self,
            request: crate::bindings::master::SetStackCurrentCallbackThresholdRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 5usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(51u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt den Schwellwert zurück, wie von :func:`Set Stack Current Callback Threshold` gesetzt.
*/
        pub async fn get_stack_current_callback_threshold(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetStackCurrentCallbackThresholdResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(52u8, &payload).await?;
            Ok(
                crate::bindings::master::GetStackCurrentCallbackThresholdResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt den Schwellwert für den :cb:`Stack Voltage Reached` Callback.

Die folgenden Optionen sind möglich:

.. csv-table::
 :header: "Option", "Beschreibung"
 :widths: 10, 100

 "'x'",    "Callback ist inaktiv"
 "'o'",    "Callback wird ausgelöst, wenn die Spannung *außerhalb* des min und max Wertes ist"
 "'i'",    "Callback wird ausgelöst, wenn die Spannung *innerhalb* des min und max Wertes ist"
 "'<'",    "Callback wird ausgelöst, wenn die Spannung kleiner als der min Wert ist (max wird ignoriert)"
 "'>'",    "Callback wird ausgelöst, wenn die Spannung größer als der min Wert ist (max wird ignoriert)"
*/
        pub async fn set_stack_voltage_callback_threshold(
            &mut self,
            request: crate::bindings::master::SetStackVoltageCallbackThresholdRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 5usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(53u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt den Schwellwert zurück, wie von :func:`Set Stack Voltage Callback Threshold` gesetzt.
*/
        pub async fn get_stack_voltage_callback_threshold(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetStackVoltageCallbackThresholdResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(54u8, &payload).await?;
            Ok(
                crate::bindings::master::GetStackVoltageCallbackThresholdResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt den Schwellwert für den :cb:`USB Voltage Reached` Callback.

Die folgenden Optionen sind möglich:

.. csv-table::
 :header: "Option", "Beschreibung"
 :widths: 10, 100

 "'x'",    "Callback ist inaktiv"
 "'o'",    "Callback wird ausgelöst, wenn die Spannung *außerhalb* des min und max Wertes ist"
 "'i'",    "Callback wird ausgelöst, wenn die Spannung *innerhalb* des min und max Wertes ist"
 "'<'",    "Callback wird ausgelöst, wenn die Spannung kleiner als der min Wert ist (max wird ignoriert)"
 "'>'",    "Callback wird ausgelöst, wenn die Spannung größer als der min Wert ist (max wird ignoriert)"
*/
        pub async fn set_usb_voltage_callback_threshold(
            &mut self,
            request: crate::bindings::master::SetUsbVoltageCallbackThresholdRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 5usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(55u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt den Schwellwert zurück, wie von :func:`Set USB Voltage Callback Threshold` gesetzt.
*/
        pub async fn get_usb_voltage_callback_threshold(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetUsbVoltageCallbackThresholdResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(56u8, &payload).await?;
            Ok(
                crate::bindings::master::GetUsbVoltageCallbackThresholdResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt die Periode mit welcher die Schwellwert Callbacks

* :cb:`Stack Current Reached`,
* :cb:`Stack Voltage Reached`,
* :cb:`USB Voltage Reached`

ausgelöst werden, wenn die Schwellwerte

* :func:`Set Stack Current Callback Threshold`,
* :func:`Set Stack Voltage Callback Threshold`,
* :func:`Set USB Voltage Callback Threshold`

weiterhin erreicht bleiben.
*/
        pub async fn set_debounce_period(
            &mut self,
            request: u32,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 4usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(57u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Entprellperiode zurück, wie von :func:`Set Debounce Period` gesetzt.
*/
        pub async fn get_debounce_period(
            &mut self,
        ) -> Result<u32, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(58u8, &payload).await?;
            Ok(u32::from_le_byte_slice(&result.body()[0..4usize]))
        }
        /**
Dieser Callback wird mit der Periode, wie gesetzt mit
:func:`Set Stack Current Callback Period`, ausgelöst. Der :word:`parameter` ist
die Stromstärke des Sensors.

Der :cb:`Stack Current` Callback wird nur ausgelöst, wenn sich die Stromstärke
seit der letzten Auslösung geändert hat.
*/
        pub async fn stack_current_stream(
            &mut self,
        ) -> impl futures_core::Stream<Item = u16> {
            self.device
                .get_callback_receiver(59u8)
                .await
                .map(|p| u16::from_le_byte_slice(&p.body()[0..2usize]))
        }
        /**
Dieser Callback wird mit der Periode, wie gesetzt mit
:func:`Set Stack Voltage Callback Period`, ausgelöst. Der :word:`parameter`
ist die Spannung des Sensors.

Der :cb:`Stack Voltage` Callback wird nur ausgelöst, wenn sich die Spannung seit
der letzten Auslösung geändert hat.
*/
        pub async fn stack_voltage_stream(
            &mut self,
        ) -> impl futures_core::Stream<Item = u16> {
            self.device
                .get_callback_receiver(60u8)
                .await
                .map(|p| u16::from_le_byte_slice(&p.body()[0..2usize]))
        }
        /**
Dieser Callback wird mit der Periode, wie gesetzt mit
:func:`Set USB Voltage Callback Period`, ausgelöst. Der :word:`parameter` ist
die USB Spannung.

Der :cb:`USB Voltage` Callback wird nur ausgelöst, wenn sich die USB Spannung
seit der letzten Auslösung geändert hat.

Funktioniert nicht mit Hardware Version 2.1 oder neuer.
*/
        pub async fn usb_voltage_stream(
            &mut self,
        ) -> impl futures_core::Stream<Item = u16> {
            self.device
                .get_callback_receiver(61u8)
                .await
                .map(|p| u16::from_le_byte_slice(&p.body()[0..2usize]))
        }
        /**
Dieser Callback wird ausgelöst, wenn der Schwellwert, wie von
:func:`Set Stack Current Callback Threshold` gesetzt, erreicht wird.
Der :word:`parameter` ist der Stromverbrauch des Stapels.

Wenn der Schwellwert erreicht bleibt, wird der Callback mit der Periode, wie
mit :func:`Set Debounce Period` gesetzt, ausgelöst.
*/
        pub async fn stack_current_reached_stream(
            &mut self,
        ) -> impl futures_core::Stream<Item = u16> {
            self.device
                .get_callback_receiver(62u8)
                .await
                .map(|p| u16::from_le_byte_slice(&p.body()[0..2usize]))
        }
        /**
Dieser Callback wird ausgelöst, wenn der Schwellwert, wie von
:func:`Set Stack Voltage Callback Threshold` gesetzt, erreicht wird.
Der :word:`parameter` ist die Spannung des Stapels.

Wenn der Schwellwert erreicht bleibt, wird der Callback mit der Periode, wie
mit :func:`Set Debounce Period` gesetzt, ausgelöst.
*/
        pub async fn stack_voltage_reached_stream(
            &mut self,
        ) -> impl futures_core::Stream<Item = u16> {
            self.device
                .get_callback_receiver(63u8)
                .await
                .map(|p| u16::from_le_byte_slice(&p.body()[0..2usize]))
        }
        /**
Dieser Callback wird ausgelöst, wenn der Schwellwert, wie von
:func:`Set USB Voltage Callback Threshold` gesetzt, erreicht wird.
Der :word:`parameter` ist die Spannung des Sensors.

Wenn der Schwellwert erreicht bleibt, wird der Callback mit der Periode, wie
mit :func:`Set Debounce Period` gesetzt, ausgelöst.
*/
        pub async fn usb_voltage_reached_stream(
            &mut self,
        ) -> impl futures_core::Stream<Item = u16> {
            self.device
                .get_callback_receiver(64u8)
                .await
                .map(|p| u16::from_le_byte_slice(&p.body()[0..2usize]))
        }
        /**
Gibt *true* zurück, wenn der Master Brick an Position 0 im Stapel und eine
Ethernet Extension verfügbar ist.
*/
        pub async fn is_ethernet_present(
            &mut self,
        ) -> Result<bool, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(65u8, &payload).await?;
            Ok(bool::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Setzt die Konfiguration der Ethernet Extension. Mögliche Werte für
``connection`` sind:

.. csv-table::
 :header: "Wert", "Beschreibung"
 :widths: 10, 90

 "0", "DHCP"
 "1", "Statische IP"

Wenn ``connection`` auf die statische IP Option gesetzt wird, dann müssen
``ip``, ``subnet_mask`` und ``gateway`` als ein Array der Größe 4 angegeben
werden. Dabei ist das erste Element im Array das niederwertigste Byte. Falls
``connection`` auf die DHCP Option gesetzt ist, werden ``ip``, ``subnet_mask``
und ``gateway`` ignoriert.

Der letzte Parameter ist der Port auf den das Anwendungsprogramm sich
verbindet.

Die Werte sind im EEPROM gespeichert und werden nur beim Start angewandt.
Das bedeutet der Master Brick muss nach einer Konfiguration neu gestartet
werden.

Wir empfehlen den Brick Viewer zu verwenden, um die Ethernet Extension zu
konfigurieren.
*/
        pub async fn set_ethernet_configuration(
            &mut self,
            request: crate::bindings::master::SetEthernetConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 15usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(66u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Konfiguration zurück, wie von :func:`Set Ethernet Configuration` gesetzt.
*/
        pub async fn get_ethernet_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetEthernetConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(67u8, &payload).await?;
            Ok(
                crate::bindings::master::GetEthernetConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Gibt den Status der Ethernet Extension zurück.

``mac_address``, ``ip``, ``subnet_mask`` und ``gateway`` werden als Array
übergeben. Das erste Element des Arrays ist das niederwertigste Byte.

``rx_count`` und ``tx_count`` sind die Anzahl der Bytes die seit dem letzten
Neustart empfangen/gesendet wurden.

``hostname`` ist der aktuell genutzte Hostname.
*/
        pub async fn get_ethernet_status(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetEthernetStatusResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(68u8, &payload).await?;
            Ok(
                crate::bindings::master::GetEthernetStatusResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt den Hostnamen der Ethernet Extension. Der Hostname wird von
Access Points als Hostname in der DHCP Client Tabelle angezeigt.

Das setzen eines leeren Strings stellt den voreingestellten Hostnamen
wieder her.

Der aktuelle Hostname kann mit :func:`Get Ethernet Status` herausgefunden werden.
*/
        pub async fn set_ethernet_hostname(
            &mut self,
            request: [char; 32usize],
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 32usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(69u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Setzt die MAC Adresse der Ethernet Extension. Die Ethernet Extension sollte
mit einer vorkonfigurierten MAC Adresse ausgeliefert werden. Diese MAC Adresse
steht auch auf einem Aufkleber auf der Ethernet Extension.

Die MAC Adresse kann mit :func:`Get Ethernet Status` wieder ausgelesen werden.
*/
        pub async fn set_ethernet_mac_address(
            &mut self,
            request: [u8; 6usize],
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 6usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(70u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Setzt die Ethernet WebSocket-Konfiguration. Der erste Parameter setzt
die Anzahl der Socket-Verbindungen die für WebSockets reserviert werden.
Der mögliche Wertebereich ist 0-7. Die Verbindungen werden zwischen den
normalen Sockets und den WebSockets aufgeteilt. Beispiel: Wenn die Socket-Verbindungen auf 3
gesetzt werden, stehen 3 WebSockets und 4 normale Sockets zur Verfügung.

Der zweite Parameter ist der Port für die WebSocket-Verbindungen. Der Port
kann nicht der gleiche sein wie der Port des normalen Sockets.

Die Werte sind im EEPROM gespeichert und werden nur beim Start angewandt.
Das bedeutet der Master Brick muss nach einer Konfiguration neu gestartet
werden.

Wir empfehlen den Brick Viewer zu verwenden, um die Ethernet Extension zu
konfigurieren.
*/
        pub async fn set_ethernet_websocket_configuration(
            &mut self,
            request: crate::bindings::master::SetEthernetWebsocketConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 3usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(71u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Konfiguration zurück, wie von :func:`Set Ethernet Configuration` gesetzt.
*/
        pub async fn get_ethernet_websocket_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetEthernetWebsocketConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(72u8, &payload).await?;
            Ok(
                crate::bindings::master::GetEthernetWebsocketConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt das Authentifizierungsgeheimnis. Das Geheimnis ist ein String aus bis zu
64 Buchstaben. Ein leerer String deaktiviert die Authentifizierung.

Für mehr Informationen zur Authentifizierung siehe das dazugehörige
:ref:`Tutorial <tutorial_authentication>`.

Das Authentifizierungsgehemnis wird im EEPROM gespeichert und nur beim Start angewandt.
Das bedeutet der Master Brick muss nach einer Konfiguration neu gestartet
werden.

Wir empfehlen den Brick Viewer zu verwenden, um die Authentifizierung der Ethernet
Extension einzurichten.

Der Standardwert ist ein leerer String (Authentifizierung deaktiviert).
*/
        pub async fn set_ethernet_authentication_secret(
            &mut self,
            request: [char; 64usize],
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 64usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(73u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt das Authentifizierungsgeheimnis zurück, wie von
:func:`Set Ethernet Authentication Secret` gesetzt.
*/
        pub async fn get_ethernet_authentication_secret(
            &mut self,
        ) -> Result<[char; 64usize], tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(74u8, &payload).await?;
            Ok(<[char; 64usize]>::from_le_byte_slice(&result.body()[0..64usize]))
        }
        /**
Setzt das Authentifizierungsgeheimnis. Das Geheimnis ist ein String aus bis zu
64 Buchstaben. Ein leerer String deaktiviert die Authentifizierung.

Für mehr Informationen zur Authentifizierung siehe das dazugehörige
:ref:`Tutorial <tutorial_authentication>`.

Das Authentifizierungsgehemnis wird im EEPROM gespeichert und nur beim Start
angewandt. Das bedeutet der Master Brick muss nach einer Konfiguration neu
gestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die Authentifizierung der WIFI
Extension einzurichten.

Der Standardwert ist ein leerer String (Authentifizierung deaktiviert).
*/
        pub async fn set_wifi_authentication_secret(
            &mut self,
            request: [char; 64usize],
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 64usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(75u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt das Authentifizierungsgeheimnis zurück, wie von
:func:`Set Wifi Authentication Secret` gesetzt.
*/
        pub async fn get_wifi_authentication_secret(
            &mut self,
        ) -> Result<[char; 64usize], tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(76u8, &payload).await?;
            Ok(<[char; 64usize]>::from_le_byte_slice(&result.body()[0..64usize]))
        }
        /**
Gibt den Typ der Verbingung zurück, über welche diese Funktion aufgerufen wurde.
*/
        pub async fn get_connection_type(
            &mut self,
        ) -> Result<
            tinkerforge_base::byte_converter::ParsedOrRaw<
                crate::bindings::master::ConnectionType,
                u8,
            >,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(77u8, &payload).await?;
            Ok(
                tinkerforge_base::byte_converter::ParsedOrRaw::<
                    crate::bindings::master::ConnectionType,
                    u8,
                >::from_le_byte_slice(&result.body()[0..1usize]),
            )
        }
        /**
Gibt *true* zurück, wenn der Master Brick an Position 0 im Stapel und eine
WIFI Extension 2.0 verfügbar ist.
*/
        pub async fn is_wifi_2_present(
            &mut self,
        ) -> Result<bool, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(78u8, &payload).await?;
            Ok(bool::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Startet den Bootloader der WIFI Extension 2.0. Gibt bei Erfolg 0 zurück.
Danach können die :func:`Write Wifi2 Serial Port` und :func:`Read Wifi2 Serial Port`
Funktionen zur Kommunikation mit dem Bootloader verwendet werden, um eine neue
Firmware zu flashen.

Der Bootloader sollte nur über eine USB Verbindung gestartet werden. Er kann
nicht über eine WIFI2 Verbindung gestartet werden, siehe die
:func:`Get Connection Type` Funktion.

Wir empfehlen den Brick Viewer zu verwenden, um die Firmware der WIFI
Extension 2.0 zu aktualisieren.
*/
        pub async fn start_wifi_2_bootloader(
            &mut self,
        ) -> Result<i8, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(79u8, &payload).await?;
            Ok(i8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Schreibt bis zu 60 Bytes (Anzahl zu schreibender Bytes mit ``length`` angeben)
auf die serielle Schnittstelle des Bootloaders der WIFI Extension 2.0. Gibt
bei Erfolg 0 zurück.

Bevor diese Funktion genutzt werden kann muss der Bootloader mit der
:func:`Start Wifi2 Bootloader` Funktion gestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die Firmware der WIFI
Extension 2.0 zu aktualisieren.
*/
        pub async fn write_wifi_2_serial_port(
            &mut self,
            request: crate::bindings::master::WriteWifi2SerialPortRequest,
        ) -> Result<i8, tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 61usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(80u8, &payload).await?;
            Ok(i8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Liest bis zu 60 Bytes (Anzahl zu lesender Bytes mit ``length`` angegeben) von
der seriellen Schnittstelle des Bootloaders der WIFI Extension 2.0. Gibt die
Anzahl der wirklich gelesenen Bytes zurück.

Bevor diese Funktion genutzt werden kann muss der Bootloader mit der
:func:`Start Wifi2 Bootloader` Funktion gestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die Firmware der WIFI
Extension 2.0 zu aktualisieren.
*/
        pub async fn read_wifi_2_serial_port(
            &mut self,
            request: u8,
        ) -> Result<
            crate::bindings::master::ReadWifi2SerialPortResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(81u8, &payload).await?;
            Ok(
                crate::bindings::master::ReadWifi2SerialPortResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt das WLAN-Authentifizierungsgeheimnis. Das Geheimnis ist ein String aus
bis zu 64 Buchstaben. Ein leerer String deaktiviert die Authentifizierung. Der
Standardwert ist ein leerer String (Authentifizierung deaktiviert).

Für mehr Informationen zur Authentifizierung siehe das dazugehörige
:ref:`Tutorial <tutorial_authentication>`.

Um Konfigurationsänderungen für die WIFI Extension 2.0 zu übernehmen muss die
:func:`Save Wifi2 Configuration` Funktion aufgerufen und der Master Brick
danach neugestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die WIFI Extension 2.0 zu
konfigurieren.
*/
        pub async fn set_wifi_2_authentication_secret(
            &mut self,
            request: [char; 64usize],
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 64usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(82u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt das WLAN-Authentifizierungsgeheimnis zurück, wie von
:func:`Set Wifi2 Authentication Secret` gesetzt.
*/
        pub async fn get_wifi_2_authentication_secret(
            &mut self,
        ) -> Result<[char; 64usize], tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(83u8, &payload).await?;
            Ok(<[char; 64usize]>::from_le_byte_slice(&result.body()[0..64usize]))
        }
        /**
Setzt die allgemeine Konfiguration der WIFI Extension 2.0.

Der ``port`` Parameter setzt die Portnummer auf die sich das Anwendungsprogramm
verbindet.

Der ``websocket_port`` Parameter setzt die WebSocket-Portnummer auf die sich das
JavaScript Anwendungsprogramm verbindet.

Der ``website_port`` Parameter setzt die Portnummer für die Webseite der
WIFI Extension 2.0.

Der ``phy_mode`` Parameter setzt den zu verwendenden WLAN-Modus. Mögliche Werte
sinf B, G und N.

Die ``sleep_mode`` und ``website`` Parameter werden momentan nicht verwendet.

Um Konfigurationsänderungen für die WIFI Extension 2.0 zu übernehmen muss die
:func:`Save Wifi2 Configuration` Funktion aufgerufen und der Master Brick
danach neugestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die WIFI Extension 2.0 zu
konfigurieren.
*/
        pub async fn set_wifi_2_configuration(
            &mut self,
            request: crate::bindings::master::SetWifi2ConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 9usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(84u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die allgemeine Konfiguration zurück, wie von :func:`Set Wifi2 Configuration` gesetzt.
*/
        pub async fn get_wifi_2_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifi2ConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(85u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifi2ConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Gibt den Client und Access Point Status der WIFI Extension 2.0 zurück.
*/
        pub async fn get_wifi_2_status(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifi2StatusResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(86u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifi2StatusResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt die Client-spezifische Konfiguration der WIFI Extension 2.0.

Der ``enable`` Parameter aktiviert oder deaktiviert den Client-Teil der
WIFI Extension 2.0. Der Standardwert ist *true*.

Der ``ssid`` Parameter die SSID (bis zu 32 Zeichen) des Access Points zu dem
die WLAN Verbindung hergestellt werden soll.

Wenn die ``ip``, ``subnet_mask`` und ``gateway`` Parameter alle auf Null gesetzt
sind, dann wird DHCP verwendet. Andernfalls kann mit diese drei Parametern eine
statische IP Adresse eingestellt werden. Die Standardeinstellung ist DHCP.

Wenn der ``mac_address`` Parameter auf Null gesetzt ist, dann wird die
voreingestellt MAC Adresse verwendet. Andernfalls kann mit diesem Parameter
eine eigene MAC Adresse eingestellt werden.

Wenn der ``bssid`` Parameter auf Null gesetzt ist, dann verbindet sich die
WIFI Extension 2.0 mit einem Access Point wenn die eingestellt SSID
übereinstimmt. Andernfalls kann dieses Parameter verwendet werden, damit sich
die WIFI Extension 2.0 nur dann mit einem Access Point verbindet, wenn SSID
und BSSID übereinstimmen.

Um Konfigurationsänderungen für die WIFI Extension 2.0 zu übernehmen muss die
:func:`Save Wifi2 Configuration` Funktion aufgerufen und der Master Brick
danach neugestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die WIFI Extension 2.0 zu
konfigurieren.
*/
        pub async fn set_wifi_2_client_configuration(
            &mut self,
            request: crate::bindings::master::SetWifi2ClientConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 57usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(87u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Client Konfiguration zurück, wie von
:func:`Set Wifi2 Client Configuration` gesetzt.
*/
        pub async fn get_wifi_2_client_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifi2ClientConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(88u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifi2ClientConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt den Client Hostnamen (bis zu 32 Zeichen) der WIFI Extension 2.0. Der
Hostname wird von Access Points als Hostname in der DHCP Client Tabelle angezeigt.

Um Konfigurationsänderungen für die WIFI Extension 2.0 zu übernehmen muss die
:func:`Save Wifi2 Configuration` Funktion aufgerufen und der Master Brick
danach neugestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die WIFI Extension 2.0 zu
konfigurieren.
*/
        pub async fn set_wifi_2_client_hostname(
            &mut self,
            request: [char; 32usize],
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 32usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(89u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt den Client Hostnamen zurück, wie von :func:`Set Wifi2 Client Hostname` gesetzt.
*/
        pub async fn get_wifi_2_client_hostname(
            &mut self,
        ) -> Result<[char; 32usize], tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(90u8, &payload).await?;
            Ok(<[char; 32usize]>::from_le_byte_slice(&result.body()[0..32usize]))
        }
        /**
Setzt das Client-Passwort (bis zu 63 Zeichen) für WPA/WPA2 Verschlüsselung.

Um Konfigurationsänderungen für die WIFI Extension 2.0 zu übernehmen muss die
:func:`Save Wifi2 Configuration` Funktion aufgerufen und der Master Brick
danach neugestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die WIFI Extension 2.0 zu
konfigurieren.
*/
        pub async fn set_wifi_2_client_password(
            &mut self,
            request: [char; 64usize],
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 64usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(91u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt das Client-Passwort zurück, wie von :func:`Set Wifi2 Client Password` gesetzt.

.. note::
 Seit WIFI Extension 2.0 Firmware Version 2.1.3 wird das Passwort
 nicht mehr zurückgegeben.
*/
        pub async fn get_wifi_2_client_password(
            &mut self,
        ) -> Result<[char; 64usize], tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(92u8, &payload).await?;
            Ok(<[char; 64usize]>::from_le_byte_slice(&result.body()[0..64usize]))
        }
        /**
Setzt die Access-Point-spezifische Konfiguration der WIFI Extension 2.0.

Der ``enable`` Parameter aktiviert oder deaktiviert den Access-Point-Teil der
WIFI Extension 2.0. Der Standardwert ist *true*.

Der ``ssid`` Parameter die SSID (bis zu 32 Zeichen) des Access Points.

Wenn die ``ip``, ``subnet_mask`` und ``gateway`` Parameter alle auf Null gesetzt
sind, dann wird ein DHCP Server aktiviert. Andernfalls kann mit diese drei
Parametern eine statische IP Adresse eingestellt werden. Die Standardeinstellung
ist DHCP.

Der ``encryption`` Parameter legt den Verschlüsselungsmodus fest. Mögliche Werte
sind Open (keine  Verschlüsselung), WEP oder WPA/WPA2 PSK.
Mit der :func:`Set Wifi2 AP Password` Kann das
Verschlüsselungspasswort gesetzt werden.

Der ``hidden`` Parameter legt fest, oder der Access Point seine SSID versteckt
oder zeigt.

Der ``channel`` Parameter gibt den Kanal (1 to 13) des Access Points and.

Wenn der ``mac_address`` Parameter auf Null gesetzt ist, dann wird die
voreingestellt MAC Adresse verwendet. Andernfalls kann mit diesem Parameter
eine eigene MAC Adresse eingestellt werden.

Um Konfigurationsänderungen für die WIFI Extension 2.0 zu übernehmen muss die
:func:`Save Wifi2 Configuration` Funktion aufgerufen und der Master Brick
danach neugestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die WIFI Extension 2.0 zu
konfigurieren.
*/
        pub async fn set_wifi_2_ap_configuration(
            &mut self,
            request: crate::bindings::master::SetWifi2ApConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 54usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(93u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Access-Point-Konfiguration zurück, wie von
:func:`Set Wifi2 AP Configuration` gesetzt.
*/
        pub async fn get_wifi_2_ap_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifi2ApConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(94u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifi2ApConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt das Access-Point-Passwort (mindestens 8 und bis zu 63 Zeichen) für den eingestellten
Verschlüsselungsmodus, siehe :func:`Set Wifi2 AP Configuration`.

Um Konfigurationsänderungen für die WIFI Extension 2.0 zu übernehmen muss die
:func:`Save Wifi2 Configuration` Funktion aufgerufen und der Master Brick
danach neugestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die WIFI Extension 2.0 zu
konfigurieren.
*/
        pub async fn set_wifi_2_ap_password(
            &mut self,
            request: [char; 64usize],
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 64usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(95u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt das Access-Point-Passwort zurück, wie von :func:`Set Wifi2 AP Password` gesetzt.

.. note::
 Seit WIFI Extension 2.0 Firmware Version 2.1.3 wird das Passwort
 nicht mehr zurückgegeben.
*/
        pub async fn get_wifi_2_ap_password(
            &mut self,
        ) -> Result<[char; 64usize], tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(96u8, &payload).await?;
            Ok(<[char; 64usize]>::from_le_byte_slice(&result.body()[0..64usize]))
        }
        /**
Alle Konfigurationsfunktionen der WIFI Extension 2.0 ändern die Werte nicht
dauerhaft. Nach einer Konfiguration muss diese Funktion aufgerufen werden, um
die Werte dauerhaft zu speichern.

Die Werte sind im EEPROM gespeichert und werden nur beim Start angewandt.
Das bedeutet der Master Brick muss nach einer Konfiguration neu gestartet
werden.
*/
        pub async fn save_wifi_2_configuration(
            &mut self,
        ) -> Result<u8, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(97u8, &payload).await?;
            Ok(u8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Gibt die aktuelle Version der WIFI Extension 2.0 Firmware zurück.
*/
        pub async fn get_wifi_2_firmware_version(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifi2FirmwareVersionResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(98u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifi2FirmwareVersionResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Aktiviert die grüne Status LED der WIFI Extension 2.0.
*/
        pub async fn enable_wifi_2_status_led(
            &mut self,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            self.device
                .set(99u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Deaktiviert die grüne Status LED der WIFI Extension 2.0.
*/
        pub async fn disable_wifi_2_status_led(
            &mut self,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            self.device
                .set(100u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt *true* zurück falls die grüne Status LED der WIFI Extension 2.0 aktiviert ist.
*/
        pub async fn is_wifi_2_status_led_enabled(
            &mut self,
        ) -> Result<bool, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(101u8, &payload).await?;
            Ok(bool::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Benötigt WIFI Extension 2.0 Firmware 2.1.0.

Set die Mesh-Konfiguration der WIFI Extension 2.0.

Der ``enable`` Parameter aktiviert oder deaktiviert den Mesh-Teil der
WIFI Extension 2.0. Der Mesh-Teil kann nicht
zusammen mit dem Client- und Access-Point-Teil aktiviert werden.

Wenn die ``root_ip``, ``root_subnet_mask`` und ``root_gateway`` Parameter alle
auf Null gesetzt sind, dann wird DHCP verwendet. Andernfalls kann mit diese
drei Parametern eine statische IP Adresse eingestellt werden. Die
Standardeinstellung ist DHCP.

Wenn der ``router_bssid`` Parameter auf Null gesetzt ist, dann verbindet sich
die WIFI Extension 2.0 mit einem Access Point wenn die eingestellt SSID
übereinstimmt, siehe :func:`Set Wifi2 Mesh Router SSID`. Andernfalls kann dieses
Parameter verwendet werden, damit sich die WIFI Extension 2.0 nur dann mit
einem Access Point verbindet, wenn SSID und BSSID übereinstimmen. Die BSSID
kann auch verwendet werden, um eine Verbindung mit einer verstecken SSID
herzustellen.

Die ``group_id`` und ``group_ssid_prefix`` Parameter identifizieren in bestimmtes
Mesh-Netzwerk und alle WIFI Extension 2.0 mit der gleichen Gruppeneinstellung
gehören um gleichen Mesh-Netzwerk.

Die ``gateway_ip`` und ``gateway_port`` Parameter geben an, wie der Mesh-Gateway
(brickd) erreicht werden kann.

Um Konfigurationsänderungen für die WIFI Extension 2.0 zu übernehmen muss die
:func:`Save Wifi2 Configuration` Funktion aufgerufen und der Master Brick
danach neugestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die WIFI Extension 2.0 zu
konfigurieren.
*/
        pub async fn set_wifi_2_mesh_configuration(
            &mut self,
            request: crate::bindings::master::SetWifi2MeshConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 47usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(102u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Benötigt WIFI Extension 2.0 Firmware 2.1.0.

Gibt das Mesh Konfiguration zurück, wie von :func:`Set Wifi2 Mesh Configuration` gesetzt.
*/
        pub async fn get_wifi_2_mesh_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifi2MeshConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(103u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifi2MeshConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Benötigt WIFI Extension 2.0 Firmware 2.1.0.

Setzt die Mesh-Router-SSID der WIFI Extension 2.0. Diese wird verwendet um den
Mesh Router festzulegen.

Zu beachten ist, dass zwar 32 Zeichen als SSID übergeben werden können, aber im
Moment davon nur die ersten 31 Zeichen genutzt werden bedingt durch einen Bug
in der verwendeten Mesh-Bibliothek.

Um Konfigurationsänderungen für die WIFI Extension 2.0 zu übernehmen muss die
:func:`Save Wifi2 Configuration` Funktion aufgerufen und der Master Brick
danach neugestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die WIFI Extension 2.0 zu
konfigurieren.
*/
        pub async fn set_wifi_2_mesh_router_ssid(
            &mut self,
            request: [char; 32usize],
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 32usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(104u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Benötigt WIFI Extension 2.0 Firmware 2.1.0.

Gibt das Mesh-Router-SSID zurück, wie von :func:`Set Wifi2 Mesh Router SSID` gesetzt.
*/
        pub async fn get_wifi_2_mesh_router_ssid(
            &mut self,
        ) -> Result<[char; 32usize], tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(105u8, &payload).await?;
            Ok(<[char; 32usize]>::from_le_byte_slice(&result.body()[0..32usize]))
        }
        /**
Benötigt WIFI Extension 2.0 Firmware 2.1.0.

Setzt das Mesh-Router-Passwort (bis zu 64 Zeichen) für WPA/WPA2 Verschlüsselung.
Das Password wird für die Verbindung zum Mesh Router verwendet.

Um Konfigurationsänderungen für die WIFI Extension 2.0 zu übernehmen muss die
:func:`Save Wifi2 Configuration` Funktion aufgerufen und der Master Brick
danach neugestartet werden.

Wir empfehlen den Brick Viewer zu verwenden, um die WIFI Extension 2.0 zu
konfigurieren.
*/
        pub async fn set_wifi_2_mesh_router_password(
            &mut self,
            request: [char; 64usize],
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 64usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(106u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Benötigt WIFI Extension 2.0 Firmware 2.1.0.

Gibt das Mesh-Router-Password zurück, wie von :func:`Set Wifi2 Mesh Router Password` gesetzt.
*/
        pub async fn get_wifi_2_mesh_router_password(
            &mut self,
        ) -> Result<[char; 64usize], tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(107u8, &payload).await?;
            Ok(<[char; 64usize]>::from_le_byte_slice(&result.body()[0..64usize]))
        }
        /**
Benötigt WIFI Extension 2.0 Firmware 2.1.0.

Gibt den allgemeinen Mesh-Status der WIFI Extension 2.0 zurück.
*/
        pub async fn get_wifi_2_mesh_common_status(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifi2MeshCommonStatusResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(108u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifi2MeshCommonStatusResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Benötigt WIFI Extension 2.0 Firmware 2.1.0.

Gibt den Mesh-Client-Status der WIFI Extension 2.0 zurück.
*/
        pub async fn get_wifi_2_mesh_client_status(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifi2MeshClientStatusResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(109u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifi2MeshClientStatusResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Benötigt WIFI Extension 2.0 Firmware 2.1.0.

Gibt den Mesh-AP-Status der WIFI Extension 2.0 zurück.
*/
        pub async fn get_wifi_2_mesh_ap_status(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetWifi2MeshApStatusResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(110u8, &payload).await?;
            Ok(
                crate::bindings::master::GetWifi2MeshApStatusResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Diese Funktion wird Tinkerforge-Intern genutzt um initial den
Bootstrapper und Bootloader auf Bricklets zu flashen.

Falls die Notwendigkeit besteht einen Bootstrapper/Bootloader
zu flashen (zum Beispiel weil ein eigenes Bricklet entwickelet
wurde) bitte nicht diese Funktion direkt benutzen.

Dafür kann unser Open Source Flash/Test-Tool genutzt werden:
`https://github.com/Tinkerforge/flash-test <https://github.com/Tinkerforge/flash-test>`__
*/
        pub async fn set_bricklet_xmc_flash_config(
            &mut self,
            request: crate::bindings::master::SetBrickletXmcFlashConfigRequest,
        ) -> Result<
            crate::bindings::master::SetBrickletXmcFlashConfigResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 64usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(111u8, &payload).await?;
            Ok(
                crate::bindings::master::SetBrickletXmcFlashConfigResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Diese Funktion wird Tinkerforge-Intern genutzt um initial den
Bootstrapper und Bootloader auf Bricklets zu flashen.

Falls die Notwendigkeit besteht einen Bootstrapper/Bootloader
zu flashen (zum Beispiel weil ein eigenes Bricklet entwickelet
wurde) bitte nicht diese Funktion direkt benutzen.

Dafür kann unser Open Source Flash/Test-Tool genutzt werden:
`https://github.com/Tinkerforge/flash-test <https://github.com/Tinkerforge/flash-test>`__
*/
        pub async fn set_bricklet_xmc_flash_data(
            &mut self,
            request: [u8; 64usize],
        ) -> Result<u32, tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 64usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(112u8, &payload).await?;
            Ok(u32::from_le_byte_slice(&result.body()[0..4usize]))
        }
        #[doc = "\nDiese Funktion ist nur nur Master Brick Hardware Version >= 3.0 verfügbar.\n\nAktiviert/Deaktiviert alle vier Bricklets wenn auf *true*/*false* gesetzt.\n\nWenn die Bricklets deaktiviert werden, wird die Stromversorgung zu den Bricklets getrennt.\nDie Bricklets verlieren dabei ihre aktuelle konfiguration.\n"]
        pub async fn set_bricklets_enabled(
            &mut self,
            request: bool,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(113u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt *true* zurück wenn die Bricklets aktiviert sind und *false* wenn sie deaktiviert sind.
*/
        pub async fn get_bricklets_enabled(
            &mut self,
        ) -> Result<bool, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(114u8, &payload).await?;
            Ok(bool::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Das SPITF-Protokoll kann mit einer dynamischen Baudrate genutzt werden. Wenn die dynamische
Baudrate aktiviert ist, versucht der Brick die Baudrate anhand des Datenaufkommens
zwischen Brick und Bricklet anzupassen.

Die Baudrate wird exponentiell erhöht wenn viele Daten gesendet/empfangen werden
und linear verringert wenn wenig Daten gesendet/empfangen werden.

Diese Vorgehensweise verringert die Baudrate in Anwendungen wo nur wenig Daten
ausgetauscht werden müssen (z.B. eine Wetterstation) und erhöht die Robustheit.
Wenn immer viele Daten ausgetauscht werden (z.B. Thermal Imaging Bricklet), wird
die Baudrate automatisch erhöht.

In Fällen wo wenige Daten all paar Sekunden so schnell wie Möglich übertragen werden
sollen (z.B. RS485 Bricklet mit hoher Baudrate aber kleinem Payload) kann die
dynamische Baudrate zum maximieren der Performance ausgestellt werden.

Die maximale Baudrate kann pro Port mit der Funktion :func:`Set SPITFP Baudrate`.
gesetzt werden. Falls die dynamische Baudrate nicht aktiviert ist, wird die Baudrate
wie von :func:`Set SPITFP Baudrate` gesetzt statisch verwendet.
*/
        pub async fn set_spitfp_baudrate_config(
            &mut self,
            request: crate::bindings::master::SetSpitfpBaudrateConfigRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 5usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(231u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Baudratenkonfiguration zurück, siehe :func:`Set SPITFP Baudrate Config`.
*/
        pub async fn get_spitfp_baudrate_config(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetSpitfpBaudrateConfigResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(232u8, &payload).await?;
            Ok(
                crate::bindings::master::GetSpitfpBaudrateConfigResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Gibt den Timeout-Zähler für die verschiedenen Kommunikationsmöglichkeiten zurück

Die Kommunikationsmöglichkeiten 0-2 stehen auf allen Bricks zur verfügung, 3-7 nur auf Master Bricks.

Diese Funktion ist hauptsächlich zum debuggen während der Entwicklung gedacht.
Im normalen Betrieb sollten alle Zähler fast immer auf 0 stehen bleiben.
*/
        pub async fn get_send_timeout_count(
            &mut self,
            request: crate::bindings::master::CommunicationMethod,
        ) -> Result<u32, tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(233u8, &payload).await?;
            Ok(u32::from_le_byte_slice(&result.body()[0..4usize]))
        }
        /**
Setzt die Baudrate eines spezifischen Bricklet Ports .

Für einen höheren Durchsatz der Bricklets kann die Baudrate erhöht werden.
Wenn der Fehlerzähler auf Grund von lokaler Störeinstrahlung hoch ist
(siehe :func:`Get SPITFP Error Count`) kann die Baudrate verringert werden.

Wenn das Feature der dynamische Baudrate aktiviert ist, setzt diese Funktion
die maximale Baudrate (siehe :func:`Set SPITFP Baudrate Config`).

EMV Tests werden mit der Standardbaudrate durchgeführt. Falls eine
CE-Kompatibilität o.ä. in der Anwendung notwendig ist empfehlen wir die
Baudrate nicht zu ändern.
*/
        pub async fn set_spitfp_baudrate(
            &mut self,
            request: crate::bindings::master::SetSpitfpBaudrateRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 5usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(234u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Baudrate für einen Bricklet Port zurück, siehe
:func:`Set SPITFP Baudrate`.
*/
        pub async fn get_spitfp_baudrate(
            &mut self,
            request: char,
        ) -> Result<u32, tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(235u8, &payload).await?;
            Ok(u32::from_le_byte_slice(&result.body()[0..4usize]))
        }
        /**
Gibt die Anzahl der Fehler die während der Kommunikation zwischen Brick und
Bricklet aufgetreten sind zurück.

Die Fehler sind aufgeteilt in

* ACK-Checksummen Fehler,
* Message-Checksummen Fehler,
* Framing Fehler und
* Overflow Fehler.

Die Fehlerzähler sind für Fehler die auf der Seite des Bricks auftreten.
Jedes Bricklet hat eine ähnliche Funktion welche die Fehler auf Brickletseite
ausgibt.
*/
        pub async fn get_spitfp_error_count(
            &mut self,
            request: char,
        ) -> Result<
            crate::bindings::master::GetSpitfpErrorCountResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(237u8, &payload).await?;
            Ok(
                crate::bindings::master::GetSpitfpErrorCountResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Aktiviert die Status LED.

Die Status LED ist die blaue LED neben dem USB-Stecker. Wenn diese aktiviert
ist, ist sie an und sie flackert wenn Daten transferiert werden. Wenn sie
deaktiviert ist, ist sie immer aus.

Der Standardzustand ist aktiviert.
*/
        pub async fn enable_status_led(
            &mut self,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            self.device
                .set(238u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Deaktiviert die Status LED.

Die Status LED ist die blaue LED neben dem USB-Stecker. Wenn diese aktiviert
ist, ist sie an und sie flackert wenn Daten transferiert werden. Wenn sie
deaktiviert ist, ist sie immer aus.

Der Standardzustand ist aktiviert.
*/
        pub async fn disable_status_led(
            &mut self,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            self.device
                .set(239u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt *true* zurück wenn die Status LED aktiviert ist, *false* sonst.
*/
        pub async fn is_status_led_enabled(
            &mut self,
        ) -> Result<bool, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(240u8, &payload).await?;
            Ok(bool::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Gibt die Firmware und Protokoll Version und den Namen des Bricklets für einen
gegebenen Port zurück.

Der einzige Zweck dieser Funktion ist es, automatischen Flashen von Bricklet
v1.x.y Plugins zu ermöglichen.
*/
        pub async fn get_protocol_1_bricklet_name(
            &mut self,
            request: char,
        ) -> Result<
            crate::bindings::master::GetProtocol1BrickletNameResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(241u8, &payload).await?;
            Ok(
                crate::bindings::master::GetProtocol1BrickletNameResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Gibt die Temperatur, gemessen im Mikrocontroller, aus. Der
Rückgabewert ist nicht die Umgebungstemperatur.

Die Temperatur ist lediglich proportional zur echten Temperatur und hat eine
Genauigkeit von ±15%. Daher beschränkt sich der praktische Nutzen auf die
Indikation von Temperaturveränderungen.
*/
        pub async fn get_chip_temperature(
            &mut self,
        ) -> Result<i16, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(242u8, &payload).await?;
            Ok(i16::from_le_byte_slice(&result.body()[0..2usize]))
        }
        /**
Ein Aufruf dieser Funktion setzt den Brick zurück. Befindet sich der Brick
innerhalb eines Stapels wird der gesamte Stapel zurück gesetzt.

Nach dem Zurücksetzen ist es notwendig neue Geräteobjekte zu erzeugen,
Funktionsaufrufe auf bestehende führt zu undefiniertem Verhalten.
*/
        pub async fn reset(
            &mut self,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            self.device
                .set(243u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Schreibt 32 Bytes Firmware auf das Bricklet, dass am gegebenen Port angeschlossen ist.
Die Bytes werden an die Position offset * 32 geschrieben.

Diese Funktion wird vom Brick Viewer während des Flashens benutzt. In einem
normalem Nutzerprogramm sollte diese Funktion nicht benötigt werden.
*/
        pub async fn write_bricklet_plugin(
            &mut self,
            request: crate::bindings::master::WriteBrickletPluginRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 34usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(246u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Liest 32 Bytes Firmware vom Bricklet, dass am gegebenen Port angeschlossen ist.
Die Bytes werden ab der Position offset * 32 gelesen.

Diese Funktion wird vom Brick Viewer während des Flashens benutzt. In einem
normalem Nutzerprogramm sollte diese Funktion nicht benötigt werden.
*/
        pub async fn read_bricklet_plugin(
            &mut self,
            request: crate::bindings::master::ReadBrickletPluginRequest,
        ) -> Result<[u8; 32usize], tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 2usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(247u8, &payload).await?;
            Ok(<[u8; 32usize]>::from_le_byte_slice(&result.body()[0..32usize]))
        }
        /**
Gibt die UID, die UID zu der der Brick verbunden ist, die
Position, die Hard- und Firmware Version sowie den Device Identifier
zurück.

Die Position ist die Position im Stack von '0' (unterster Brick) bis '8' (oberster Brick).

Eine Liste der Device Identifier Werte ist :ref:`hier <device_identifier>` zu
finden. |device_identifier_constant|
*/
        pub async fn get_identity(
            &mut self,
        ) -> Result<
            crate::bindings::master::GetIdentityResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(255u8, &payload).await?;
            Ok(
                crate::bindings::master::GetIdentityResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
    }
}
pub mod lcd_128_x_64 {
    #[allow(unused_imports)]
    use tinkerforge_base::byte_converter::{FromByteSlice, ToBytes};
    #[allow(unused_imports)]
    use tokio_stream::StreamExt;
    #[allow(unused_imports)]
    use std::convert::TryInto;
    #[derive(Clone, Debug)]
    pub struct Lcd128X64Bricklet {
        device: tinkerforge_base::device::Device,
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct WritePixelsLowLevelRequest {
        pub x_start: u8,
        pub y_start: u8,
        pub x_end: u8,
        pub y_end: u8,
        pub pixels_length: u16,
        pub pixels_chunk_offset: u16,
        pub pixels_chunk_data: [bool; 448usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for WritePixelsLowLevelRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let x_start = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let y_start = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let x_end = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let y_end = u8::from_le_byte_slice(&bytes[3usize..4usize]);
            let pixels_length = u16::from_le_byte_slice(&bytes[4usize..6usize]);
            let pixels_chunk_offset = u16::from_le_byte_slice(&bytes[6usize..8usize]);
            let pixels_chunk_data = <[bool; 448usize]>::from_le_byte_slice(
                &bytes[8usize..64usize],
            );
            Self {
                x_start,
                y_start,
                x_end,
                y_end,
                pixels_length,
                pixels_chunk_offset,
                pixels_chunk_data,
            }
        }
        fn bytes_expected() -> usize {
            64usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for WritePixelsLowLevelRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.x_start.write_to_slice(&mut target[0usize..1usize]);
            self.y_start.write_to_slice(&mut target[1usize..2usize]);
            self.x_end.write_to_slice(&mut target[2usize..3usize]);
            self.y_end.write_to_slice(&mut target[3usize..4usize]);
            self.pixels_length.write_to_slice(&mut target[4usize..6usize]);
            self.pixels_chunk_offset.write_to_slice(&mut target[6usize..8usize]);
            self.pixels_chunk_data.write_to_slice(&mut target[8usize..64usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct WritePixelsRequest {
        pub x_start: u8,
        pub y_start: u8,
        pub x_end: u8,
        pub y_end: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for WritePixelsRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let x_start = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let y_start = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let x_end = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let y_end = u8::from_le_byte_slice(&bytes[3usize..4usize]);
            Self {
                x_start,
                y_start,
                x_end,
                y_end,
            }
        }
        fn bytes_expected() -> usize {
            4usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for WritePixelsRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.x_start.write_to_slice(&mut target[0usize..1usize]);
            self.y_start.write_to_slice(&mut target[1usize..2usize]);
            self.x_end.write_to_slice(&mut target[2usize..3usize]);
            self.y_end.write_to_slice(&mut target[3usize..4usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct ReadPixelsLowLevelRequest {
        pub x_start: u8,
        pub y_start: u8,
        pub x_end: u8,
        pub y_end: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for ReadPixelsLowLevelRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let x_start = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let y_start = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let x_end = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let y_end = u8::from_le_byte_slice(&bytes[3usize..4usize]);
            Self {
                x_start,
                y_start,
                x_end,
                y_end,
            }
        }
        fn bytes_expected() -> usize {
            4usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for ReadPixelsLowLevelRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.x_start.write_to_slice(&mut target[0usize..1usize]);
            self.y_start.write_to_slice(&mut target[1usize..2usize]);
            self.x_end.write_to_slice(&mut target[2usize..3usize]);
            self.y_end.write_to_slice(&mut target[3usize..4usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct ReadPixelsLowLevelResponse {
        pub pixels_length: u16,
        pub pixels_chunk_offset: u16,
        pub pixels_chunk_data: [bool; 480usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for ReadPixelsLowLevelResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let pixels_length = u16::from_le_byte_slice(&bytes[0usize..2usize]);
            let pixels_chunk_offset = u16::from_le_byte_slice(&bytes[2usize..4usize]);
            let pixels_chunk_data = <[bool; 480usize]>::from_le_byte_slice(
                &bytes[4usize..64usize],
            );
            Self {
                pixels_length,
                pixels_chunk_offset,
                pixels_chunk_data,
            }
        }
        fn bytes_expected() -> usize {
            64usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for ReadPixelsLowLevelResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.pixels_length.write_to_slice(&mut target[0usize..2usize]);
            self.pixels_chunk_offset.write_to_slice(&mut target[2usize..4usize]);
            self.pixels_chunk_data.write_to_slice(&mut target[4usize..64usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct ReadPixelsRequest {
        pub x_start: u8,
        pub y_start: u8,
        pub x_end: u8,
        pub y_end: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for ReadPixelsRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let x_start = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let y_start = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let x_end = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let y_end = u8::from_le_byte_slice(&bytes[3usize..4usize]);
            Self {
                x_start,
                y_start,
                x_end,
                y_end,
            }
        }
        fn bytes_expected() -> usize {
            4usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for ReadPixelsRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.x_start.write_to_slice(&mut target[0usize..1usize]);
            self.y_start.write_to_slice(&mut target[1usize..2usize]);
            self.x_end.write_to_slice(&mut target[2usize..3usize]);
            self.y_end.write_to_slice(&mut target[3usize..4usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetDisplayConfigurationRequest {
        pub contrast: u8,
        pub backlight: u8,
        pub invert: bool,
        pub automatic_draw: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetDisplayConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let contrast = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let backlight = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let invert = bool::from_le_byte_slice(&bytes[2usize..3usize]);
            let automatic_draw = bool::from_le_byte_slice(&bytes[3usize..4usize]);
            Self {
                contrast,
                backlight,
                invert,
                automatic_draw,
            }
        }
        fn bytes_expected() -> usize {
            4usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetDisplayConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.contrast.write_to_slice(&mut target[0usize..1usize]);
            self.backlight.write_to_slice(&mut target[1usize..2usize]);
            self.invert.write_to_slice(&mut target[2usize..3usize]);
            self.automatic_draw.write_to_slice(&mut target[3usize..4usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetDisplayConfigurationResponse {
        pub contrast: u8,
        pub backlight: u8,
        pub invert: bool,
        pub automatic_draw: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetDisplayConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let contrast = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let backlight = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let invert = bool::from_le_byte_slice(&bytes[2usize..3usize]);
            let automatic_draw = bool::from_le_byte_slice(&bytes[3usize..4usize]);
            Self {
                contrast,
                backlight,
                invert,
                automatic_draw,
            }
        }
        fn bytes_expected() -> usize {
            4usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetDisplayConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.contrast.write_to_slice(&mut target[0usize..1usize]);
            self.backlight.write_to_slice(&mut target[1usize..2usize]);
            self.invert.write_to_slice(&mut target[2usize..3usize]);
            self.automatic_draw.write_to_slice(&mut target[3usize..4usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct WriteLineRequest {
        pub line: u8,
        pub position: u8,
        pub text: [char; 22usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for WriteLineRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let line = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let position = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let text = <[char; 22usize]>::from_le_byte_slice(&bytes[2usize..24usize]);
            Self { line, position, text }
        }
        fn bytes_expected() -> usize {
            24usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for WriteLineRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.line.write_to_slice(&mut target[0usize..1usize]);
            self.position.write_to_slice(&mut target[1usize..2usize]);
            self.text.write_to_slice(&mut target[2usize..24usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetTouchPositionResponse {
        pub pressure: u16,
        pub x: u16,
        pub y: u16,
        pub age: u32,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetTouchPositionResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let pressure = u16::from_le_byte_slice(&bytes[0usize..2usize]);
            let x = u16::from_le_byte_slice(&bytes[2usize..4usize]);
            let y = u16::from_le_byte_slice(&bytes[4usize..6usize]);
            let age = u32::from_le_byte_slice(&bytes[6usize..10usize]);
            Self { pressure, x, y, age }
        }
        fn bytes_expected() -> usize {
            10usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetTouchPositionResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.pressure.write_to_slice(&mut target[0usize..2usize]);
            self.x.write_to_slice(&mut target[2usize..4usize]);
            self.y.write_to_slice(&mut target[4usize..6usize]);
            self.age.write_to_slice(&mut target[6usize..10usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetTouchPositionCallbackConfigurationRequest {
        pub period: u32,
        pub value_has_to_change: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetTouchPositionCallbackConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let period = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let value_has_to_change = bool::from_le_byte_slice(&bytes[4usize..5usize]);
            Self {
                period,
                value_has_to_change,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for SetTouchPositionCallbackConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.period.write_to_slice(&mut target[0usize..4usize]);
            self.value_has_to_change.write_to_slice(&mut target[4usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetTouchPositionCallbackConfigurationResponse {
        pub period: u32,
        pub value_has_to_change: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetTouchPositionCallbackConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let period = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let value_has_to_change = bool::from_le_byte_slice(&bytes[4usize..5usize]);
            Self {
                period,
                value_has_to_change,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for GetTouchPositionCallbackConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.period.write_to_slice(&mut target[0usize..4usize]);
            self.value_has_to_change.write_to_slice(&mut target[4usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct TouchPositionCallback {
        pub pressure: u16,
        pub x: u16,
        pub y: u16,
        pub age: u32,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for TouchPositionCallback {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let pressure = u16::from_le_byte_slice(&bytes[0usize..2usize]);
            let x = u16::from_le_byte_slice(&bytes[2usize..4usize]);
            let y = u16::from_le_byte_slice(&bytes[4usize..6usize]);
            let age = u32::from_le_byte_slice(&bytes[6usize..10usize]);
            Self { pressure, x, y, age }
        }
        fn bytes_expected() -> usize {
            10usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for TouchPositionCallback {
        fn write_to_slice(self, target: &mut [u8]) {
            self.pressure.write_to_slice(&mut target[0usize..2usize]);
            self.x.write_to_slice(&mut target[2usize..4usize]);
            self.y.write_to_slice(&mut target[4usize..6usize]);
            self.age.write_to_slice(&mut target[6usize..10usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Gesture {
        LeftToRight,
        RightToLeft,
        TopToBottom,
        BottomToTop,
    }
    impl Into<u8> for Gesture {
        fn into(self) -> u8 {
            match self {
                Gesture::LeftToRight => 0u8,
                Gesture::RightToLeft => 1u8,
                Gesture::TopToBottom => 2u8,
                Gesture::BottomToTop => 3u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for Gesture {
        fn write_to_slice(self, target: &mut [u8]) {
            <Gesture as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for Gesture {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<Gesture> for u8 {
        type Error = ();
        fn try_into(self) -> Result<Gesture, Self::Error> {
            match self {
                0u8 => Ok(Gesture::LeftToRight),
                1u8 => Ok(Gesture::RightToLeft),
                2u8 => Ok(Gesture::TopToBottom),
                3u8 => Ok(Gesture::BottomToTop),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetTouchGestureResponse {
        pub gesture: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::lcd_128_x_64::Gesture,
            u8,
        >,
        pub duration: u32,
        pub pressure_max: u16,
        pub x_start: u16,
        pub y_start: u16,
        pub x_end: u16,
        pub y_end: u16,
        pub age: u32,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetTouchGestureResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let gesture = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::lcd_128_x_64::Gesture,
                u8,
            >::from_le_byte_slice(&bytes[0usize..1usize]);
            let duration = u32::from_le_byte_slice(&bytes[1usize..5usize]);
            let pressure_max = u16::from_le_byte_slice(&bytes[5usize..7usize]);
            let x_start = u16::from_le_byte_slice(&bytes[7usize..9usize]);
            let y_start = u16::from_le_byte_slice(&bytes[9usize..11usize]);
            let x_end = u16::from_le_byte_slice(&bytes[11usize..13usize]);
            let y_end = u16::from_le_byte_slice(&bytes[13usize..15usize]);
            let age = u32::from_le_byte_slice(&bytes[15usize..19usize]);
            Self {
                gesture,
                duration,
                pressure_max,
                x_start,
                y_start,
                x_end,
                y_end,
                age,
            }
        }
        fn bytes_expected() -> usize {
            19usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetTouchGestureResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.gesture.write_to_slice(&mut target[0usize..1usize]);
            self.duration.write_to_slice(&mut target[1usize..5usize]);
            self.pressure_max.write_to_slice(&mut target[5usize..7usize]);
            self.x_start.write_to_slice(&mut target[7usize..9usize]);
            self.y_start.write_to_slice(&mut target[9usize..11usize]);
            self.x_end.write_to_slice(&mut target[11usize..13usize]);
            self.y_end.write_to_slice(&mut target[13usize..15usize]);
            self.age.write_to_slice(&mut target[15usize..19usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetTouchGestureCallbackConfigurationRequest {
        pub period: u32,
        pub value_has_to_change: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetTouchGestureCallbackConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let period = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let value_has_to_change = bool::from_le_byte_slice(&bytes[4usize..5usize]);
            Self {
                period,
                value_has_to_change,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for SetTouchGestureCallbackConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.period.write_to_slice(&mut target[0usize..4usize]);
            self.value_has_to_change.write_to_slice(&mut target[4usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetTouchGestureCallbackConfigurationResponse {
        pub period: u32,
        pub value_has_to_change: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetTouchGestureCallbackConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let period = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let value_has_to_change = bool::from_le_byte_slice(&bytes[4usize..5usize]);
            Self {
                period,
                value_has_to_change,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for GetTouchGestureCallbackConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.period.write_to_slice(&mut target[0usize..4usize]);
            self.value_has_to_change.write_to_slice(&mut target[4usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct TouchGestureCallback {
        pub gesture: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::lcd_128_x_64::Gesture,
            u8,
        >,
        pub duration: u32,
        pub pressure_max: u16,
        pub x_start: u16,
        pub y_start: u16,
        pub x_end: u16,
        pub y_end: u16,
        pub age: u32,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for TouchGestureCallback {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let gesture = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::lcd_128_x_64::Gesture,
                u8,
            >::from_le_byte_slice(&bytes[0usize..1usize]);
            let duration = u32::from_le_byte_slice(&bytes[1usize..5usize]);
            let pressure_max = u16::from_le_byte_slice(&bytes[5usize..7usize]);
            let x_start = u16::from_le_byte_slice(&bytes[7usize..9usize]);
            let y_start = u16::from_le_byte_slice(&bytes[9usize..11usize]);
            let x_end = u16::from_le_byte_slice(&bytes[11usize..13usize]);
            let y_end = u16::from_le_byte_slice(&bytes[13usize..15usize]);
            let age = u32::from_le_byte_slice(&bytes[15usize..19usize]);
            Self {
                gesture,
                duration,
                pressure_max,
                x_start,
                y_start,
                x_end,
                y_end,
                age,
            }
        }
        fn bytes_expected() -> usize {
            19usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for TouchGestureCallback {
        fn write_to_slice(self, target: &mut [u8]) {
            self.gesture.write_to_slice(&mut target[0usize..1usize]);
            self.duration.write_to_slice(&mut target[1usize..5usize]);
            self.pressure_max.write_to_slice(&mut target[5usize..7usize]);
            self.x_start.write_to_slice(&mut target[7usize..9usize]);
            self.y_start.write_to_slice(&mut target[9usize..11usize]);
            self.x_end.write_to_slice(&mut target[11usize..13usize]);
            self.y_end.write_to_slice(&mut target[13usize..15usize]);
            self.age.write_to_slice(&mut target[15usize..19usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Color {
        White,
        Black,
    }
    impl Into<bool> for Color {
        fn into(self) -> bool {
            match self {
                Color::White => false,
                Color::Black => true,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for Color {
        fn write_to_slice(self, target: &mut [u8]) {
            <Color as Into<bool>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for Color {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            bool::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<Color> for bool {
        type Error = ();
        fn try_into(self) -> Result<Color, Self::Error> {
            match self {
                false => Ok(Color::White),
                true => Ok(Color::Black),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct DrawLineRequest {
        pub position_x_start: u8,
        pub position_y_start: u8,
        pub position_x_end: u8,
        pub position_y_end: u8,
        pub color: crate::bindings::lcd_128_x_64::Color,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for DrawLineRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let position_x_start = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let position_y_start = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let position_x_end = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let position_y_end = u8::from_le_byte_slice(&bytes[3usize..4usize]);
            let color = crate::bindings::lcd_128_x_64::Color::from_le_byte_slice(
                &bytes[4usize..5usize],
            );
            Self {
                position_x_start,
                position_y_start,
                position_x_end,
                position_y_end,
                color,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for DrawLineRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.position_x_start.write_to_slice(&mut target[0usize..1usize]);
            self.position_y_start.write_to_slice(&mut target[1usize..2usize]);
            self.position_x_end.write_to_slice(&mut target[2usize..3usize]);
            self.position_y_end.write_to_slice(&mut target[3usize..4usize]);
            self.color.write_to_slice(&mut target[4usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct DrawBoxRequest {
        pub position_x_start: u8,
        pub position_y_start: u8,
        pub position_x_end: u8,
        pub position_y_end: u8,
        pub fill: bool,
        pub color: crate::bindings::lcd_128_x_64::Color,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for DrawBoxRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let position_x_start = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let position_y_start = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let position_x_end = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let position_y_end = u8::from_le_byte_slice(&bytes[3usize..4usize]);
            let fill = bool::from_le_byte_slice(&bytes[4usize..5usize]);
            let color = crate::bindings::lcd_128_x_64::Color::from_le_byte_slice(
                &bytes[5usize..6usize],
            );
            Self {
                position_x_start,
                position_y_start,
                position_x_end,
                position_y_end,
                fill,
                color,
            }
        }
        fn bytes_expected() -> usize {
            6usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for DrawBoxRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.position_x_start.write_to_slice(&mut target[0usize..1usize]);
            self.position_y_start.write_to_slice(&mut target[1usize..2usize]);
            self.position_x_end.write_to_slice(&mut target[2usize..3usize]);
            self.position_y_end.write_to_slice(&mut target[3usize..4usize]);
            self.fill.write_to_slice(&mut target[4usize..5usize]);
            self.color.write_to_slice(&mut target[5usize..6usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Font {
        _6X8,
        _6X16,
        _6X24,
        _6X32,
        _12X16,
        _12X24,
        _12X32,
        _18X24,
        _18X32,
        _24X32,
    }
    impl Into<u8> for Font {
        fn into(self) -> u8 {
            match self {
                Font::_6X8 => 0u8,
                Font::_6X16 => 1u8,
                Font::_6X24 => 2u8,
                Font::_6X32 => 3u8,
                Font::_12X16 => 4u8,
                Font::_12X24 => 5u8,
                Font::_12X32 => 6u8,
                Font::_18X24 => 7u8,
                Font::_18X32 => 8u8,
                Font::_24X32 => 9u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for Font {
        fn write_to_slice(self, target: &mut [u8]) {
            <Font as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for Font {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<Font> for u8 {
        type Error = ();
        fn try_into(self) -> Result<Font, Self::Error> {
            match self {
                0u8 => Ok(Font::_6X8),
                1u8 => Ok(Font::_6X16),
                2u8 => Ok(Font::_6X24),
                3u8 => Ok(Font::_6X32),
                4u8 => Ok(Font::_12X16),
                5u8 => Ok(Font::_12X24),
                6u8 => Ok(Font::_12X32),
                7u8 => Ok(Font::_18X24),
                8u8 => Ok(Font::_18X32),
                9u8 => Ok(Font::_24X32),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct DrawTextRequest {
        pub position_x: u8,
        pub position_y: u8,
        pub font: crate::bindings::lcd_128_x_64::Font,
        pub color: crate::bindings::lcd_128_x_64::Color,
        pub text: [char; 22usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for DrawTextRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let position_x = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let position_y = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let font = crate::bindings::lcd_128_x_64::Font::from_le_byte_slice(
                &bytes[2usize..3usize],
            );
            let color = crate::bindings::lcd_128_x_64::Color::from_le_byte_slice(
                &bytes[3usize..4usize],
            );
            let text = <[char; 22usize]>::from_le_byte_slice(&bytes[4usize..26usize]);
            Self {
                position_x,
                position_y,
                font,
                color,
                text,
            }
        }
        fn bytes_expected() -> usize {
            26usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for DrawTextRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.position_x.write_to_slice(&mut target[0usize..1usize]);
            self.position_y.write_to_slice(&mut target[1usize..2usize]);
            self.font.write_to_slice(&mut target[2usize..3usize]);
            self.color.write_to_slice(&mut target[3usize..4usize]);
            self.text.write_to_slice(&mut target[4usize..26usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetGuiButtonRequest {
        pub index: u8,
        pub position_x: u8,
        pub position_y: u8,
        pub width: u8,
        pub height: u8,
        pub text: [char; 16usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for SetGuiButtonRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let index = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let position_x = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let position_y = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let width = u8::from_le_byte_slice(&bytes[3usize..4usize]);
            let height = u8::from_le_byte_slice(&bytes[4usize..5usize]);
            let text = <[char; 16usize]>::from_le_byte_slice(&bytes[5usize..21usize]);
            Self {
                index,
                position_x,
                position_y,
                width,
                height,
                text,
            }
        }
        fn bytes_expected() -> usize {
            21usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetGuiButtonRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.index.write_to_slice(&mut target[0usize..1usize]);
            self.position_x.write_to_slice(&mut target[1usize..2usize]);
            self.position_y.write_to_slice(&mut target[2usize..3usize]);
            self.width.write_to_slice(&mut target[3usize..4usize]);
            self.height.write_to_slice(&mut target[4usize..5usize]);
            self.text.write_to_slice(&mut target[5usize..21usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetGuiButtonResponse {
        pub active: bool,
        pub position_x: u8,
        pub position_y: u8,
        pub width: u8,
        pub height: u8,
        pub text: [char; 16usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetGuiButtonResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let active = bool::from_le_byte_slice(&bytes[0usize..1usize]);
            let position_x = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let position_y = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let width = u8::from_le_byte_slice(&bytes[3usize..4usize]);
            let height = u8::from_le_byte_slice(&bytes[4usize..5usize]);
            let text = <[char; 16usize]>::from_le_byte_slice(&bytes[5usize..21usize]);
            Self {
                active,
                position_x,
                position_y,
                width,
                height,
                text,
            }
        }
        fn bytes_expected() -> usize {
            21usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetGuiButtonResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.active.write_to_slice(&mut target[0usize..1usize]);
            self.position_x.write_to_slice(&mut target[1usize..2usize]);
            self.position_y.write_to_slice(&mut target[2usize..3usize]);
            self.width.write_to_slice(&mut target[3usize..4usize]);
            self.height.write_to_slice(&mut target[4usize..5usize]);
            self.text.write_to_slice(&mut target[5usize..21usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetGuiButtonPressedCallbackConfigurationRequest {
        pub period: u32,
        pub value_has_to_change: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetGuiButtonPressedCallbackConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let period = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let value_has_to_change = bool::from_le_byte_slice(&bytes[4usize..5usize]);
            Self {
                period,
                value_has_to_change,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for SetGuiButtonPressedCallbackConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.period.write_to_slice(&mut target[0usize..4usize]);
            self.value_has_to_change.write_to_slice(&mut target[4usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetGuiButtonPressedCallbackConfigurationResponse {
        pub period: u32,
        pub value_has_to_change: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetGuiButtonPressedCallbackConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let period = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let value_has_to_change = bool::from_le_byte_slice(&bytes[4usize..5usize]);
            Self {
                period,
                value_has_to_change,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for GetGuiButtonPressedCallbackConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.period.write_to_slice(&mut target[0usize..4usize]);
            self.value_has_to_change.write_to_slice(&mut target[4usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GuiButtonPressedCallback {
        pub index: u8,
        pub pressed: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GuiButtonPressedCallback {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let index = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let pressed = bool::from_le_byte_slice(&bytes[1usize..2usize]);
            Self { index, pressed }
        }
        fn bytes_expected() -> usize {
            2usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GuiButtonPressedCallback {
        fn write_to_slice(self, target: &mut [u8]) {
            self.index.write_to_slice(&mut target[0usize..1usize]);
            self.pressed.write_to_slice(&mut target[1usize..2usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Direction {
        Horizontal,
        Vertical,
    }
    impl Into<u8> for Direction {
        fn into(self) -> u8 {
            match self {
                Direction::Horizontal => 0u8,
                Direction::Vertical => 1u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for Direction {
        fn write_to_slice(self, target: &mut [u8]) {
            <Direction as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for Direction {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<Direction> for u8 {
        type Error = ();
        fn try_into(self) -> Result<Direction, Self::Error> {
            match self {
                0u8 => Ok(Direction::Horizontal),
                1u8 => Ok(Direction::Vertical),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetGuiSliderRequest {
        pub index: u8,
        pub position_x: u8,
        pub position_y: u8,
        pub length: u8,
        pub direction: crate::bindings::lcd_128_x_64::Direction,
        pub value: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for SetGuiSliderRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let index = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let position_x = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let position_y = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let length = u8::from_le_byte_slice(&bytes[3usize..4usize]);
            let direction = crate::bindings::lcd_128_x_64::Direction::from_le_byte_slice(
                &bytes[4usize..5usize],
            );
            let value = u8::from_le_byte_slice(&bytes[5usize..6usize]);
            Self {
                index,
                position_x,
                position_y,
                length,
                direction,
                value,
            }
        }
        fn bytes_expected() -> usize {
            6usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetGuiSliderRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.index.write_to_slice(&mut target[0usize..1usize]);
            self.position_x.write_to_slice(&mut target[1usize..2usize]);
            self.position_y.write_to_slice(&mut target[2usize..3usize]);
            self.length.write_to_slice(&mut target[3usize..4usize]);
            self.direction.write_to_slice(&mut target[4usize..5usize]);
            self.value.write_to_slice(&mut target[5usize..6usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetGuiSliderResponse {
        pub active: bool,
        pub position_x: u8,
        pub position_y: u8,
        pub length: u8,
        pub direction: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::lcd_128_x_64::Direction,
            u8,
        >,
        pub value: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetGuiSliderResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let active = bool::from_le_byte_slice(&bytes[0usize..1usize]);
            let position_x = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            let position_y = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let length = u8::from_le_byte_slice(&bytes[3usize..4usize]);
            let direction = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::lcd_128_x_64::Direction,
                u8,
            >::from_le_byte_slice(&bytes[4usize..5usize]);
            let value = u8::from_le_byte_slice(&bytes[5usize..6usize]);
            Self {
                active,
                position_x,
                position_y,
                length,
                direction,
                value,
            }
        }
        fn bytes_expected() -> usize {
            6usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetGuiSliderResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.active.write_to_slice(&mut target[0usize..1usize]);
            self.position_x.write_to_slice(&mut target[1usize..2usize]);
            self.position_y.write_to_slice(&mut target[2usize..3usize]);
            self.length.write_to_slice(&mut target[3usize..4usize]);
            self.direction.write_to_slice(&mut target[4usize..5usize]);
            self.value.write_to_slice(&mut target[5usize..6usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetGuiSliderValueCallbackConfigurationRequest {
        pub period: u32,
        pub value_has_to_change: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetGuiSliderValueCallbackConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let period = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let value_has_to_change = bool::from_le_byte_slice(&bytes[4usize..5usize]);
            Self {
                period,
                value_has_to_change,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for SetGuiSliderValueCallbackConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.period.write_to_slice(&mut target[0usize..4usize]);
            self.value_has_to_change.write_to_slice(&mut target[4usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetGuiSliderValueCallbackConfigurationResponse {
        pub period: u32,
        pub value_has_to_change: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetGuiSliderValueCallbackConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let period = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let value_has_to_change = bool::from_le_byte_slice(&bytes[4usize..5usize]);
            Self {
                period,
                value_has_to_change,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for GetGuiSliderValueCallbackConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.period.write_to_slice(&mut target[0usize..4usize]);
            self.value_has_to_change.write_to_slice(&mut target[4usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GuiSliderValueCallback {
        pub index: u8,
        pub value: u8,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GuiSliderValueCallback {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let index = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let value = u8::from_le_byte_slice(&bytes[1usize..2usize]);
            Self { index, value }
        }
        fn bytes_expected() -> usize {
            2usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GuiSliderValueCallback {
        fn write_to_slice(self, target: &mut [u8]) {
            self.index.write_to_slice(&mut target[0usize..1usize]);
            self.value.write_to_slice(&mut target[1usize..2usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum ChangeTabOn {
        Click,
        Swipe,
        ClickAndSwipe,
    }
    impl Into<u8> for ChangeTabOn {
        fn into(self) -> u8 {
            match self {
                ChangeTabOn::Click => 1u8,
                ChangeTabOn::Swipe => 2u8,
                ChangeTabOn::ClickAndSwipe => 3u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for ChangeTabOn {
        fn write_to_slice(self, target: &mut [u8]) {
            <ChangeTabOn as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for ChangeTabOn {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<ChangeTabOn> for u8 {
        type Error = ();
        fn try_into(self) -> Result<ChangeTabOn, Self::Error> {
            match self {
                1u8 => Ok(ChangeTabOn::Click),
                2u8 => Ok(ChangeTabOn::Swipe),
                3u8 => Ok(ChangeTabOn::ClickAndSwipe),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetGuiTabConfigurationRequest {
        pub change_tab_config: crate::bindings::lcd_128_x_64::ChangeTabOn,
        pub clear_gui: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetGuiTabConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let change_tab_config = crate::bindings::lcd_128_x_64::ChangeTabOn::from_le_byte_slice(
                &bytes[0usize..1usize],
            );
            let clear_gui = bool::from_le_byte_slice(&bytes[1usize..2usize]);
            Self {
                change_tab_config,
                clear_gui,
            }
        }
        fn bytes_expected() -> usize {
            2usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetGuiTabConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.change_tab_config.write_to_slice(&mut target[0usize..1usize]);
            self.clear_gui.write_to_slice(&mut target[1usize..2usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetGuiTabConfigurationResponse {
        pub change_tab_config: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::lcd_128_x_64::ChangeTabOn,
            u8,
        >,
        pub clear_gui: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetGuiTabConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let change_tab_config = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::lcd_128_x_64::ChangeTabOn,
                u8,
            >::from_le_byte_slice(&bytes[0usize..1usize]);
            let clear_gui = bool::from_le_byte_slice(&bytes[1usize..2usize]);
            Self {
                change_tab_config,
                clear_gui,
            }
        }
        fn bytes_expected() -> usize {
            2usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetGuiTabConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.change_tab_config.write_to_slice(&mut target[0usize..1usize]);
            self.clear_gui.write_to_slice(&mut target[1usize..2usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetGuiTabTextRequest {
        pub index: u8,
        pub text: [char; 5usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for SetGuiTabTextRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let index = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let text = <[char; 5usize]>::from_le_byte_slice(&bytes[1usize..6usize]);
            Self { index, text }
        }
        fn bytes_expected() -> usize {
            6usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetGuiTabTextRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.index.write_to_slice(&mut target[0usize..1usize]);
            self.text.write_to_slice(&mut target[1usize..6usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetGuiTabTextResponse {
        pub active: bool,
        pub text: [char; 5usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetGuiTabTextResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let active = bool::from_le_byte_slice(&bytes[0usize..1usize]);
            let text = <[char; 5usize]>::from_le_byte_slice(&bytes[1usize..6usize]);
            Self { active, text }
        }
        fn bytes_expected() -> usize {
            6usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetGuiTabTextResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.active.write_to_slice(&mut target[0usize..1usize]);
            self.text.write_to_slice(&mut target[1usize..6usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetGuiTabIconRequest {
        pub index: u8,
        pub icon: [bool; 168usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for SetGuiTabIconRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let index = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let icon = <[bool; 168usize]>::from_le_byte_slice(&bytes[1usize..22usize]);
            Self { index, icon }
        }
        fn bytes_expected() -> usize {
            22usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetGuiTabIconRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.index.write_to_slice(&mut target[0usize..1usize]);
            self.icon.write_to_slice(&mut target[1usize..22usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetGuiTabIconResponse {
        pub active: bool,
        pub icon: [bool; 168usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetGuiTabIconResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let active = bool::from_le_byte_slice(&bytes[0usize..1usize]);
            let icon = <[bool; 168usize]>::from_le_byte_slice(&bytes[1usize..22usize]);
            Self { active, icon }
        }
        fn bytes_expected() -> usize {
            22usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetGuiTabIconResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.active.write_to_slice(&mut target[0usize..1usize]);
            self.icon.write_to_slice(&mut target[1usize..22usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetGuiTabSelectedCallbackConfigurationRequest {
        pub period: u32,
        pub value_has_to_change: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetGuiTabSelectedCallbackConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let period = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let value_has_to_change = bool::from_le_byte_slice(&bytes[4usize..5usize]);
            Self {
                period,
                value_has_to_change,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for SetGuiTabSelectedCallbackConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.period.write_to_slice(&mut target[0usize..4usize]);
            self.value_has_to_change.write_to_slice(&mut target[4usize..5usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetGuiTabSelectedCallbackConfigurationResponse {
        pub period: u32,
        pub value_has_to_change: bool,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetGuiTabSelectedCallbackConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let period = u32::from_le_byte_slice(&bytes[0usize..4usize]);
            let value_has_to_change = bool::from_le_byte_slice(&bytes[4usize..5usize]);
            Self {
                period,
                value_has_to_change,
            }
        }
        fn bytes_expected() -> usize {
            5usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes
    for GetGuiTabSelectedCallbackConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.period.write_to_slice(&mut target[0usize..4usize]);
            self.value_has_to_change.write_to_slice(&mut target[4usize..5usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum GraphType {
        Dot,
        Line,
        Bar,
    }
    impl Into<u8> for GraphType {
        fn into(self) -> u8 {
            match self {
                GraphType::Dot => 0u8,
                GraphType::Line => 1u8,
                GraphType::Bar => 2u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GraphType {
        fn write_to_slice(self, target: &mut [u8]) {
            <GraphType as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GraphType {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<GraphType> for u8 {
        type Error = ();
        fn try_into(self) -> Result<GraphType, Self::Error> {
            match self {
                0u8 => Ok(GraphType::Dot),
                1u8 => Ok(GraphType::Line),
                2u8 => Ok(GraphType::Bar),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetGuiGraphConfigurationRequest {
        pub index: u8,
        pub graph_type: crate::bindings::lcd_128_x_64::GraphType,
        pub position_x: u8,
        pub position_y: u8,
        pub width: u8,
        pub height: u8,
        pub text_x: [char; 4usize],
        pub text_y: [char; 4usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetGuiGraphConfigurationRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let index = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let graph_type = crate::bindings::lcd_128_x_64::GraphType::from_le_byte_slice(
                &bytes[1usize..2usize],
            );
            let position_x = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let position_y = u8::from_le_byte_slice(&bytes[3usize..4usize]);
            let width = u8::from_le_byte_slice(&bytes[4usize..5usize]);
            let height = u8::from_le_byte_slice(&bytes[5usize..6usize]);
            let text_x = <[char; 4usize]>::from_le_byte_slice(&bytes[6usize..10usize]);
            let text_y = <[char; 4usize]>::from_le_byte_slice(&bytes[10usize..14usize]);
            Self {
                index,
                graph_type,
                position_x,
                position_y,
                width,
                height,
                text_x,
                text_y,
            }
        }
        fn bytes_expected() -> usize {
            14usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetGuiGraphConfigurationRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.index.write_to_slice(&mut target[0usize..1usize]);
            self.graph_type.write_to_slice(&mut target[1usize..2usize]);
            self.position_x.write_to_slice(&mut target[2usize..3usize]);
            self.position_y.write_to_slice(&mut target[3usize..4usize]);
            self.width.write_to_slice(&mut target[4usize..5usize]);
            self.height.write_to_slice(&mut target[5usize..6usize]);
            self.text_x.write_to_slice(&mut target[6usize..10usize]);
            self.text_y.write_to_slice(&mut target[10usize..14usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetGuiGraphConfigurationResponse {
        pub active: bool,
        pub graph_type: tinkerforge_base::byte_converter::ParsedOrRaw<
            crate::bindings::lcd_128_x_64::GraphType,
            u8,
        >,
        pub position_x: u8,
        pub position_y: u8,
        pub width: u8,
        pub height: u8,
        pub text_x: [char; 4usize],
        pub text_y: [char; 4usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetGuiGraphConfigurationResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let active = bool::from_le_byte_slice(&bytes[0usize..1usize]);
            let graph_type = tinkerforge_base::byte_converter::ParsedOrRaw::<
                crate::bindings::lcd_128_x_64::GraphType,
                u8,
            >::from_le_byte_slice(&bytes[1usize..2usize]);
            let position_x = u8::from_le_byte_slice(&bytes[2usize..3usize]);
            let position_y = u8::from_le_byte_slice(&bytes[3usize..4usize]);
            let width = u8::from_le_byte_slice(&bytes[4usize..5usize]);
            let height = u8::from_le_byte_slice(&bytes[5usize..6usize]);
            let text_x = <[char; 4usize]>::from_le_byte_slice(&bytes[6usize..10usize]);
            let text_y = <[char; 4usize]>::from_le_byte_slice(&bytes[10usize..14usize]);
            Self {
                active,
                graph_type,
                position_x,
                position_y,
                width,
                height,
                text_x,
                text_y,
            }
        }
        fn bytes_expected() -> usize {
            14usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetGuiGraphConfigurationResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.active.write_to_slice(&mut target[0usize..1usize]);
            self.graph_type.write_to_slice(&mut target[1usize..2usize]);
            self.position_x.write_to_slice(&mut target[2usize..3usize]);
            self.position_y.write_to_slice(&mut target[3usize..4usize]);
            self.width.write_to_slice(&mut target[4usize..5usize]);
            self.height.write_to_slice(&mut target[5usize..6usize]);
            self.text_x.write_to_slice(&mut target[6usize..10usize]);
            self.text_y.write_to_slice(&mut target[10usize..14usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct SetGuiGraphDataLowLevelRequest {
        pub index: u8,
        pub data_length: u16,
        pub data_chunk_offset: u16,
        pub data_chunk_data: [u8; 59usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for SetGuiGraphDataLowLevelRequest {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let index = u8::from_le_byte_slice(&bytes[0usize..1usize]);
            let data_length = u16::from_le_byte_slice(&bytes[1usize..3usize]);
            let data_chunk_offset = u16::from_le_byte_slice(&bytes[3usize..5usize]);
            let data_chunk_data = <[u8; 59usize]>::from_le_byte_slice(
                &bytes[5usize..64usize],
            );
            Self {
                index,
                data_length,
                data_chunk_offset,
                data_chunk_data,
            }
        }
        fn bytes_expected() -> usize {
            64usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for SetGuiGraphDataLowLevelRequest {
        fn write_to_slice(self, target: &mut [u8]) {
            self.index.write_to_slice(&mut target[0usize..1usize]);
            self.data_length.write_to_slice(&mut target[1usize..3usize]);
            self.data_chunk_offset.write_to_slice(&mut target[3usize..5usize]);
            self.data_chunk_data.write_to_slice(&mut target[5usize..64usize]);
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetGuiGraphDataLowLevelResponse {
        pub data_length: u16,
        pub data_chunk_offset: u16,
        pub data_chunk_data: [u8; 59usize],
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetGuiGraphDataLowLevelResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let data_length = u16::from_le_byte_slice(&bytes[0usize..2usize]);
            let data_chunk_offset = u16::from_le_byte_slice(&bytes[2usize..4usize]);
            let data_chunk_data = <[u8; 59usize]>::from_le_byte_slice(
                &bytes[4usize..63usize],
            );
            Self {
                data_length,
                data_chunk_offset,
                data_chunk_data,
            }
        }
        fn bytes_expected() -> usize {
            63usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetGuiGraphDataLowLevelResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.data_length.write_to_slice(&mut target[0usize..2usize]);
            self.data_chunk_offset.write_to_slice(&mut target[2usize..4usize]);
            self.data_chunk_data.write_to_slice(&mut target[4usize..63usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum TouchLedConfig {
        Off,
        On,
        ShowHeartbeat,
        ShowTouch,
    }
    impl Into<u8> for TouchLedConfig {
        fn into(self) -> u8 {
            match self {
                TouchLedConfig::Off => 0u8,
                TouchLedConfig::On => 1u8,
                TouchLedConfig::ShowHeartbeat => 2u8,
                TouchLedConfig::ShowTouch => 3u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for TouchLedConfig {
        fn write_to_slice(self, target: &mut [u8]) {
            <TouchLedConfig as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for TouchLedConfig {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<TouchLedConfig> for u8 {
        type Error = ();
        fn try_into(self) -> Result<TouchLedConfig, Self::Error> {
            match self {
                0u8 => Ok(TouchLedConfig::Off),
                1u8 => Ok(TouchLedConfig::On),
                2u8 => Ok(TouchLedConfig::ShowHeartbeat),
                3u8 => Ok(TouchLedConfig::ShowTouch),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetSpitfpErrorCountResponse {
        pub error_count_ack_checksum: u32,
        pub error_count_message_checksum: u32,
        pub error_count_frame: u32,
        pub error_count_overflow: u32,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice
    for GetSpitfpErrorCountResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let error_count_ack_checksum = u32::from_le_byte_slice(
                &bytes[0usize..4usize],
            );
            let error_count_message_checksum = u32::from_le_byte_slice(
                &bytes[4usize..8usize],
            );
            let error_count_frame = u32::from_le_byte_slice(&bytes[8usize..12usize]);
            let error_count_overflow = u32::from_le_byte_slice(&bytes[12usize..16usize]);
            Self {
                error_count_ack_checksum,
                error_count_message_checksum,
                error_count_frame,
                error_count_overflow,
            }
        }
        fn bytes_expected() -> usize {
            16usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetSpitfpErrorCountResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.error_count_ack_checksum.write_to_slice(&mut target[0usize..4usize]);
            self.error_count_message_checksum
                .write_to_slice(&mut target[4usize..8usize]);
            self.error_count_frame.write_to_slice(&mut target[8usize..12usize]);
            self.error_count_overflow.write_to_slice(&mut target[12usize..16usize]);
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum BootloaderMode {
        Bootloader,
        Firmware,
        BootloaderWaitForReboot,
        FirmwareWaitForReboot,
        FirmwareWaitForEraseAndReboot,
    }
    impl Into<u8> for BootloaderMode {
        fn into(self) -> u8 {
            match self {
                BootloaderMode::Bootloader => 0u8,
                BootloaderMode::Firmware => 1u8,
                BootloaderMode::BootloaderWaitForReboot => 2u8,
                BootloaderMode::FirmwareWaitForReboot => 3u8,
                BootloaderMode::FirmwareWaitForEraseAndReboot => 4u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for BootloaderMode {
        fn write_to_slice(self, target: &mut [u8]) {
            <BootloaderMode as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for BootloaderMode {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<BootloaderMode> for u8 {
        type Error = ();
        fn try_into(self) -> Result<BootloaderMode, Self::Error> {
            match self {
                0u8 => Ok(BootloaderMode::Bootloader),
                1u8 => Ok(BootloaderMode::Firmware),
                2u8 => Ok(BootloaderMode::BootloaderWaitForReboot),
                3u8 => Ok(BootloaderMode::FirmwareWaitForReboot),
                4u8 => Ok(BootloaderMode::FirmwareWaitForEraseAndReboot),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum BootloaderStatus {
        Ok,
        InvalidMode,
        NoChange,
        EntryFunctionNotPresent,
        DeviceIdentifierIncorrect,
        CrcMismatch,
    }
    impl Into<u8> for BootloaderStatus {
        fn into(self) -> u8 {
            match self {
                BootloaderStatus::Ok => 0u8,
                BootloaderStatus::InvalidMode => 1u8,
                BootloaderStatus::NoChange => 2u8,
                BootloaderStatus::EntryFunctionNotPresent => 3u8,
                BootloaderStatus::DeviceIdentifierIncorrect => 4u8,
                BootloaderStatus::CrcMismatch => 5u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for BootloaderStatus {
        fn write_to_slice(self, target: &mut [u8]) {
            <BootloaderStatus as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for BootloaderStatus {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<BootloaderStatus> for u8 {
        type Error = ();
        fn try_into(self) -> Result<BootloaderStatus, Self::Error> {
            match self {
                0u8 => Ok(BootloaderStatus::Ok),
                1u8 => Ok(BootloaderStatus::InvalidMode),
                2u8 => Ok(BootloaderStatus::NoChange),
                3u8 => Ok(BootloaderStatus::EntryFunctionNotPresent),
                4u8 => Ok(BootloaderStatus::DeviceIdentifierIncorrect),
                5u8 => Ok(BootloaderStatus::CrcMismatch),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum StatusLedConfig {
        Off,
        On,
        ShowHeartbeat,
        ShowStatus,
    }
    impl Into<u8> for StatusLedConfig {
        fn into(self) -> u8 {
            match self {
                StatusLedConfig::Off => 0u8,
                StatusLedConfig::On => 1u8,
                StatusLedConfig::ShowHeartbeat => 2u8,
                StatusLedConfig::ShowStatus => 3u8,
            }
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for StatusLedConfig {
        fn write_to_slice(self, target: &mut [u8]) {
            <StatusLedConfig as Into<u8>>::into(self).write_to_slice(target);
        }
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for StatusLedConfig {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            u8::from_le_byte_slice(bytes).try_into().expect("unsupported enum value")
        }
        fn bytes_expected() -> usize {
            1usize
        }
    }
    impl std::convert::TryInto<StatusLedConfig> for u8 {
        type Error = ();
        fn try_into(self) -> Result<StatusLedConfig, Self::Error> {
            match self {
                0u8 => Ok(StatusLedConfig::Off),
                1u8 => Ok(StatusLedConfig::On),
                2u8 => Ok(StatusLedConfig::ShowHeartbeat),
                3u8 => Ok(StatusLedConfig::ShowStatus),
                _ => Err(()),
            }
        }
    }
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct GetIdentityResponse {
        pub uid: [char; 8usize],
        pub connected_uid: [char; 8usize],
        pub position: char,
        pub hardware_version_major: u8,
        pub hardware_version_minor: u8,
        pub hardware_version_revision: u8,
        pub firmware_version_major: u8,
        pub firmware_version_minor: u8,
        pub firmware_version_revision: u8,
        pub device_identifier: u16,
    }
    impl tinkerforge_base::byte_converter::FromByteSlice for GetIdentityResponse {
        fn from_le_byte_slice(bytes: &[u8]) -> Self {
            let uid = <[char; 8usize]>::from_le_byte_slice(&bytes[0usize..8usize]);
            let connected_uid = <[char; 8usize]>::from_le_byte_slice(
                &bytes[8usize..16usize],
            );
            let position = char::from_le_byte_slice(&bytes[16usize..17usize]);
            let hardware_version_major = u8::from_le_byte_slice(
                &bytes[17usize..18usize],
            );
            let hardware_version_minor = u8::from_le_byte_slice(
                &bytes[18usize..19usize],
            );
            let hardware_version_revision = u8::from_le_byte_slice(
                &bytes[19usize..20usize],
            );
            let firmware_version_major = u8::from_le_byte_slice(
                &bytes[20usize..21usize],
            );
            let firmware_version_minor = u8::from_le_byte_slice(
                &bytes[21usize..22usize],
            );
            let firmware_version_revision = u8::from_le_byte_slice(
                &bytes[22usize..23usize],
            );
            let device_identifier = u16::from_le_byte_slice(&bytes[23usize..25usize]);
            Self {
                uid,
                connected_uid,
                position,
                hardware_version_major,
                hardware_version_minor,
                hardware_version_revision,
                firmware_version_major,
                firmware_version_minor,
                firmware_version_revision,
                device_identifier,
            }
        }
        fn bytes_expected() -> usize {
            25usize
        }
    }
    impl tinkerforge_base::byte_converter::ToBytes for GetIdentityResponse {
        fn write_to_slice(self, target: &mut [u8]) {
            self.uid.write_to_slice(&mut target[0usize..8usize]);
            self.connected_uid.write_to_slice(&mut target[8usize..16usize]);
            self.position.write_to_slice(&mut target[16usize..17usize]);
            self.hardware_version_major.write_to_slice(&mut target[17usize..18usize]);
            self.hardware_version_minor.write_to_slice(&mut target[18usize..19usize]);
            self.hardware_version_revision.write_to_slice(&mut target[19usize..20usize]);
            self.firmware_version_major.write_to_slice(&mut target[20usize..21usize]);
            self.firmware_version_minor.write_to_slice(&mut target[21usize..22usize]);
            self.firmware_version_revision.write_to_slice(&mut target[22usize..23usize]);
            self.device_identifier.write_to_slice(&mut target[23usize..25usize]);
        }
    }
    impl Lcd128X64Bricklet {
        pub fn new(
            uid: impl Into<tinkerforge_base::base58::Uid>,
            connection: tinkerforge_base::ip_connection::async_io::AsyncIpConnection,
        ) -> Lcd128X64Bricklet {
            Self {
                device: tinkerforge_base::device::Device::new(
                    uid.into(),
                    connection,
                    "LCD 128x64",
                ),
            }
        }
        pub fn uid(&self) -> tinkerforge_base::base58::Uid {
            self.device.uid()
        }
        /**
Schreibt Pixel in das angegebene Fenster.

Die Pixel werden zeilenweise von oben nach unten geschrieben
und die Zeilen werden jeweils von links nach rechts geschrieben.

Wenn Automatic Draw aktiviert ist (Standard), dann werden die Pixel direkt auf
den Display geschrieben. Nur Pixel die sich wirklich verändert haben werden
auf dem Display aktualisiert.

Wenn Automatic Draw deaktiviert ist, dann werden die Pixel in einen internen
Buffer geschrieben der dann durch einen Aufruf von :func:`Draw Buffered Frame`
auf dem Display angezeigt werden kann. Dadurch kann Flicker vermieden werden,
wenn ein komplexes Bild in mehreren Schritten aufgebaut wird.

Automatic Draw kann über die :func:`Set Display Configuration` Funktion
eingestellt werden.
*/
        pub async fn write_pixels_low_level(
            &mut self,
            request: crate::bindings::lcd_128_x_64::WritePixelsLowLevelRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 64usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(1u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Schreibt Pixel in das angegebene Fenster.

Die Pixel werden zeilenweise von oben nach unten geschrieben
und die Zeilen werden jeweils von links nach rechts geschrieben.

Wenn Automatic Draw aktiviert ist (Standard), dann werden die Pixel direkt auf
den Display geschrieben. Nur Pixel die sich wirklich verändert haben werden
auf dem Display aktualisiert.

Wenn Automatic Draw deaktiviert ist, dann werden die Pixel in einen internen
Buffer geschrieben der dann durch einen Aufruf von :func:`Draw Buffered Frame`
auf dem Display angezeigt werden kann. Dadurch kann Flicker vermieden werden,
wenn ein komplexes Bild in mehreren Schritten aufgebaut wird.

Automatic Draw kann über die :func:`Set Display Configuration` Funktion
eingestellt werden.
*/
        pub async fn write_pixels(
            &mut self,
            request: crate::bindings::lcd_128_x_64::WritePixelsRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 4usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(1u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Liest Pixel aus dem angegebenen Fenster.

Die Pixel werden zeilenweise von oben nach unten
und die Zeilen werden jeweils von links nach rechts gelesen.

Wenn Automatic Draw aktiviert ist (Standard), dann werden die Pixel direkt vom
Display gelesen.

Wenn Automatic Draw deaktiviert ist, dann werden die Pixel aus einen internen
Buffer gelesen (siehe :func:`Draw Buffered Frame`).

Automatic Draw kann über die :func:`Set Display Configuration` Funktion
eingestellt werden.
*/
        pub async fn read_pixels_low_level(
            &mut self,
            request: crate::bindings::lcd_128_x_64::ReadPixelsLowLevelRequest,
        ) -> Result<
            crate::bindings::lcd_128_x_64::ReadPixelsLowLevelResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 4usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(2u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::ReadPixelsLowLevelResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Liest Pixel aus dem angegebenen Fenster.

Die Pixel werden zeilenweise von oben nach unten
und die Zeilen werden jeweils von links nach rechts gelesen.

Wenn Automatic Draw aktiviert ist (Standard), dann werden die Pixel direkt vom
Display gelesen.

Wenn Automatic Draw deaktiviert ist, dann werden die Pixel aus einen internen
Buffer gelesen (siehe :func:`Draw Buffered Frame`).

Automatic Draw kann über die :func:`Set Display Configuration` Funktion
eingestellt werden.
*/
        pub async fn read_pixels(
            &mut self,
            request: crate::bindings::lcd_128_x_64::ReadPixelsRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 4usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(2u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Löscht den kompletten aktuellen Inhalt des Displays.

Wenn Automatic Draw aktiviert ist (Standard), dann werden die Pixel direkt
gelöscht.

Wenn Automatic Draw deaktiviert ist, dann werden die Pixel im internen
Buffer gelöscht der dann durch einen Aufruf von :func:`Draw Buffered Frame`
auf dem Display angezeigt werden kann. Dadurch kann Flicker vermieden werden,
wenn ein komplexes Bild in mehreren Schritten aufgebaut wird.

Automatic Draw kann über die :func:`Set Display Configuration` Funktion
eingestellt werden.
*/
        pub async fn clear_display(
            &mut self,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            self.device
                .set(3u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Setzt die Konfiguration des Displays.

Wenn Automatic Draw aktiviert (*true*) ist dann wird das Display bei jedem
Aufruf von :func:`Write Pixels` und :func:`Write Line` aktualisiert. Wenn
Automatic Draw deaktiviert (*false*) ist, dann werden Änderungen in einen
internen Buffer geschrieben, der dann bei bei einem Aufruf von
:func:`Draw Buffered Frame` auf dem Display angezeigt wird.
*/
        pub async fn set_display_configuration(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetDisplayConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 4usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(4u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Konfiguration zurück, wie von :func:`Set Display Configuration` gesetzt.
*/
        pub async fn get_display_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetDisplayConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(5u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetDisplayConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Schreibt einen Text in die angegebene Zeile mit einer vorgegebenen Position.

Beispiel: (1, 10, "Hallo") schreibt *Hallo* in die Mitte der zweiten Zeile
des Displays.

Das Display nutzt einen speziellen 5x7 Pixel Zeichensatz. Der Zeichensatz
kann mit Hilfe von Brick Viewer angezeigt werden.

Wenn Automatic Draw aktiviert ist (Standard), dann wird der Text direkt auf
den Display geschrieben. Nur Pixel die sich wirklich verändert haben werden
auf dem Display aktualisiert.

Wenn Automatic Draw deaktiviert ist, dann wird der Text in einen internen
Buffer geschrieben der dann durch einen Aufruf von :func:`Draw Buffered Frame`
auf dem Display angezeigt werden kann. Dadurch kann Flicker vermieden werden,
wenn ein komplexes Bild in mehreren Schritten aufgebaut wird.

Automatic Draw kann über die :func:`Set Display Configuration` Funktion
eingestellt werden.

Diese Funktion ist ein 1:1-Ersatz für die Funktion mit dem gleichen Namen
im LCD 20x4 Bricklet. Mit der Funktion :func:`Draw Text` kann Text Pixelgenau
und mit unterschiedlichen Font-Größen gezeichnet werden.
*/
        pub async fn write_line(
            &mut self,
            request: crate::bindings::lcd_128_x_64::WriteLineRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 24usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(6u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Stellt den aktuell Inhalt des internen Buffers auf dem Display dar. Normalerweise
schreibt jeder Aufruf von :func:`Write Pixels` und :func:`Write Line` direkt auf
den Display. Wenn jedoch Automatic Draw deaktiviert ist (:func:`Set Display Configuration`),
dann werden Änderungen in einen internen Buffer anstatt auf den
Display geschrieben. Der internen Buffer kann dann durch einen Aufruf dieser
Funktion auf den Display geschrieben werden. Dadurch kann Flicker vermieden werden,
wenn ein komplexes Bild in mehreren Schritten aufgebaut wird.

Wenn `Force Complete Redraw` auf *true* gesetzt ist, dann wird der gesamte Display
aktualisiert, anstatt nur die Pixel die sich wirklich verändert haben. Normalerweise
sollte dies nicht notwendig sein, außer bei hängenden Pixeln bedingt durch Fehler.
*/
        pub async fn draw_buffered_frame(
            &mut self,
            request: bool,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(7u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die letzte gültige Touch-Position zurück:

* Pressure: Anpressdruck des Touches
* X: Touch-Position auf der X-Achse
* Y: Touch-Position auf der Y-Achse
* Age: Alter des Touches (wie lange ist die Erkennung des Touches her)
*/
        pub async fn get_touch_position(
            &mut self,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetTouchPositionResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(8u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetTouchPositionResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Die Periode ist die Periode mit der der :cb:`Touch Position` Callback
ausgelöst wird. Ein Wert von 0 schaltet den Callback ab.

Wenn der `value has to change`-Parameter auf True gesetzt wird, wird der
Callback nur ausgelöst, wenn der Wert sich im Vergleich zum letzten mal geändert
hat. Ändert der Wert sich nicht innerhalb der Periode, so wird der Callback
sofort ausgelöst, wenn der Wert sich das nächste mal ändert.

Wird der Parameter auf False gesetzt, so wird der Callback dauerhaft mit der
festen Periode ausgelöst unabhängig von den Änderungen des Werts.
*/
        pub async fn set_touch_position_callback_configuration(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetTouchPositionCallbackConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 5usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(9u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Callback-Konfiguration zurück, wie mittels
:func:`Set Touch Position Callback Configuration` gesetzt.
*/
        pub async fn get_touch_position_callback_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetTouchPositionCallbackConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(10u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetTouchPositionCallbackConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Dieser Callback wird mit der Periode, wie gesetzt mit
:func:`Set Touch Position Callback Configuration`, ausgelöst. Die :word:`parameters` sind
die gleichen wie die von :func:`Get Touch Position`.
*/
        pub async fn touch_position_stream(
            &mut self,
        ) -> impl futures_core::Stream<
            Item = crate::bindings::lcd_128_x_64::TouchPositionCallback,
        > {
            self.device
                .get_callback_receiver(11u8)
                .await
                .map(|p| TouchPositionCallback::from_le_byte_slice(p.body()))
        }
        /**
Gibt eine der vier Touch-Gesten zurück, die das Bricklet automatisch erkennen kann.

Die Gesten umfassen Wischen von links nach rechts, rechts nach links, oben nach
unten und unten nach oben.

Zusätzlich zu Geste wird der Vektor von Start- nach Endposition des Wischens
angegeben. Dieser kann genutzt werden um die genaue Position der Geste zu
ermitteln (z.B. ob ein Wischen von oben nach unten auf der linken oder rechten
des Bildschirms erkannt wurde).

Der Age Parameter gibt das Alter der Geste an (wie lange ist die Erkennung
der Geste her).
*/
        pub async fn get_touch_gesture(
            &mut self,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetTouchGestureResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(12u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetTouchGestureResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Die Periode ist die Periode mit der der :cb:`Touch Gesture` Callback
ausgelöst wird. Ein Wert von 0 schaltet den Callback ab.

Wenn der `value has to change`-Parameter auf True gesetzt wird, wird der
Callback nur ausgelöst, wenn der Wert sich im Vergleich zum letzten mal geändert
hat. Ändert der Wert sich nicht innerhalb der Periode, so wird der Callback
sofort ausgelöst, wenn der Wert sich das nächste mal ändert.

Wird der Parameter auf False gesetzt, so wird der Callback dauerhaft mit der
festen Periode ausgelöst unabhängig von den Änderungen des Werts.
*/
        pub async fn set_touch_gesture_callback_configuration(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetTouchGestureCallbackConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 5usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(13u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Callback-Konfiguration zurück, wie mittels
:func:`Set Touch Gesture Callback Configuration` gesetzt.
*/
        pub async fn get_touch_gesture_callback_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetTouchGestureCallbackConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(14u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetTouchGestureCallbackConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Dieser Callback wird mit der Periode, wie gesetzt mit
:func:`Set Touch Gesture Callback Configuration`, ausgelöst. Die :word:`parameters` sind
die gleichen wie die von :func:`Get Touch Gesture`.
*/
        pub async fn touch_gesture_stream(
            &mut self,
        ) -> impl futures_core::Stream<
            Item = crate::bindings::lcd_128_x_64::TouchGestureCallback,
        > {
            self.device
                .get_callback_receiver(15u8)
                .await
                .map(|p| TouchGestureCallback::from_le_byte_slice(p.body()))
        }
        /**
Zeichnet eine weiße oder schwarze Linie von (x, y)-start nach
(x, y)-end.
*/
        pub async fn draw_line(
            &mut self,
            request: crate::bindings::lcd_128_x_64::DrawLineRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 5usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(16u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Zeichnet ein weißes oder schwarzes Rechteck von (x, y)-start nach
(x, y)-end.

Wenn fill auf true gesetzt wird, wird das Rechteck mit
der angegebenen Farbe ausgefüllt. Ansonsten wird nur der Umriss
gezeichnet.
*/
        pub async fn draw_box(
            &mut self,
            request: crate::bindings::lcd_128_x_64::DrawBoxRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 6usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(17u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Zeichnet einen Text an die Pixelposition (x, y).

Es können 9 unterschiedliche Font-Größen genutzt werden und der Text
kann in weiß oder schwarz gezeichnet werden.

Der der Zeichensatz entspricht Codepage 437.
*/
        pub async fn draw_text(
            &mut self,
            request: crate::bindings::lcd_128_x_64::DrawTextRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 26usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(18u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Zeichnet einen klickbaren Button an Position (x, y) mit dem gegebenem Text.

Es können bis zu 12 Buttons genutzt werden.

Die x-Position + Width muss im Wertebereich von 1 bis 128 liegen und die
y-Position+Height muss im Wertebereich von 1 bis 64 liegen.

Die minimale nützliche Breite/Höhe eines Buttons ist 3.

Der Callback für Button-Events kann mit der Funktion
:func:`Set GUI Button Pressed Callback Configuration` eingestellt werden.
Der Callback wird sowohl für gedrückt als auch losgelassen Events ausgelöst.

Der Button wird in einem separaten GUI-Buffer gezeichnet und der Rahmen des
Buttons wird immer über den Grafiken bleiben die mit :func:`Write Pixels`
gezeichnet werden. Um einen Button zu entfernen kann die Funktion
:func:`Remove GUI Button` genutzt werden.

Wenn anstatt des Textes ein Icon verwendet werden soll, kann dieses innerhalb
des Buttons mit per :func:`Write Pixels` gezeichnet werden.
*/
        pub async fn set_gui_button(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetGuiButtonRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 21usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(19u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Button-Eigenschaften für den gegebenen `Index` zurück, wie von
:func:`Set GUI Button` gesetzt.

Zusätzlich gibt der `Active`-Parameter an ob der Button aktuell aktiv/sichtbar ist
oder nicht.
*/
        pub async fn get_gui_button(
            &mut self,
            request: u8,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetGuiButtonResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(20u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetGuiButtonResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Entfernt den Button mit dem gegebenen Index.

Index 255 kann genutzt werden um alle Buttons zu entfernen.
*/
        pub async fn remove_gui_button(
            &mut self,
            request: u8,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(21u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Die Periode ist die Periode mit der der :cb:`GUI Button Pressed` Callback
ausgelöst wird. Ein Wert von 0 schaltet den Callback ab.

Wenn der `value has to change`-Parameter auf True gesetzt wird, wird der
Callback nur ausgelöst, wenn der Wert sich im Vergleich zum letzten mal geändert
hat. Ändert der Wert sich nicht innerhalb der Periode, so wird der Callback
sofort ausgelöst, wenn der Wert sich das nächste mal ändert.

Wird der Parameter auf False gesetzt, so wird der Callback dauerhaft mit der
festen Periode ausgelöst unabhängig von den Änderungen des Werts.
*/
        pub async fn set_gui_button_pressed_callback_configuration(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetGuiButtonPressedCallbackConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 5usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(22u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Callback-Konfiguration zurück, wie mittels
:func:`Set GUI Button Pressed Callback Configuration` gesetzt.
*/
        pub async fn get_gui_button_pressed_callback_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetGuiButtonPressedCallbackConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(23u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetGuiButtonPressedCallbackConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Gibt den aktuellen Button-Zustand für einen gegebenen Index zurück.

Der Zustand kann entweder gedrückt (true) oder losgelassen (false) sein.
*/
        pub async fn get_gui_button_pressed(
            &mut self,
            request: u8,
        ) -> Result<bool, tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(24u8, &payload).await?;
            Ok(bool::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Dieser Callback wird mit der Periode, wie gesetzt mit
:func:`Set GUI Button Pressed Callback Configuration`, ausgelöst. Die :word:`parameters` sind
die gleichen wie die von :func:`Get GUI Button Pressed`.
*/
        pub async fn gui_button_pressed_stream(
            &mut self,
        ) -> impl futures_core::Stream<
            Item = crate::bindings::lcd_128_x_64::GuiButtonPressedCallback,
        > {
            self.device
                .get_callback_receiver(25u8)
                .await
                .map(|p| GuiButtonPressedCallback::from_le_byte_slice(p.body()))
        }
        /**
Zeichnet einen Slider an Position (x, y) mit der gegebenen Länge.

Es können bis zu 6 Slider genutzt werden.

Wenn eine horizontale Richtung verwendet wird muss Die x-Position + Länge
im Wertebereich von 1 bis 128 und die y-Position im Wertebereich von
0 bis 46 liegen.

Wenn eine vertikale Richtung verwendet wird muss Die y-Position + Länge
im Wertebereich von 1 bis 64 und die x-Position im Wertebereich von
0 bis 110 liegen.

Die minimale Länge des Sliders ist 8.

Der :word:`parameter` value ist die Startposition des Sliders. Diese kann
zwischen 0 und length-8 liegen.

Der Callback für Slider-Events kann mit der Funktion
:func:`Set GUI Slider Value Callback Configuration` eingestellt werden.

Der Slider wird in einem separaten GUI-Buffer gezeichnet und der Rahmen des
Buttons wrd immer über den Grafiken bleiben die mit :func:`Write Pixels`
gezeichnet werden. Um einen Button zu entfernen kann die Funktion
:func:`Remove GUI Slider` genutzt werden.
*/
        pub async fn set_gui_slider(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetGuiSliderRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 6usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(26u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Slider-Eigenschaften für den gegebenen `Index` zurück, wie von
:func:`Set GUI Slider` gesetzt.

Zusätzlich gibt der `Active`-Parameter an ob der Button aktuell aktiv/sichtbar ist
oder nicht.
*/
        pub async fn get_gui_slider(
            &mut self,
            request: u8,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetGuiSliderResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(27u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetGuiSliderResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Entfernt den Slider mit dem gegebenen Index.

Index 255 kann genutzt werden um alle Slider zu entfernen.
*/
        pub async fn remove_gui_slider(
            &mut self,
            request: u8,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(28u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Die Periode ist die Periode mit der der :cb:`GUI Slider Value` Callback
ausgelöst wird. Ein Wert von 0 schaltet den Callback ab.

Wenn der `value has to change`-Parameter auf True gesetzt wird, wird der
Callback nur ausgelöst, wenn der Wert sich im Vergleich zum letzten mal geändert
hat. Ändert der Wert sich nicht innerhalb der Periode, so wird der Callback
sofort ausgelöst, wenn der Wert sich das nächste mal ändert.

Wird der Parameter auf False gesetzt, so wird der Callback dauerhaft mit der
festen Periode ausgelöst unabhängig von den Änderungen des Werts.
*/
        pub async fn set_gui_slider_value_callback_configuration(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetGuiSliderValueCallbackConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 5usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(29u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Callback-Konfiguration zurück, wie mittels
:func:`Set GUI Slider Value Callback Configuration` gesetzt.
*/
        pub async fn get_gui_slider_value_callback_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetGuiSliderValueCallbackConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(30u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetGuiSliderValueCallbackConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Gibt den aktuellen Wert des Slider mit dem gegebenen Index zurück.
*/
        pub async fn get_gui_slider_value(
            &mut self,
            request: u8,
        ) -> Result<u8, tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(31u8, &payload).await?;
            Ok(u8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Dieser Callback wird mit der Periode, wie gesetzt mit
:func:`Set GUI Slider Value Callback Configuration`, ausgelöst. Die :word:`parameters` sind
die gleichen wie die von :func:`Get GUI Slider Value`.
*/
        pub async fn gui_slider_value_stream(
            &mut self,
        ) -> impl futures_core::Stream<
            Item = crate::bindings::lcd_128_x_64::GuiSliderValueCallback,
        > {
            self.device
                .get_callback_receiver(32u8)
                .await
                .map(|p| GuiSliderValueCallback::from_le_byte_slice(p.body()))
        }
        /**
Setzt die generelle Konfiguration für Tabs. Tabs können auf klicken, wischen
(links/rechts und rechts/links) oder beides reagieren.

Zusätzlich kann `Clear GUI` auf true gesetzt werden. In diesem Fall werden
bei einem wechsel der Tabs automatisch alle GUI Elemente (Buttons, Slider,
Graphen) gelöscht.
*/
        pub async fn set_gui_tab_configuration(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetGuiTabConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 2usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(33u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Tab-Konfiguration zurück, wie von :func:`Set GUI Tab Configuration` gesetzt.
*/
        pub async fn get_gui_tab_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetGuiTabConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(34u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetGuiTabConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Fügt einen Text-Tab mit dem gegebenen Index hinzu.

Es können bis zu 10 Tabs verwendet werden.

Ein Text-Tab mit dem gleichen Index wie ein Icon-Tab überschreibt diesen.
*/
        pub async fn set_gui_tab_text(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetGuiTabTextRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 6usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(35u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt den Text für den gegebenen Index zurück, wie von :func:`Set GUI Tab Text`
gesetzt.

Zusätzlich gibt der `Active`-Parameter an ob der Tab aktuell aktiv/sichtbar ist
oder nicht.
*/
        pub async fn get_gui_tab_text(
            &mut self,
            request: u8,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetGuiTabTextResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(36u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetGuiTabTextResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Fügt einen Icon-Tab mit dem gegebenen Index hinzu. Das Icon kann eine Breite von
28 Pixel bei einer Höhe von 6 Pixel haben. Es wird Zeile für Zeile von links
nach rechts gezeichnet.

Es können bis zu 10 Tabs verwendet werden.

Ein Icon-Tab mit dem gleichen Index wie ein Text-Tab überschreibt diesen.
*/
        pub async fn set_gui_tab_icon(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetGuiTabIconRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 22usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(37u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt das Icon für den gegebenen Index zurück, wie von :func:`Set GUI Tab Icon`
gesetzt.

Zusätzlich gibt der `Active`-Parameter an ob der Tab aktuell aktiv/sichtbar ist
oder nicht.
*/
        pub async fn get_gui_tab_icon(
            &mut self,
            request: u8,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetGuiTabIconResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(38u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetGuiTabIconResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Entfernt den Tab mit dem gegebenen Index.

Index 255 kann genutzt werden um alle Tabs zu entfernen.
*/
        pub async fn remove_gui_tab(
            &mut self,
            request: u8,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(39u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Setzt den Tab mit dem gegebenen Index als "selected" (wird auf dem Display als
ausgewählt gezeichnet)
*/
        pub async fn set_gui_tab_selected(
            &mut self,
            request: u8,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(40u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Die Periode ist die Periode mit der der :cb:`GUI Tab Selected` Callback
ausgelöst wird. Ein Wert von 0 schaltet den Callback ab.

Wenn der `value has to change`-Parameter auf True gesetzt wird, wird der
Callback nur ausgelöst, wenn der Wert sich im Vergleich zum letzten mal geändert
hat. Ändert der Wert sich nicht innerhalb der Periode, so wird der Callback
sofort ausgelöst, wenn der Wert sich das nächste mal ändert.

Wird der Parameter auf False gesetzt, so wird der Callback dauerhaft mit der
festen Periode ausgelöst unabhängig von den Änderungen des Werts.
*/
        pub async fn set_gui_tab_selected_callback_configuration(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetGuiTabSelectedCallbackConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 5usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(41u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Callback-Konfiguration zurück, wie mittels
:func:`Set GUI Tab Selected Callback Configuration` gesetzt.
*/
        pub async fn get_gui_tab_selected_callback_configuration(
            &mut self,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetGuiTabSelectedCallbackConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(42u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetGuiTabSelectedCallbackConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Gibt den Index des aktuell ausgewählten Tabs zurück.
Wenn es keine Tabs gibt, wird -1 als Index zurückgegeben.
*/
        pub async fn get_gui_tab_selected(
            &mut self,
        ) -> Result<i8, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(43u8, &payload).await?;
            Ok(i8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Dieser Callback wird mit der Periode, wie gesetzt mit
:func:`Set GUI Tab Selected Callback Configuration`, ausgelöst. Die :word:`parameters` sind
die gleichen wie die von :func:`Get GUI Tab Selected`.
*/
        pub async fn gui_tab_selected_stream(
            &mut self,
        ) -> impl futures_core::Stream<Item = i8> {
            self.device
                .get_callback_receiver(44u8)
                .await
                .map(|p| i8::from_le_byte_slice(&p.body()[0..1usize]))
        }
        /**
Setzt die Konfiguration für bis zu vier Graphen.

Der Graph kann vom Typ Dot-, Line- oder Bar-Graph sein.

Die x- und y-Positionen sind Pixel-Positionen.

Es können bis zu 4 Buchstaben Text zur Beschreibung der x- und y-Achse
genutzt werden. Der Text wird auf die Innenseite des Graphen gezeichnet und
er kann Datenpunkte des Graphen überschreiben. Wenn der Text außerhalb des
Graphen benötigt wird kann die Beschriftung hier leer gelassen werden. Der
Text kann im Nachhinein mit :func:`Draw Text` hinzugefügt werden.

Die Datenpunkte des Graphen können mit der Funktion :func:`Set GUI Graph Data`
gesetzt und aktualisiert werden.

Der Graph wird in einem separaten GUI-Buffer gezeichnet und der Rahmen sowie die
Datenpunkte des Graphen werden immer über den Grafiken bleiben die mit
:func:`Write Pixels` gezeichnet werden. Um einen Graphen zu entfernen kann die
Funktion :func:`Remove GUI Graph` genutzt werden.
*/
        pub async fn set_gui_graph_configuration(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetGuiGraphConfigurationRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 14usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(45u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Graph-Eigenschaften für den gegebenen `Index` zurück, wie von
:func:`Set GUI Graph Configuration` gesetzt.

Zusätzlich gibt der `Active`-Parameter an ob der Button aktuell aktiv/sichtbar ist
oder nicht.
*/
        pub async fn get_gui_graph_configuration(
            &mut self,
            request: u8,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetGuiGraphConfigurationResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(46u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetGuiGraphConfigurationResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt die Datenpukte für den Graph mit dem gegebenen Index. Der Graph muss mit
:func:`Set GUI Graph Configuration` konfiguriert werden bevor die ersten Daten
gesetzt werden können.

Der Graph zeigt die ersten n Werte der gesetzten Daten an, wobei n die Breite (width)
ist die mit :func:`Set GUI Graph Configuration` gesetzt wurde. Wenn weniger als
n Werte gesetzt werden, werden die restlichen Datenpunkte als 0 angezeigt.

Die maximale Anzahl an Datenpunkte die gesetzt werden kann ist 118 (dies entspricht
auch der maximalen Breite des Graphen).

Die gesetzten Werte müssen zwischen 0 und 255 skaliert werden. 0 wird unten und
255 wird oben im Graph gezeichnet.
*/
        pub async fn set_gui_graph_data_low_level(
            &mut self,
            request: crate::bindings::lcd_128_x_64::SetGuiGraphDataLowLevelRequest,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 64usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(47u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Setzt die Datenpukte für den Graph mit dem gegebenen Index. Der Graph muss mit
:func:`Set GUI Graph Configuration` konfiguriert werden bevor die ersten Daten
gesetzt werden können.

Der Graph zeigt die ersten n Werte der gesetzten Daten an, wobei n die Breite (width)
ist die mit :func:`Set GUI Graph Configuration` gesetzt wurde. Wenn weniger als
n Werte gesetzt werden, werden die restlichen Datenpunkte als 0 angezeigt.

Die maximale Anzahl an Datenpunkte die gesetzt werden kann ist 118 (dies entspricht
auch der maximalen Breite des Graphen).

Die gesetzten Werte müssen zwischen 0 und 255 skaliert werden. 0 wird unten und
255 wird oben im Graph gezeichnet.
*/
        pub async fn set_gui_graph_data(
            &mut self,
            request: u8,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(47u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Datenpunkte des Graphen mit dem gegebenen Index zurück, wie von
:func:`Set GUI Graph Data` gesetzt.
*/
        pub async fn get_gui_graph_data_low_level(
            &mut self,
            request: u8,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetGuiGraphDataLowLevelResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(48u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetGuiGraphDataLowLevelResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Gibt die Datenpunkte des Graphen mit dem gegebenen Index zurück, wie von
:func:`Set GUI Graph Data` gesetzt.
*/
        pub async fn get_gui_graph_data(
            &mut self,
            request: u8,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(48u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Entfernt den Graph mit dem gegebenen Index.

Index 255 kann genutzt werden um alle Graphen zu entfernen.
*/
        pub async fn remove_gui_graph(
            &mut self,
            request: u8,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(49u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Entfernt alle GUI-Elemente (Buttons, Slider, Graphen, Tabs).
*/
        pub async fn remove_all_gui(
            &mut self,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            self.device
                .set(50u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Setzt die Konfiguration der Kommunikations-LED. Standardmäßig ist die
LED an wenn das LCD berührt wird.

Die LED kann auch permanent an/aus gestellt werden oder einen Herzschlag anzeigen.

Wenn das Bricklet sich im Bootloadermodus befindet ist die LED aus.
*/
        pub async fn set_touch_led_config(
            &mut self,
            request: crate::bindings::lcd_128_x_64::TouchLedConfig,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(51u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Konfiguration zurück, wie von :func:`Set Touch LED Config` gesetzt.
*/
        pub async fn get_touch_led_config(
            &mut self,
        ) -> Result<
            tinkerforge_base::byte_converter::ParsedOrRaw<
                crate::bindings::lcd_128_x_64::TouchLedConfig,
                u8,
            >,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(52u8, &payload).await?;
            Ok(
                tinkerforge_base::byte_converter::ParsedOrRaw::<
                    crate::bindings::lcd_128_x_64::TouchLedConfig,
                    u8,
                >::from_le_byte_slice(&result.body()[0..1usize]),
            )
        }
        /**
Gibt die Anzahl der Fehler die während der Kommunikation zwischen Brick und
Bricklet aufgetreten sind zurück.

Die Fehler sind aufgeteilt in

* ACK-Checksummen Fehler,
* Message-Checksummen Fehler,
* Framing Fehler und
* Overflow Fehler.

Die Fehlerzähler sind für Fehler die auf der Seite des Bricklets auftreten.
Jedes Brick hat eine ähnliche Funktion welche die Fehler auf Brickseite
ausgibt.
*/
        pub async fn get_spitfp_error_count(
            &mut self,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetSpitfpErrorCountResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(234u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetSpitfpErrorCountResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
        /**
Setzt den Bootloader-Modus und gibt den Status zurück nachdem die
Modusänderungsanfrage bearbeitet wurde.

Mit dieser Funktion ist es möglich vom Bootloader- in den Firmware-Modus zu
wechseln und umgekehrt. Ein Welchsel vom Bootloader- in der den Firmware-Modus
ist nur möglich wenn Entry-Funktion, Device Identifier und CRC vorhanden und
korrekt sind.

Diese Funktion wird vom Brick Viewer während des Flashens benutzt. In einem
normalem Nutzerprogramm sollte diese Funktion nicht benötigt werden.
*/
        pub async fn set_bootloader_mode(
            &mut self,
            request: crate::bindings::lcd_128_x_64::BootloaderMode,
        ) -> Result<
            tinkerforge_base::byte_converter::ParsedOrRaw<
                crate::bindings::lcd_128_x_64::BootloaderStatus,
                u8,
            >,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(235u8, &payload).await?;
            Ok(
                tinkerforge_base::byte_converter::ParsedOrRaw::<
                    crate::bindings::lcd_128_x_64::BootloaderStatus,
                    u8,
                >::from_le_byte_slice(&result.body()[0..1usize]),
            )
        }
        /**
Gibt den aktuellen Bootloader-Modus zurück, siehe :func:`Set Bootloader Mode`.
*/
        pub async fn get_bootloader_mode(
            &mut self,
        ) -> Result<
            tinkerforge_base::byte_converter::ParsedOrRaw<
                crate::bindings::lcd_128_x_64::BootloaderMode,
                u8,
            >,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(236u8, &payload).await?;
            Ok(
                tinkerforge_base::byte_converter::ParsedOrRaw::<
                    crate::bindings::lcd_128_x_64::BootloaderMode,
                    u8,
                >::from_le_byte_slice(&result.body()[0..1usize]),
            )
        }
        /**
Setzt den Firmware-Pointer für :func:`Write Firmware`. Der Pointer
muss um je 64 Byte erhöht werden. Die Daten werden alle 4 Datenblöcke
in den Flash geschrieben (4 Datenblöcke entsprechen einer Page mit 256 Byte).

Diese Funktion wird vom Brick Viewer während des Flashens benutzt. In einem
normalem Nutzerprogramm sollte diese Funktion nicht benötigt werden.
*/
        pub async fn set_write_firmware_pointer(
            &mut self,
            request: u32,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 4usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(237u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Schreibt 64 Bytes Firmware an die Position die vorher von
:func:`Set Write Firmware Pointer` gesetzt wurde. Die Firmware wird
alle 4 Datenblöcke in den Flash geschrieben.

Eine Firmware kann nur im Bootloader-Mode geschrieben werden.

Diese Funktion wird vom Brick Viewer während des Flashens benutzt. In einem
normalem Nutzerprogramm sollte diese Funktion nicht benötigt werden.
*/
        pub async fn write_firmware(
            &mut self,
            request: [u8; 64usize],
        ) -> Result<u8, tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 64usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            let result = self.device.get(238u8, &payload).await?;
            Ok(u8::from_le_byte_slice(&result.body()[0..1usize]))
        }
        /**
Setzt die Konfiguration der Status-LED. Standardmäßig zeigt
die LED die Kommunikationsdatenmenge an. Sie blinkt einmal auf pro 10 empfangenen
Datenpaketen zwischen Brick und Bricklet.

Die LED kann auch permanent an/aus gestellt werden oder einen Herzschlag anzeigen.

Wenn das Bricklet sich im Bootlodermodus befindet ist die LED aus.
*/
        pub async fn set_status_led_config(
            &mut self,
            request: crate::bindings::lcd_128_x_64::StatusLedConfig,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 1usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(239u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die Konfiguration zurück, wie von :func:`Set Status LED Config` gesetzt.
*/
        pub async fn get_status_led_config(
            &mut self,
        ) -> Result<
            tinkerforge_base::byte_converter::ParsedOrRaw<
                crate::bindings::lcd_128_x_64::StatusLedConfig,
                u8,
            >,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(240u8, &payload).await?;
            Ok(
                tinkerforge_base::byte_converter::ParsedOrRaw::<
                    crate::bindings::lcd_128_x_64::StatusLedConfig,
                    u8,
                >::from_le_byte_slice(&result.body()[0..1usize]),
            )
        }
        /**
Gibt die Temperatur, gemessen im Mikrocontroller, aus. Der
Rückgabewert ist nicht die Umgebungstemperatur.

Die Temperatur ist lediglich proportional zur echten Temperatur und hat eine
hohe Ungenauigkeit. Daher beschränkt sich der praktische Nutzen auf die
Indikation von Temperaturveränderungen.
*/
        pub async fn get_chip_temperature(
            &mut self,
        ) -> Result<i16, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(242u8, &payload).await?;
            Ok(i16::from_le_byte_slice(&result.body()[0..2usize]))
        }
        /**
Ein Aufruf dieser Funktion setzt das Bricklet zurück. Nach einem
Neustart sind alle Konfiguration verloren.

Nach dem Zurücksetzen ist es notwendig neue Objekte zu erzeugen,
Funktionsaufrufe auf bestehenden führen zu undefiniertem Verhalten.
*/
        pub async fn reset(
            &mut self,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            self.device
                .set(243u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Schreibt eine neue UID in den Flash. Die UID muss zuerst
vom Base58 encodierten String in einen Integer decodiert
werden.

Wir empfehlen die Nutzung des Brick Viewers zum ändern
der UID.
*/
        pub async fn write_uid(
            &mut self,
            request: u32,
        ) -> Result<(), tinkerforge_base::error::TinkerforgeError> {
            let mut payload = [0; 4usize];
            tinkerforge_base::byte_converter::ToBytes::write_to_slice(
                request,
                &mut payload,
            );
            self.device
                .set(248u8, &payload, Some(std::time::Duration::from_secs(20)))
                .await?;
            Ok(())
        }
        /**
Gibt die aktuelle UID als Integer zurück. Dieser Integer
kann als Base58 encodiert werden um an den üblichen
UID-String zu gelangen.
*/
        pub async fn read_uid(
            &mut self,
        ) -> Result<u32, tinkerforge_base::error::TinkerforgeError> {
            let payload = [0; 0usize];
            let result = self.device.get(249u8, &payload).await?;
            Ok(u32::from_le_byte_slice(&result.body()[0..4usize]))
        }
        /**
Gibt die UID, die UID zu der das Bricklet verbunden ist, die
Position, die Hard- und Firmware Version sowie den Device Identifier
zurück.

Die Position ist 'a', 'b', 'c', 'd', 'e', 'f', 'g' oder 'h' (Bricklet Anschluss).
Ein Bricklet hinter einem :ref:`Isolator Bricklet <isolator_bricklet>` ist immer an
Position 'z'.

Eine Liste der Device Identifier Werte ist :ref:`hier <device_identifier>` zu
finden. |device_identifier_constant|
*/
        pub async fn get_identity(
            &mut self,
        ) -> Result<
            crate::bindings::lcd_128_x_64::GetIdentityResponse,
            tinkerforge_base::error::TinkerforgeError,
        > {
            let payload = [0; 0usize];
            let result = self.device.get(255u8, &payload).await?;
            Ok(
                crate::bindings::lcd_128_x_64::GetIdentityResponse::from_le_byte_slice(
                    result.body(),
                ),
            )
        }
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum DeviceIdentifier {
    MasterBrick,
    Lcd128X64Bricklet,
}
impl DeviceIdentifier {
    pub fn name(self) -> &'static str {
        match self {
            DeviceIdentifier::MasterBrick => "Master",
            DeviceIdentifier::Lcd128X64Bricklet => "LCD 128x64",
        }
    }
}
impl Into<u16> for DeviceIdentifier {
    fn into(self) -> u16 {
        match self {
            DeviceIdentifier::MasterBrick => 13u16,
            DeviceIdentifier::Lcd128X64Bricklet => 298u16,
        }
    }
}
impl std::convert::TryInto<DeviceIdentifier> for u16 {
    type Error = ();
    fn try_into(self) -> Result<DeviceIdentifier, Self::Error> {
        match self {
            13u16 => Ok(DeviceIdentifier::MasterBrick),
            298u16 => Ok(DeviceIdentifier::Lcd128X64Bricklet),
            _ => Err(()),
        }
    }
}
