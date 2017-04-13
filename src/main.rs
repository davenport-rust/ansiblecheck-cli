#[macro_use]
extern crate clap;
use clap::{Arg, App};
// use clap::SubCommand;
use std::env;
// use std::process::Command;
// use std::process::exit;
use std::fs;



fn main() {
    let matches = App::new("ansiblecheck")
                          .version("0.1.0")
                          .author("Christopher Davenport <Chris@ChristopherDavenport.tech>")
                          .about("Checks Ansible Modules")
                          .arg(Arg::with_name("os")
                               .short("s")
                               .long("os")
                               .value_name("name")
                               .help("Sets the operating system to test")
                               .takes_value(true)
                               .required(true)
                               .possible_values(&OperatingSystem::variants())
                            //    .default_value("All")
                           )
                           .arg(Arg::with_name("os_version")
                                .short("v")
                                .long("osv")
                                .value_name("name")
                                .help("Sets the operating system version to test")
                                .takes_value(true)
                                .required(true)
                                .possible_values(&OperatingSystemVersion::variants())
                                // .default_value("All")
                            )
                        //   .arg(Arg::with_name("INPUT")
                        //        .help("Sets the input file to use")
                        //        .required(true)
                        //        .index(1))
                        //   .arg(Arg::with_name("v")
                        //        .short("v")
                        //        .multiple(true)
                        //        .help("Sets the level of verbosity"))
                        //   .subcommand(SubCommand::with_name("test")
                        //               .about("controls testing features")
                        //               .version("1.3")
                        //               .author("Someone E. <someone_else@other.com>")
                        //               .arg(Arg::with_name("debug")
                        //                   .short("d")
                        //                   .help("print debug information verbosely")))
                          .get_matches();


    let os = value_t!(matches.value_of("os"), OperatingSystem).unwrap_or_else(|e| e.exit());
    let osv = value_t!(matches.value_of("os_version"), OperatingSystemVersion).unwrap_or_else(|e| e.exit());
    let pwd = env::current_dir().unwrap();

    if operating_system_exists(os, osv){
        // println!("Value for os: {}", os);
        // println!("Using os version: {}", matches.value_of("os_version").unwrap());


        println!("Current directory is: {}", pwd.display());

        // let subfolder = "/tests/test.yml";

        let paths = fs::read_dir("./tests").unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }
    } else {
        println!()
    }

    // let mut output = Command::new("docker")
    //         .arg("run")
    //         .arg("-a")
    //         .spawn()
    //         .expect("ls command failed to start");
}

// fn operating_system_string(os: OperatingSystem) -> String {
//     match os {
//         OperatingSystem::Arch => "archlinux".to_string(),
//         OperatingSystem::Debian => "debian".to_string(),
//         OperatingSystem::EL => "el".to_string(),
//         OperatingSystem::OracleLinux => "oraclelinux".to_string(),
//         OperatingSystem::OpenSUSE => "opensuse".to_string(),
//         OperatingSystem::Ubuntu => "ubuntu".to_string(),
//         // OperatingSystem::All => "all".to_string()
//     }
// }
//
// fn operation_system_version_string(version: OperatingSystemVersion) -> String {
//     match version {
//         OperatingSystemVersion::Latest => "latest".to_string(),
//         OperatingSystemVersion::Wheezy => "wheezy".to_string(),
//         OperatingSystemVersion::Stretch => "stretch".to_string(),
//         OperatingSystemVersion::Jessie => "jessie".to_string(),
//         OperatingSystemVersion::Seven => "7".to_string(),
//         OperatingSystemVersion::Six => "6".to_string(),
//         OperatingSystemVersion::Tumbleweed => "tumbleweed".to_string(),
//         OperatingSystemVersion::Leap => "42.2".to_string(),
//         OperatingSystemVersion::Yakkety => "yakkety".to_string(),
//         OperatingSystemVersion::Xenial => "xenial".to_string(),
//         OperatingSystemVersion::Trusty => "trusty".to_string(),
//         OperatingSystemVersion::Precise => "precise".to_string(),
//         // OperatingSystemVersion::All => "all".to_string()
//     }
// }



