#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <sys/ipc.h>
#include <sys/sem.h>
#include <sys/shm.h>
#include <errno.h>


#define BOARD_EDGE_LEN 3
#define BOARD_FIELD_COUNT (BOARD_EDGE_LEN * BOARD_EDGE_LEN)
const unsigned int ARBITRARY_INVALID_MOVE = -1;

typedef enum FieldState {
    Empty = ' ', 
    Player0 = 'X',
    Player1 = 'O' 
} FieldState;

typedef struct Board {
    char fields[BOARD_FIELD_COUNT];
    char last_move_owner;
} Board;

typedef struct MatchStatus {
    bool is_fin;
    FieldState winner;
} MatchStatus;

typedef struct sembuf Sembuf;


void print_board(Board * b);
void print_help();
unsigned int acquire_valid_move(Board * b);
bool is_valid_move(unsigned int move, Board * b);
MatchStatus check_state(Board * b);
void clean_sem(int _, void * semid);
void clean_shm(int _, void * shmid);

Sembuf _SEM_AWAIT = { .sem_num = 0, .sem_op = -1, .sem_flg = 0};
Sembuf _SEM_RELEASE = { .sem_num = 0, .sem_op = 1, .sem_flg = 0};
Sembuf * SEM_AWAIT = &_SEM_AWAIT, * SEM_RELEASE = &_SEM_RELEASE;


int main(int argc, char *argv[])
{
    key_t key = ftok("/allgreed/lab7", 0);

    int shared_mem_segment = shmget(key, sizeof(Board), 0666 | IPC_CREAT);
    int maybe_semaphore = semget(key, 1, 0666 | IPC_CREAT | IPC_EXCL);

    int semaphore;
    char player;

    if (maybe_semaphore == -1 && errno == EEXIST)
    {
        semaphore = semget(key, 1, 0666);
        player = Player1;
    }
    else
    {
        semaphore = maybe_semaphore;
        player = Player0;
    }
    
    Board * board = (Board *) shmat(shared_mem_segment, NULL, 0);
    MatchStatus status;

    print_help();
    printf("\nWitaj %c, niechaj rozpocznie się gra!\n\n", player);

    if (player == Player0)
    {
        for (int i = 0; i < BOARD_FIELD_COUNT; ++i) board->fields[i] = Empty;
        // TODO: heh
        goto BLE;  
    }

    while(true)
    {
        printf("\nTwój przeciwnik wybiera...\n");
        semop(semaphore, SEM_AWAIT, 1);

        BLE:
        print_board(board);

        status = check_state(board);
        if (status.is_fin)
        {
            semop(semaphore, SEM_RELEASE, 1);
            break;
        }
        
        int move = acquire_valid_move(board);
        board->fields[move] = player;
        board->last_move_owner = player;

        print_board(board);

        semop(semaphore, SEM_RELEASE, 1);
    }

    printf("\nFinito! Wynik rozgrywki: %s\n",
    status.winner == Empty
        ? "remis"
        : status.winner == player
            ? "wygrana!"
            : "przegrana :C"
    );

    semop(semaphore, SEM_AWAIT, 1);

    shmctl(shared_mem_segment, IPC_RMID, NULL);
    semctl(semaphore, 0, IPC_RMID, 0);
}


bool is_valid_move(unsigned int move, Board * b)
{ 
    return move <= (BOARD_FIELD_COUNT - 1) && b->fields[move] == Empty;
}

MatchStatus check_state(Board * b)
{
    MatchStatus outcome;

    // TODO: simplify this
    // rows
    for (int i = 0; i < BOARD_EDGE_LEN; ++i)
    {
        int mult = BOARD_EDGE_LEN * i;
        int acc = 0;

        for (int j = 1; j < BOARD_EDGE_LEN; ++j)
        {
            acc += b->fields[j - 1 + mult] == b->fields[j + mult];
        }

        if (acc == BOARD_EDGE_LEN - 1 && b->fields[0 + mult] != Empty)
        {
            return (MatchStatus){ .is_fin = true, .winner = b->fields[0 + mult] };
        }
    }

    // TODO: simplify this
    // columns
    for (int i = 0; i < BOARD_EDGE_LEN; ++i)
    {
        int acc = 0;

        for (int j = 1; j < BOARD_EDGE_LEN; ++j)
        {
            acc += b->fields[i + (BOARD_EDGE_LEN * (j - 1))] == b->fields[i + (BOARD_EDGE_LEN * j)];
        }

        if (acc == BOARD_EDGE_LEN - 1 && b->fields[i] != Empty)
        {
            return (MatchStatus){ .is_fin = true, .winner = b->fields[i] };
        }
    }

    // TODO: simplify this
    // diagonals
    {
        int i = 0;
        int acc = 0;

        for (int j = 1; j < BOARD_EDGE_LEN; ++j)
        {
            acc += b->fields[i + ((BOARD_EDGE_LEN + 1) * (j - 1))] == b->fields[i + ((BOARD_EDGE_LEN + 1) * j)];
        }

        if (acc == BOARD_EDGE_LEN - 1 && b->fields[i] != Empty)
        {
            return (MatchStatus){ .is_fin = true, .winner = b->fields[i] };
        }
    }
    {
        int i = 2;
        int acc = 0;

        for (int j = 1; j < BOARD_EDGE_LEN; ++j)
        {
            acc += b->fields[i + ((BOARD_EDGE_LEN - 1) * (j - 1))] == b->fields[i + ((BOARD_EDGE_LEN - 1) * j)];
        }

        if (acc == BOARD_EDGE_LEN - 1 && b->fields[i] != Empty)
        {
            return (MatchStatus){ .is_fin = true, .winner = b->fields[i] };
        }
    }

    // TODO: simplify this
    // full board
    outcome = (MatchStatus){ .is_fin = true, .winner = Empty };
    for(int i = 0; i < BOARD_FIELD_COUNT; ++i)
    {
        if (b->fields[i] == Empty)
        {
            outcome.is_fin = false;
        }
    }

    return outcome;
}


unsigned int acquire_valid_move(Board * b)
{
    printf("\n");

    while(true)
    {
        printf("Co wybierasz? ");

        unsigned int move_candidate = ARBITRARY_INVALID_MOVE;
        int _ = scanf("%u", &move_candidate); // err ?

        if (is_valid_move(move_candidate, b))
        {
            return move_candidate;
        }

        printf("Nie! Tam nie!\n");
    }
}

// tried to do this with macros but on the second thought... I don't want to xD
#if BOARD_EDGE_LEN != 3
    #error "Setting BOARD_EDGE_LEN to not 3 is not supported"
#endif

const char * BOARD_TEMPLATE = 
    "|%c|%c|%c|\n"
    "|%c|%c|%c|\n"
    "|%c|%c|%c|\n"
;

void print_board(Board * b)
{
    printf(BOARD_TEMPLATE,
    b->fields[0],
    b->fields[1],
    b->fields[2],
    b->fields[3],
    b->fields[4],
    b->fields[5],
    b->fields[6],
    b->fields[7],
    b->fields[8]
    );
}

void print_help()
{
    Board board;
    for (int i = 0; i < BOARD_FIELD_COUNT; ++i)
        board.fields[i] = i + '0';

    print_board(&board);
}
