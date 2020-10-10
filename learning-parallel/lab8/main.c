#include <stdio.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <netinet/in.h>
#include <signal.h>
#include <strings.h>
#include <stdlib.h>


const int TARGET_IP = 168430310; // 10.10.10.230
const int TARGET_PORT = 5000;
const int L3_PROTO = AF_INET;
const int L4_PROTO = SOCK_DGRAM;
const int DEFAULT_SOCKET_PROTO = 0;


int main(int argc, char *argv[])
{
    int input = atoi(argv[1]);
    
    int socket_fd = socket(L3_PROTO, L4_PROTO, DEFAULT_SOCKET_PROTO);

    // TODO: change local socket name to something better
    struct sockaddr_in local, dest;
    bzero((char *) &local, sizeof(local));
    bzero((char *) &dest, sizeof(dest));

    local.sin_family=L3_PROTO;
    local.sin_addr.s_addr=htonl(INADDR_ANY);
    local.sin_port=htons((ushort) 5000);

    dest.sin_family=L3_PROTO;
    dest.sin_addr.s_addr=htonl(TARGET_IP);
    dest.sin_port=htons((ushort) TARGET_PORT);

    bind(socket_fd, (struct sockaddr *) &local, sizeof(local));

    input = htonl(input);
    sendto(socket_fd, (char *) &input, sizeof(int), 0, (struct sockaddr *) &dest, sizeof(dest));

    int output;
    unsigned int ble = sizeof(local);
    recvfrom(socket_fd, (char *) &output, sizeof(int), 0, (struct sockaddr *) &local, &ble);
    output=ntohl(output);

    printf("%d\n", output);
}
