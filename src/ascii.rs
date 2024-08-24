use crate::info::os_pretty_name;
use std::collections::HashMap;

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

pub fn get_distro_ascii(ascii_override: Option<String>) -> String {
    let distros = HashMap::from([
        ("arch", "   ___               __ \n  / _ |  ____ ____  / / \n / __ | / __// __/ / _ \\\n/_/ |_|/_/   \\__/ /_//_/"),
        ("debian", "   ___      __   _         \n  / _ \\___ / /  (_)__ ____ \n / // / -_) _ \\/ / _ `/ _ \\\n/____/\\__/_.__/_/\\_,_/_//_/"),
        ("fedora", "   ____       __             \n  / __/__ ___/ /__  _______ _\n / _// -_) _  / _ \\/ __/ _ `/\n/_/  \\__/\\_,_/\\___/_/  \\_,_/"),
        ("endeavour", "   ____        __                           \n  / __/__  ___/ /__ ___ __  _____  __ ______\n / _// _ \\/ _  / -_) _ `/ |/ / _ \\/ // / __/\n/___/_//_/\\_,_/\\__/\\_,_/|___/\\___/\\_,_/_/"),
        ("void", "  _   __     _    __\n | | / /__  (_)__/ /\n | |/ / _ \\/ / _  / \n |___/\\___/_/\\_,_/"),
        ("ubuntu", "  __  ____             __      \n / / / / /  __ _____  / /___ __\n/ /_/ / _ \\/ // / _ \\/ __/ // /\n\\____/_.__/\\_,_/_//_/\\__/\\_,_/"),
        ("suse", "  ____                ____            \n / __ \\___  ___ ___  / __/_ _____ ___ \n/ /_/ / _ \\/ -_) _ \\_\\ \\/ // (_-</ -_)\n\\____/ .__/\\__/_//_/___/\\_,_/___/\\__/ \n    /_/"),
        ("rasp", "   ___                __   _         \n  / _ \\___ ____ ___  / /  (_)__ ____ \n / , _/ _ `(_-</ _ \\/ _ \\/ / _ `/ _ \\\n/_/|_|\\_,_/___/ .__/_.__/_/\\_,_/_//_/\n             /_/"),
        ("mint", "   __  ____      __ \n  /  |/  (_)__  / /_\n / /|_/ / / _ \\/ __/\n/_/  /_/_/_//_/\\__/"),
        ("mx", "   __  ____  __  __   _               \n  /  |/  / |/_/ / /  (_)__  __ ____ __\n / /|_/ />  <  / /__/ / _ \\/ // /\\ \\ /\n/_/  /_/_/|_| /____/_/_//_/\\_,_//_\\_\\"),
        ("gentoo", "  _____         __          \n / ___/__ ___  / /____  ___ \n/ (_ / -_) _ \\/ __/ _ \\/ _ \\\n\\___/\\__/_//_/\\__/\\___/\\___/"),
        ("funtoo", "   ____          __          \n  / __/_ _____  / /____  ___ \n / _// // / _ \\/ __/ _ \\/ _ \n/_/  \\_,_/_//_/\\__/\\___/\\___/"),
        ("slack", "   ______         __                      \n  / __/ /__ _____/ /___    _____ ________ \n _\\ \\/ / _ `/ __/  '_/ |/|/ / _ `/ __/ -_)\n/___/_/\\_,_/\\__/_/\\_\\|__,__/\\_,_/_/  \\__/ "),
        ("uwuntu", "  __  __       __  __     __      \n / / / /    __/ / / /__  / /___ __\n/ /_/ / |/|/ / /_/ / _ \\/ __/ // /\n\\____/|__,__/\\____/_//_/\\__/\\_,_/"),
        ("nix", "   _  ___      ____  ____\n  / |/ (_)_ __/ __ \\/ __/\n /    / /\\ \\ / /_/ /\\ \\  \n/_/|_/_//_\\_\\\\____/___/"),
        ("vanilla", "  _   __          _ ____    \n | | / /__ ____  (_) / /__ _\n | |/ / _ `/ _ \\/ / / / _ `/\n |___/\\_,_/_//_/_/_/_/\\_,_/"),
        ("kali", "   __ __     ___   \n  / //_/__ _/ (_)  \n / ,< / _ `/ / /   \n/_/|_|\\_,_/_/_/"),
        ("cachy", "  _____         __       \n / ___/__ _____/ /  __ __\n/ /__/ _ `/ __/ _ \\/ // /\n\\___/\\_,_/\\__/_//_/\\_, / \n                  /___/"),
        ("netbsd", "   _  __    __  ___  _______ \n  / |/ /__ / /_/ _ )/ __/ _ \\\n /    / -_) __/ _  |\\ \\/ // /\n/_/|_/\\__/\\__/____/___/____/ "),
        ("freebsd", "   ___            ___  _______ \n  / _/______ ___ / _ )/ __/ _ \\\n / _/ __/ -_) -_) _  |\\ \\/ // /\n/_//_/  \\__/\\__/____/___/____/")
    ]);

    let search_term = ascii_override
        .clone()
        .map(|o| o.to_ascii_lowercase())
        .unwrap_or_else(|| {
            os_pretty_name(None, "ID")
                .unwrap_or_default()
                .to_ascii_lowercase()
        });

    let ascii_result = distros
        .iter()
        .find_map(|(&key, &value)| {
            if search_term.contains(key) {
                Some(value.to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| {
            "   ___      _____      __ \n  / _ \\___ / _/ /_____/ / \n / , _(_-</ _/ __/ __/ _ \\\n/_/|_/___/_/ \\__/\\__/_//_/".to_string()
        });

    ascii_result
}
