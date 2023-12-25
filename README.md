# abuse_ip_db_to_iptables
Rust program aiming to ban automatically all IP addresses from abuseIPDB to IPtables.

I consider this program done but feel free to push new options or functionalities.

## INSTALLATION

The following steps are for a Debian installation. Feel free to adapt it to your system.

### Requirements

You (might) need to install some dependencies.

```shell
# Installing the rust language (you can remove it later in this tutorial)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Follow the steps of installation, this should be rather simple.

# Then updating your path may be a good thing.
source "$HOME/.cargo/env"

# Other dependencies
sudo apt install git iptables build_essentials pkg-config libssl-dev
```

You also need one token access from [AbuseIPDB](https://www.abuseipdb.com/). It is free, you just have to create your
account.

### Compiling the application

```shell
# Place yourself in the directory of your choice and get the code.
git clone https://github.com/racine-p-a/abuse_ip_db_to_iptables.git
# Move to this new directory
mv abuse_ip_db_to_iptables
# Compile the code (this can last one minute)
cargo build --release
# The binary is now available in the sub-folder target/release
# Move it where you think it suits the best for you.
```

## USAGE

You just need the binary that you compiled and your token access to AbuseIPDB.
```shell
# In this example, I did not move the binary after compilation.
sudo ./target/release/abuse_ip_db_to_iptables YOUR_TOKEN_HERE
```

THis will download the last database of toxic IP from the AbuseIPDB website and then put them on the ban list of
iptables.

IMPORTANT ! You have to know that you can request the database only ten times per day. You shouldn't use it
more than once daily or weekly. If for some usage, you need it more often, you must use a paid account on
AbuseIPDB but this scenario is unlikely.

## CLEAN-UP

```shell
# Uninstall the dependencies
# git iptables build_essentials pkg-config libssl-dev
sudo apt remove YOUR_DEPENDENCIES_YOU_WANT_TO_REMOVE

# Uninstall Rust
rustup self uninstall

# Remove the source code folder. IF YOU DO THIS, REMEMBER TO MOVE THE APPLICATION SOMEWHERE ELSE BEFORE... 
sudo rm -r abuse_ip_db_to_iptables
```

## OTHER

### Reset your IPtables ban list

If you want to delete all entries in the ban list, you can purge it this way :
```shell
sudo iptables -F
```

If you want to check the banned IP, use the following command.
```shell
sudo iptables -L INPUT -v -n
```