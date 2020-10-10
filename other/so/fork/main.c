#include <stdlib.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>
#include <stdio.h>
#include <signal.h>

void noop_action(int sig) {}

// visit for more info:
// https://www.youtube.com/watch?v=3akTtCu_F_k
// https://stackoverflow.com/a/4597931/9134286
// https://en.wikipedia.org/wiki/Fork_(system_call)#Overview
// https://en.wikipedia.org/wiki/Virtual_address_space

int main()
{
    static struct sigaction act;

    act.sa_handler = noop_action;
    sigaction(SIGUSR1, &act, NULL);

    pid_t pid = getpid();

    int* test = malloc(sizeof(int));
    *test = -8;
    printf("-2 [Set] pid = %d, *test = %d\n", pid, *test);
    printf("-1       test = %p\n", test);

    pid_t cpid;

    if (cpid = fork())
    {
        // 0
        printf(" 0       parent, I'm in space!\n");
        pause(); 

        // 2
        *test = 6;
        printf(" 2 [Set] parent, *test = %d\n", *test);
        kill(cpid, SIGUSR1);
        pause();

        // 5
        printf(" 5       parent, *test = %d\n", *test);
        kill(cpid, SIGUSR1);
    }
    else // child
    {
        // 0, 1
        pid_t ppid = getppid();
        printf(" 1       child , *test = %d, test = %p\n", *test, test);
        kill(ppid, SIGUSR1);
        pause();

        // 3
        printf(" 3       child , *test = %d, test = %p\n", *test, test);

        // 4
        *test = 42;
        printf(" 4 [Set] child , *test = %d, test = %p\n", *test, test);
        kill(ppid, SIGUSR1);
        pause();
        
        // 6
        printf(" 6       child , *test = %d\n", *test);
    }
    
    return 0;
}
