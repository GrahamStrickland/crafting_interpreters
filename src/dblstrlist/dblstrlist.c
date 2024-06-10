#include <stdlib.h>
#include <string.h>

#include "dblstrlist.h"

struct snode {
    char *key;
    struct snode *prev;
    struct snode *next;
};

/* smalloc: make an snode */
struct snode *smalloc(void) {
    return (struct snode *) malloc(sizeof(struct snode));
}

/* sassign: assign value of snode->key to k */
void sassign(struct snode *s, char *k) {
    s->key = k;
    s->prev = s->next = NULL;
}

struct dblstrlist {
    struct snode *head;
};

/* dblstrlistalloc: make a Doubly-Linked String List */
struct dblstrlist *dblstrlistalloc(void) {
    return (struct dblstrlist *) malloc(sizeof(struct dblstrlist));
}

/* List-Search(L, k) finds the first element with key k in list L,
    returning a pointer to this element */
struct snode *list_search(struct dblstrlist *l, char *k) {
    struct snode *x = l->head;

    while (x != NULL && !strcmp(x->key, k))
        x = x->next;
    
    return x;
}

/* List-Insert(L, x) splices x onto the front of the linked list L,
    given element x whose key attribute has already been set */
void list_insert(struct dblstrlist *l, struct snode *x) {
    x->next = l->head;
    if (l->head != NULL)
        l->head->prev = x;
    l->head = x;
    x->prev = NULL;
}