arg_enum!{
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
    pub enum OperatingSystem {
        // All,
        Arch,
        Debian,
        EL,
        OracleLinux,
        OpenSUSE,
        Ubuntu
    }
}

arg_enum!{
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
    pub enum OperatingSystemVersion {
        // All,
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
}

// #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
// pub struct VersionedOperatingSystem {
//     os: OperatingSystem,
//     version: OperatingSystemVersion
// }


// #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
// pub enum CommandError {
//     OSError,
//     OSVersionError,
// }


// fn get_versioned_os(
//     operatingsystem: String,
//     version: String,
// ) -> Result<VersionedOperatingSystem, CommandError> {
//     get_os(operatingsystem).and_then(|os| get_os_version(os, version))
// }
//
// fn get_os(os: String) -> Result<OperatingSystem, CommandError> {
//     Ok(OperatingSystem::Ubuntu)
// }
//
// fn get_os_version(
//     os: OperatingSystem,
//     version: String,
// ) -> Result<VersionedOperatingSystem, CommandError> {
//     match os {
//         OperatingSystem::Arch => {
//             let vos = VersionedOperatingSystem {
//                 os: os,
//                 version: OperatingSystemVersion::Latest,
//             };
//             Ok(vos)
//         }
//         OperatingSystem::Debian => {
//             let vos = VersionedOperatingSystem {
//                 os: os,
//                 version: OperatingSystemVersion::Jessie,
//             };
//             Ok(vos)
//         }
//         OperatingSystem::EL => {
//             let vos = VersionedOperatingSystem {
//                 os: os,
//                 version: OperatingSystemVersion::Seven,
//             };
//             Ok(vos)
//         }
//         OperatingSystem::OracleLinux => {
//             let vos = VersionedOperatingSystem {
//                 os: os,
//                 version: OperatingSystemVersion::Seven,
//             };
//             Ok(vos)
//         }
//         OperatingSystem::OpenSUSE => {
//             let vos = VersionedOperatingSystem {
//                 os: os,
//                 version: OperatingSystemVersion::Leap,
//             };
//             Ok(vos)
//         }
//         OperatingSystem::Ubuntu => {
//             let vos = VersionedOperatingSystem {
//                 os: os,
//                 version: OperatingSystemVersion::Xenial,
//             };
//             Ok(vos)
//         }
//     }
// }
// #[cfg(test)]
// #[macro_use]
// extern crate quickcheck;
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     quickcheck! {
//       fn get_os_returns_ubuntu(xs: String) -> bool {
//           Ok(OperatingSystem::Ubuntu) == get_os(xs)
//       }
//     }
//
//     quickcheck! {
//          fn get_os_returns_ok(xs: String) -> bool {
//              get_os(xs).is_ok()
//          }
//      }
//
// }


fn operating_system_exists(os: OperatingSystem, osv: OperatingSystemVersion) -> bool {
    let osVec = vec![
        (OperatingSystem::Arch, OperatingSystemVersion::Latest),
        (OperatingSystem::Debian, OperatingSystemVersion::Jessie),
        (OperatingSystem::Debian, OperatingSystemVersion::Stretch),
        (OperatingSystem::Debian, OperatingSystemVersion::Wheezy),
        (OperatingSystem::EL, OperatingSystemVersion::Seven),
        (OperatingSystem::EL, OperatingSystemVersion::Six),
        (OperatingSystem::OracleLinux, OperatingSystemVersion::Seven),
        (OperatingSystem::OracleLinux, OperatingSystemVersion::Six),
        (OperatingSystem::OpenSUSE, OperatingSystemVersion::Leap),
        (OperatingSystem::OpenSUSE, OperatingSystemVersion::Tumbleweed),
        (OperatingSystem::Ubuntu, OperatingSystemVersion::Yakkety),
        (OperatingSystem::Ubuntu, OperatingSystemVersion::Xenial),
        (OperatingSystem::Ubuntu, OperatingSystemVersion::Precise),
        (OperatingSystem::Ubuntu, OperatingSystemVersion::Trusty)
    ];
    osVec.contains(&(os, osv))
}
