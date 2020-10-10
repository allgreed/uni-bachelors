#include "common.h"


const char * INPUT_QUEUE_LOCATION = "serwerfifo";

request * receive_request();
void send_response(char* result, int q);
char* get_from_db(int id);
request read_request(int q);


int main()
{

    while(true)
    {
        int input_queue = open(INPUT_QUEUE_LOCATION, O_RDONLY);

        request req = read_request(input_queue);
        if (req.len == 0) continue;
        close(input_queue);

        char* result = get_from_db(req.id);

        int output_queue = open(req.return_addr, O_WRONLY); // err?
        send_response(result, output_queue);

        free(req.return_addr);
        close(output_queue); // err?
    }
}


char* get_from_db(int id)
{
    if (id == 1)
        return "Kowalski";
    else return (char *) "nie ma";
};

request read_request(int q)
{
    request r;
    size_t req_len = read_msg_len(q);
    r.len = req_len;
    r.return_addr = (char *) malloc(sizeof(char) * req_len - sizeof(size_t)); // err?
    int _ = read(q, &r.id, sizeof(size_t));  // err??
    int __ = read(q, r.return_addr, r.len - sizeof(size_t));  // err??
    return r;
}

void send_response(char* result, int q)
{
    size_t len = strlen(result);
    char * buffer = (char * ) malloc(sizeof(char) * len + sizeof(size_t)); // err??
    buffer[0] = len;
    strcpy(buffer + sizeof(size_t), result);
    int _ = write(q, buffer, sizeof(char) * len + sizeof(size_t)); // err??
    free(buffer);
}
