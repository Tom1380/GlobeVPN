#!/bin/sh

# This script has to run as root.

apt update -y
apt install openvpn -y

# This file has to be copied on the host.
mv server.conf /etc/openvpn/server/server.conf

# Start OpenVPN on boot.
systemctl enable openvpn-server@server

# Allow IPv4 forwarding.
printf "\nnet.ipv4.ip_forward=1" >> /etc/sysctl.conf
sysctl -p

# Use iptables to forward OpenVPN traffic.
# Have the command run on boot as iptables isn't persistent.
# Reference https://askubuntu.com/a/290107
# I'm using single quotes to prevent bash from interpreting the exclamation point as an event.
echo '#!/bin/bash' > /etc/init.d/iptables_forwarding 
echo "iptables -t nat -A POSTROUTING -s 10.8.0.0/24 -o eth0 -j MASQUERADE" >> /etc/init.d/iptables_forwarding
chmod 755 /etc/init.d/iptables_forwarding
ln -s /etc/init.d/iptables_forwarding /etc/rc2.d/S01iptables_forwarding

reboot
