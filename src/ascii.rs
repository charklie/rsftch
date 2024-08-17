use crate::info::{os_pretty_name, uname_s};

macro_rules! ascii {
    ($overriden_ascii:expr, $search_string:expr, $return_string:expr) => {{
        let os_name = $overriden_ascii
            .clone()
            .map(|o| o.to_ascii_lowercase())
            .unwrap_or_else(|| {
                os_pretty_name(None, "ID")
                    .unwrap_or_default()
                    .to_ascii_lowercase()
            });
        let uname = uname_s($overriden_ascii.clone()).to_ascii_lowercase();

        if os_name.contains(&$search_string.to_ascii_lowercase())
            || uname.contains(&$search_string.to_ascii_lowercase())
        {
            $return_string.to_string()
        } else {
            "".to_string()
        }
    }};
}

pub fn ascii_test() {
    let distros = [
        "Arch Linux",
        "Debian",
        "Fedora",
        "EndeavourOS",
        "Void",
        "Ubuntu",
        "Suse",
        "Raspbian",
        "Linux Mint",
        "MX Linux",
        "Gentoo",
        "Funtoo",
        "Slackware",
        "UwUntu",
        "NixOS",
        "VanillaOS",
        "Kali Linux",
        "CachyOS",
        "NetBSD",
        "FreeBSD",
        "Unknown distro",
    ];

    for i in distros {
        println!("\n{i}: \n{}", get_distro_ascii(Some(i.to_string())));
    }
}

pub fn get_distro_ascii(overriden_ascii: Option<String>) -> String {
    let ascii_macros = vec![
        ascii!(overriden_ascii, "arch", "   ___               __ \n  / _ |  ____ ____  / / \n / __ | / __// __/ / _ \\\n/_/ |_|/_/   \\__/ /_//_/"),
        ascii!(overriden_ascii, "debian", "   ___      __   _         \n  / _ \\___ / /  (_)__ ____ \n / // / -_) _ \\/ / _ `/ _ \\\n/____/\\__/_.__/_/\\_,_/_//_/"),
        ascii!(overriden_ascii, "fedora", "   ____       __             \n  / __/__ ___/ /__  _______ _\n / _// -_) _  / _ \\/ __/ _ `/\n/_/  \\__/\\_,_/\\___/_/  \\_,_/"),
        ascii!(overriden_ascii, "endeavour", "   ____        __                           \n  / __/__  ___/ /__ ___ __  _____  __ ______\n / _// _ \\/ _  / -_) _ `/ |/ / _ \\/ // / __/\n/___/_//_/\\_,_/\\__/\\_,_/|___/\\___/\\_,_/_/"),
        ascii!(overriden_ascii, "void", "  _   __     _    __\n | | / /__  (_)__/ /\n | |/ / _ \\/ / _  / \n |___/\\___/_/\\_,_/"),
        ascii!(overriden_ascii, "ubuntu", "  __  ____             __      \n / / / / /  __ _____  / /___ __\n/ /_/ / _ \\/ // / _ \\/ __/ // /\n\\____/_.__/\\_,_/_//_/\\__/\\_,_/"),
        ascii!(overriden_ascii, "suse", "  ____                ____            \n / __ \\___  ___ ___  / __/_ _____ ___ \n/ /_/ / _ \\/ -_) _ \\_\\ \\/ // (_-</ -_)\n\\____/ .__/\\__/_//_/___/\\_,_/___/\\__/ \n    /_/"),
        ascii!(overriden_ascii, "rasp", "   ___                __   _         \n  / _ \\___ ____ ___  / /  (_)__ ____ \n / , _/ _ `(_-</ _ \\/ _ \\/ / _ `/ _ \\\n/_/|_|\\_,_/___/ .__/_.__/_/\\_,_/_//_/\n             /_/"),
        ascii!(overriden_ascii, "mint", "   __  ____      __ \n  /  |/  (_)__  / /_\n / /|_/ / / _ \\/ __/\n/_/  /_/_/_//_/\\__/"),
        ascii!(overriden_ascii, "mx", "   __  ____  __  __   _               \n  /  |/  / |/_/ / /  (_)__  __ ____ __\n / /|_/ />  <  / /__/ / _ \\/ // /\\ \\ /\n/_/  /_/_/|_| /____/_/_//_/\\_,_//_\\_\\"),
        ascii!(overriden_ascii, "gentoo", "  _____         __          \n / ___/__ ___  / /____  ___ \n/ (_ / -_) _ \\/ __/ _ \\/ _ \\\n\\___/\\__/_//_/\\__/\\___/\\___/"),
        ascii!(overriden_ascii, "funtoo", "   ____          __          \n  / __/_ _____  / /____  ___ \n / _// // / _ \\/ __/ _ \\/ _ \n/_/  \\_,_/_//_/\\__/\\___/\\___/"),
        ascii!(overriden_ascii, "slack", "   ______         __                      \n  / __/ /__ _____/ /___    _____ ________ \n _\\ \\/ / _ `/ __/  '_/ |/|/ / _ `/ __/ -_)\n/___/_/\\_,_/\\__/_/\\_\\|__,__/\\_,_/_/  \\__/ "),
        ascii!(overriden_ascii, "uwuntu", "  __  __       __  __     __      \n / / / /    __/ / / /__  / /___ __\n/ /_/ / |/|/ / /_/ / _ \\/ __/ // /\n\\____/|__,__/\\____/_//_/\\__/\\_,_/"),
        ascii!(overriden_ascii, "nix", "   _  ___      ____  ____\n  / |/ (_)_ __/ __ \\/ __/\n /    / /\\ \\ / /_/ /\\ \\  \n/_/|_/_//_\\_\\\\____/___/"),
        ascii!(overriden_ascii, "vanilla", "  _   __          _ ____    \n | | / /__ ____  (_) / /__ _\n | |/ / _ `/ _ \\/ / / / _ `/\n |___/\\_,_/_//_/_/_/_/\\_,_/"),
        ascii!(overriden_ascii, "kali", "   __ __     ___   \n  / //_/__ _/ (_)  \n / ,< / _ `/ / /   \n/_/|_|\\_,_/_/_/"),
        ascii!(overriden_ascii, "cachy", "  _____         __       \n / ___/__ _____/ /  __ __\n/ /__/ _ `/ __/ _ \\/ // /\n\\___/\\_,_/\\__/_//_/\\_, / \n                  /___/"),
        ascii!(overriden_ascii, "netbsd", "   _  __    __  ___  _______ \n  / |/ /__ / /_/ _ )/ __/ _ \\\n /    / -_) __/ _  |\\ \\/ // /\n/_/|_/\\__/\\__/____/___/____/ "),
        ascii!(overriden_ascii, "freebsd", "   ___            ___  _______ \n  / _/______ ___ / _ )/ __/ _ \\\n / _/ __/ -_) -_) _  |\\ \\/ // /\n/_//_/  \\__/\\__/____/___/____/")
    ];

    for i in ascii_macros {
        if !i.is_empty() {
            return i;
        }
    }

    "   ___      _____      __ \n  / _ \\___ / _/ /_____/ / \n / , _(_-</ _/ __/ __/ _ \\\n/_/|_/___/_/ \\__/\\__/_//_/".to_string()
}
