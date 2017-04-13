#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum OperatingSystem {
    Arch,
    Debian,
    EL,
    OracleLinux,
    OpenSUSE,
    Ubuntu
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum OperatingSystemVersion {
    Latest,
    Wheezy,
    Stretch,
    Jessie,
    Seven,
    Six,
    Tumbleweed,
    Leap,
    Yakkety,
    Xenial,
    Trusty,
    Precise
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct VersionedOperatingSystem {
    os : OperatingSystem,
    version: OperatingSystemVersion
}


#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum CommandError{
    OSError,
    OSVersionError
}


fn get_versioned_os(operatingsystem: String, version: String) -> Result<VersionedOperatingSystem, CommandError> {
    get_os(operatingsystem)
    .and_then( |os| get_os_version(os, version))
}

fn get_os(os: String) -> Result<OperatingSystem,CommandError> {
    Ok(OperatingSystem::Ubuntu)
}

fn get_os_version(os: OperatingSystem, version: String) -> Result<VersionedOperatingSystem, CommandError> {
    match os {
        OperatingSystem::Arch => {
            let vos = VersionedOperatingSystem{ os: os , version: OperatingSystemVersion::Latest };
            Ok(vos)
        },
        OperatingSystem::Debian => {
            let vos = VersionedOperatingSystem{ os : os , version: OperatingSystemVersion::Jessie};
            Ok(vos)
        },
        OperatingSystem::EL => {
            let vos = VersionedOperatingSystem{ os: os, version: OperatingSystemVersion::Seven };
            Ok(vos)
        },
        OperatingSystem::OracleLinux => {
            let vos = VersionedOperatingSystem{ os : os, version: OperatingSystemVersion::Seven};
            Ok(vos)
        }
        OperatingSystem::OpenSUSE => {
            let vos = VersionedOperatingSystem{ os:os , version: OperatingSystemVersion::Leap };
            Ok(vos)
        }
        OperatingSystem::Ubuntu => {
            let vos = VersionedOperatingSystem{ os: os, version: OperatingSystemVersion::Xenial};
            Ok(vos)
        }
    }
}

fn operating_system_string(os: OperatingSystem) -> String {
    match os {
        OperatingSystem::Arch => "archlinux".to_string(),
        OperatingSystem::Debian => "debian".to_string(),
        OperatingSystem::EL => "el".to_string(),
        OperatingSystem::OracleLinux => "oraclelinux".to_string(),
        OperatingSystem::OpenSUSE => "opensuse".to_string(),
        OperatingSystem::Ubuntu => "ubuntu".to_string()
    }
}

fn operation_system_version_string(version: OperatingSystemVersion) -> String {
    match version {
        OperatingSystemVersion::Latest      => "latest".to_string(),
        OperatingSystemVersion::Wheezy      => "wheezy".to_string(),
        OperatingSystemVersion::Stretch     => "stretch".to_string(),
        OperatingSystemVersion::Jessie      => "jessie".to_string(),
        OperatingSystemVersion::Seven       => "7".to_string(),
        OperatingSystemVersion::Six         => "6".to_string(),
        OperatingSystemVersion::Tumbleweed  => "tumbleweed".to_string(),
        OperatingSystemVersion::Leap        => "42.2".to_string(),
        OperatingSystemVersion::Yakkety     => "yakkety".to_string(),
        OperatingSystemVersion::Xenial      => "xenial".to_string(),
        OperatingSystemVersion::Trusty      => "trusty".to_string(),
        OperatingSystemVersion::Precise     => "precise".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_os_const_ubuntu() {
        assert_eq!(Ok(OperatingSystem::Ubuntu), get_os("yellow".to_string()));
    }

    #[test]
    fn get_os_ok(){
        assert!(get_os("yellow".to_string()).is_ok());
    }

}
