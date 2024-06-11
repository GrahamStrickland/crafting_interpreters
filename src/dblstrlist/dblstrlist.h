struct snode;

struct snode *smalloc(void);
void sassign(struct snode *s, char *k);
char* get_key(struct snode *s);

struct dblstrlist;

struct dblstrlist *dblstrlistalloc(void);
struct snode *list_search(struct dblstrlist *l, char *k);
void list_insert(struct dblstrlist *l, struct snode *x);
void list_delete(struct dblstrlist *l, struct snode *x);
