#!/bin/sh

# Reference https://hub.docker.com/r/kylemanna/openvpn/
# Updated to be automated.

# Of course, UDP on port 1194 has to be allowed for this to work.

# Fetch the public IP address.
MY_IP=$(dig +short myip.opendns.com @resolver1.opendns.com)

OVPN_DATA="ovpn-data-example"

docker volume create --name $OVPN_DATA

docker run -v $OVPN_DATA:/etc/openvpn --rm kylemanna/openvpn ovpn_genconfig -u udp://"$MY_IP"
docker run -v $OVPN_DATA:/etc/openvpn --rm -e EASYRSA_BATCH=1 -e EASYRSA_REQ_CN="GlobeVPN" -e EASYRSA_PASSIN=pass:1111 -e EASYRSA_PASSOUT=pass:1111 -it kylemanna/openvpn ovpn_initpki
docker run -v $OVPN_DATA:/etc/openvpn -d -p 1194:1194/udp --cap-add=NET_ADMIN kylemanna/openvpn
docker run -v $OVPN_DATA:/etc/openvpn --rm -e EASYRSA_BATCH=1 -e EASYRSA_REQ_CN="GlobeVPN" -e EASYRSA_PASSIN=pass:1111 -e EASYRSA_PASSOUT=pass:1111 -it kylemanna/openvpn easyrsa build-client-full GlobeVPN nopass
docker run -v $OVPN_DATA:/etc/openvpn --rm kylemanna/openvpn ovpn_getclient GlobeVPN > GlobeVPN.ovpn