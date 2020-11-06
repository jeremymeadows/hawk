# hawk
The idea of this project is to be able to easily share files cross-platform without an internet connection or a LAN.
Ideally, the application sets up the ad-hoc network automagically, but it may need to be something done by the user beforehand.

My plan is to have any of the connected users be able to drag a file into the program running in the terminal, and Rust reads the file and sends it over TCP to all other peers simultaneously(ish) and saves it to the directory that hawk is running in.

I don't know Rust, and I don't know how network protocols work, so this will be fun.
