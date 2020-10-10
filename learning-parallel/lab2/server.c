#include <unistd.h>
#include <fcntl.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>


const int STDIN_FD = 0;
const int STDOUT_FD = 1;
const int ESC_ASCII = 27;
const char* ESC_NEEDLE = "";
const char* INPUT_FILENAME = "dane";
const char* OUTPUT_FILENAME = "wyniki";
const char* LOCKFILE_FILENAME = "lockfile";

int main()
{
    long PAGE_SIZE = getpagesize(); // err_check? - PAGE_SIZE could be 1 theoretically
    size_t buffer_size = (PAGE_SIZE - 1) * sizeof(char);
    char * buffer = (char*) malloc(PAGE_SIZE); // err_check?, skipping dealocation - not worth it

    while(true)
    {
        int ifd;
        while((ifd = open(INPUT_FILENAME, O_RDONLY)) == -1);
        while (true)
        {
            int rbytes = read(ifd, buffer, buffer_size);
            if (rbytes <= 0)
            {
                close(ifd); // err_check?
                ifd = open(INPUT_FILENAME, O_RDONLY); // err_check?
                continue;
            }

            char * esc_char_in_buffer = strstr(buffer, ESC_NEEDLE);

            if (esc_char_in_buffer)
            {
                *esc_char_in_buffer = '\n';
                *(esc_char_in_buffer + 1) = '\0';
            }

            int _ = write(STDOUT_FD, buffer, rbytes); // err_check?

            if (esc_char_in_buffer)
            {
                break;
            }

        }
        close(ifd); // err_check?
        ifd = open(INPUT_FILENAME, O_WRONLY | O_TRUNC); // err_check?, TODO: can I do this with fewer syscalls?
        close(ifd); // err_check?

        int ofd = open(OUTPUT_FILENAME, O_CREAT | O_TRUNC | O_WRONLY, S_IROTH | S_IWUSR | S_IRUSR); // err_check?
        char * offsetted_buffer = buffer;
        while(true)
        {
            int rbytes = read(STDIN_FD, offsetted_buffer, buffer_size - (offsetted_buffer - buffer)); // err_check?
            offsetted_buffer += rbytes;
            char * esc_char_in_buffer = strstr(buffer, ESC_NEEDLE);

            if (esc_char_in_buffer)
            {
                int _ = write(ofd, buffer, offsetted_buffer - buffer); // err_check?
                break;
            }
        }
        close(ofd); // err_check?

        unlink(LOCKFILE_FILENAME); // err_check?
    }
}
