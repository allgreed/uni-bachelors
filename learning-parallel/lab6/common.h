#ifndef COMMON_H
#define COMMON_H

#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <signal.h>
#include <sys/ipc.h>
#include <sys/msg.h>

const int REQUEST_Q_KEY_PROJ = 0;
const int RESPONSE_Q_KEY_PROJ = 1;

typedef struct msg
{
	long int mtype;
	char mtext[255];
} msg;

#endif /* COMMON_H */
