typedef struct {
    char *key;
    struct snode *prev;
    struct snode *next;
} snode;

typedef struct {
    struct snode *head;
} dblstrlist;

struct snode *list_search(dblstrlist *l, char *k);

