#! bin/bash

gcc -c -o $1.o $2
gcc -shared -o $3 $1.o