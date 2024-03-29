use crate::mods::r#fn::{get_os_release_pretty_name, uname_s};

pub fn get_distro_ascii() -> String {
    if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("arch")
    {
        return "   ___               __ \n  / _ |  ____ ____  / / \n / __ | / __// __/ / _ \\\n/_/ |_|/_/   \\__/ /_//_/".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("debian")
    {
        return "   ___      __   _         \n  / _ \\___ / /  (_)__ ____ \n / // / -_) _ \\/ / _ `/ _ \\\n/____/\\__/_.__/_/\\_,_/_//_/".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("fedora")
    {
        return "   ____       __             \n  / __/__ ___/ /__  _______ _\n / _// -_) _  / _ \\/ __/ _ `/\n/_/  \\__/\\_,_/\\___/_/  \\_,_/".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("endeavouros")
    {
        return "   ____        __                           \n  / __/__  ___/ /__ ___ __  _____  __ ______\n / _// _ \\/ _  / -_) _ `/ |/ / _ \\/ // / __/\n/___/_//_/\\_,_/\\__/\\_,_/|___/\\___/\\_,_/_/   ".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("void")
    {
        return "   _   __     _    __\n | | / /__  (_)__/ /\n | |/ / _ \\/ / _  / \n |___/\\___/_/\\_,_/".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("ubuntu")
    {
        return "  __  ____             __      \n / / / / /  __ _____  / /___ __\n/ /_/ / _ \\/ // / _ \\/ __/ // /\n\\____/_.__/\\_,_/_//_/\\__/\\_,_/".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("suse")
    {
        return "  ____                ____            \n / __ \\___  ___ ___  / __/_ _____ ___ \n/ /_/ / _ \\/ -_) _ \\_\\ \\/ // (_-</ -_)\n\\____/ .__/\\__/_//_/___/\\_,_/___/\\__/ \n    /_/                            ".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("raspbian")
    {
        return "   ___                __   _         \n  / _ \\___ ____ ___  / /  (_)__ ____ \n / , _/ _ `(_-</ _ \\/ _ \\/ / _ `/ _ \\\n/_/|_|\\_,_/___/ .__/_.__/_/\\_,_/_//_/\n             /_/             ".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("mint")
    {
        return "   __  ____      __ \n  /  |/  (_)__  / /_\n / /|_/ / / _ \\/ __/\n/_/  /_/_/_//_/\\__/".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("mx")
    {
        return "   __  ____  __  __   _               \n  /  |/  / |/_/ / /  (_)__  __ ____ __\n / /|_/ />  <  / /__/ / _ \\/ // /\\ \\ /\n/_/  /_/_/|_| /____/_/_//_/\\_,_//_\\_\\ ".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("gentoo")
    {
        return "  _____         __          \n / ___/__ ___  / /____  ___ \n/ (_ / -_) _ \\/ __/ _ \\/ _ \\\n\\___/\\__/_//_/\\__/\\___/\\___/".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("slackware")
    {
        return "   ______         __                      \n  / __/ /__ _____/ /___    _____ ________ \n _\\ \\/ / _ `/ __/  '_/ |/|/ / _ `/ __/ -_)\n/___/_/\\_,_/\\__/_/\\_\\|__,__/\\_,_/_/  \\__/ ".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("uwuntu")
    {
        return "  __  __       __  __     __      \n / / / /    __/ / / /__  / /___ __\n/ /_/ / |/|/ / /_/ / _ \\/ __/ // /\n\\____/|__,__/\\____/_//_/\\__/\\_,_/".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("nix")
    {
        return "   _  ___      ____  ____\n  / |/ (_)_ __/ __ \\/ __/\n /    / /\\ \\ / /_/ /\\ \\  \n/_/|_/_//_\\_\\\\____/___/".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("vanilla")
    {
        return "  _   __          _ ____    \n | | / /__ ____  (_) / /__ _\n | |/ / _ `/ _ \\n/ / / / _ `/\n |___/\\_,_/_//_/_/_/_/\\_,_/".to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("kali")
    {
        return "   __ __     ___   \n  / //_/__ _/ (_)  \n / ,< / _ `/ / /   \n/_/|_|\\_,_/_/_/"
            .to_string();
    } else if get_os_release_pretty_name('i')
        .unwrap_or("".to_string())
        .to_ascii_lowercase()
        .contains("cachy")
    {
        return "  _____         __       \n / ___/__ _____/ /  __ __\n/ /__/ _ `/ __/ _ \\/ // /\n\\___/\\_,_/\\__/_//_/\\_, / \n                  /___/".to_string();
    }
    if uname_s().to_ascii_lowercase().contains("netbsd") {
        return "   _  __    __  ___  _______ \n  / |/ /__ / /_/ _ )/ __/ _ \\\n /    / -_) __/ _  |\\ \\/ // /\n/_/|_/\\__/\\__/____/___/____/ ".to_string();
    } else if uname_s().to_ascii_lowercase().contains("freebsd") {
        return "   ___            ___  _______ \n  / _/______ ___ / _ )/ __/ _ \\\n / _/ __/ -_) -_) _  |\\ \\/ // /\n/_//_/  \\__/\\__/____/___/____/".to_string();
    } else {
        return "   ___           __    ____    __      __ \n  / _ \\__ _____ / /_  / __/__ / /_____/ / \n / , _/ // (_-</ __/ / _// -_) __/ __/ _ \\\n/_/|_|\\_,_/___/\\__/ /_/  \\__/\\__/\\__/_//_/".to_string();
    }
}
