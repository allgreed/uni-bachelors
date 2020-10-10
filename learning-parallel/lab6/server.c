#include "common.h"


const long ANY_MSG_TYPE = 0;

void cleanup_queues();
void handle_sig(int signo);
char * process_request(char * input);


int REQUEST_Q = 0;
int RESPONSE_Q = 0;


int main()
{
    int request_q_key = ftok("/allgreed/lab6", REQUEST_Q_KEY_PROJ);
    int response_q_key = ftok("/allgreed/lab6", RESPONSE_Q_KEY_PROJ);

    atexit(cleanup_queues);
    REQUEST_Q = msgget(request_q_key, 0666 | IPC_CREAT); 
    RESPONSE_Q = msgget(response_q_key, 0666 | IPC_CREAT); 

    struct sigaction act;
    act.sa_handler = &handle_sig;
    sigaction(SIGINT, &act, NULL);

    while(true)
    {
        msg msg_buffer;
        ssize_t err = msgrcv(REQUEST_Q, &msg_buffer, sizeof(msg_buffer) - sizeof(long int), ANY_MSG_TYPE, 0); // err?

        char * result = process_request(msg_buffer.mtext);
        strcpy(msg_buffer.mtext, result);

        msgsnd(RESPONSE_Q, &msg_buffer, sizeof(msg_buffer) - sizeof(long int), 0); // err?
    }
}

char * process_request(char * input)
{
    if (!strcmp(input, "cokolwiek"))
    {
        return "whatever";
    }
    else
    {
        return "Nie znam takiego słowa";
    }
}


void cleanup_queues()
{
    printf("Deleting queues...\n");

    if (REQUEST_Q)
    {
        msgctl(REQUEST_Q, IPC_RMID, NULL);
    }

    if (RESPONSE_Q)
    {
        msgctl(RESPONSE_Q, IPC_RMID, NULL);
    }
}

void handle_sig(int signo)
{
    exit(0);
}
