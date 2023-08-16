CC=g++

LDFLAGS=-pthread -lpthread -std=c++17
LDFLAGS+=-mcpu=native
LDFLAGS+=-march=armv8.2-a+crc+fp16+rcpc+dotprod+crypto+ssbs+sha2
# cortex-a78aeはGCC9.4.9では指定できない
# LDFLAGS+=-mtune=cortex-a78ae

a.out:
	$(CC) -O3 ${LDFLAGS} main.cc

.PHONY: clean
clean:
	rm -f *.o
	rm -f a.out
