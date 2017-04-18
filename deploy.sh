printf "deploying to 'raspberrypi.local'... \n"
cd robot/target/arm-unknown-linux-gnueabihf/release/
ping -n 2 raspberrypi.local
if [ "$?" -eq 0 ]; then
    scp robot pi@raspberrypi.local:~/robot && ssh pi@raspberrypi.local
fi
