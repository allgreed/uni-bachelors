#include <stdlib.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

// visit for more info:
// https://stackoverflow.com/a/17741176/9134286
// https://stackoverflow.com/a/35447608/9134286
// https://stackoverflow.com/a/14190081/9134286

int main()
{
    FILE* fp;
    fp = fopen("./test", "w+");

    pid_t pid = getpid();
    fprintf(fp, "Parent pid: %d\n", pid);

    if (fork() == 0) // child
    {
        pid_t mypid = getpid();
        fprintf(fp, "Child pid: %d\n", mypid);
        fclose(fp);
        return 0;
    }
    else
    {
        wait(NULL);
    }
    
    fprintf(fp, "Coś ciekawego\n");
    fclose(fp);
    fprintf(fp, "Testuję\n"); // to się nie wpisze
    return 0;
}
