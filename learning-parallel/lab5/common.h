#ifndef COMMON_H
#define COMMON_H

//#include <stddef.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>


typedef struct request
{
    size_t len;
    size_t id;
    char * return_addr;
} request;

typedef struct response
{
    size_t len;
    char * payload;
} response;

size_t read_msg_len(int q)
{
    size_t len = 0;
    int _ = read(q, &len, sizeof(size_t)); // err_check?
    return len;
}

#endif /* COMMON_H */
