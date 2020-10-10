#include "common.h"


int main(int argc, char *argv[])
{
    int request_q_key = ftok("/allgreed/lab6", REQUEST_Q_KEY_PROJ);
    int response_q_key = ftok("/allgreed/lab6", RESPONSE_Q_KEY_PROJ);

    int request_q = msgget(request_q_key, 0666); 
    int response_q = msgget(response_q_key, 0666); 

    long int mtype = getpid();

    msg msg_buffer = {
        .mtype = mtype,
    };
    strcpy(msg_buffer.mtext, argv[1]);

    msgsnd(request_q, &msg_buffer, sizeof(msg_buffer) - sizeof(long), 0); // err?

    ssize_t err = msgrcv(response_q, &msg_buffer, sizeof(msg_buffer) - sizeof(long int), mtype, 0); // err?

    printf("%s\n", msg_buffer.mtext);
}
