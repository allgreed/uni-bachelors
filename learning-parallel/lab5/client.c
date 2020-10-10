#include "common.h"


const char * INPUT_QUEUE_LOCATION = "klientfifo";
const char * OUTPUT_QUEUE_LOCATION = "serwerfifo";

void send_request(request r, int q);
response read_response(int q);


int main(int argc, char *argv[])
{
    const char * pwd = ""; // TODO: get pwd
    // TODO: concat pwd and queue - efficiently
    const char * return_addr = INPUT_QUEUE_LOCATION;
    size_t id = atoi(argv[1]);

    request req = { .id = id, .return_addr = return_addr, .len = sizeof(size_t) + strlen(return_addr)};

    int output_queue = open(OUTPUT_QUEUE_LOCATION, O_WRONLY); // err?
    send_request(req, output_queue);

    int input_queue = open(INPUT_QUEUE_LOCATION, O_RDONLY);
    response r = read_response(input_queue);

    printf("Server responded with: %s\n", r.payload);
}


void send_request(request r, int q)
{
    char * buffer = (char * ) malloc(sizeof(char) * r.len + sizeof(size_t)); // err?? free?
    buffer[0] = r.len;
    buffer[sizeof(size_t)] = r.id;
    strcpy(buffer + 2 * sizeof(size_t), r.return_addr);

    int _ = write(q, buffer, sizeof(char) * r.len + sizeof(size_t)); // err??
}

response read_response(int q)
{
    response r;
    size_t r_len = read_msg_len(q);
    r.len = r_len;
    r.payload = (char *) malloc(sizeof(char) * (r_len) + 1); // err?
    int _ = read(q, r.payload, sizeof(char) * r_len);  // err??
    r.payload[sizeof(char) * (r_len)] = 0;
    return r;
}
