/* MEMFORGE from C: open, put, get and close, and the calls MEMFORGE refuses. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "memforge.h"

int main(int argc, char **argv) {
    if (argc > 1 && strcmp(argv[1], "unguarded") == 0) {
        return mf_demo_panic(0);
    }
    MemforgeDb *db = NULL;
    printf("open:  %d\n", mf_open("./data/c", &db));

    const char *key = "user:1001", *value = "Behrouz";
    const uint8_t *k = (const uint8_t *)key, *v = (const uint8_t *)value;
    printf("put:   %d\n", mf_put(db, k, strlen(key), v, strlen(value)));

    if (argc > 1 && strcmp(argv[1], "small-buffer") == 0) {
        /* A bug planted on purpose: a 4-byte buffer, passed as if it held 64. */
        uint8_t *small = malloc(4);
        size_t n = 0;
        printf("get into 4 heap bytes: %d\n", mf_get(db, k, strlen(key), small, 64, &n));
        free(small);
        return mf_close(db);
    }

    uint8_t buf[64];
    size_t len = 0;
    int rc = mf_get(db, k, strlen(key), buf, sizeof buf, &len);
    printf("get:   %d, %zu bytes: %.*s\n", rc, len, (int)len, (const char *)buf);

    printf("put with a NULL key:     %d\n", mf_put(db, NULL, 9, v, 7));
    printf("put with a bad length:   %d\n", mf_put(db, k, (size_t)-1, v, 7));
    printf("get into a 4-byte buffer: %d, needs %zu\n",
           mf_get(db, k, strlen(key), buf, 4, &len), len);
    printf("a guarded panic:         %d\n", mf_demo_panic(1));
    printf("close: %d\n", mf_close(db));
    return 0;
}
