all:
	$(MAKE) client
	$(MAKE) server
client: client.c
	gcc -g client.c common.h -o client
server: server.c
	gcc -g server.c common.h -o server
serwerfifo:
	mkfifo serwerfifo
klientfifo:
	mkfifo klientfifo
