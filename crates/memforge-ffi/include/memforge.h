/* MEMFORGE's C interface. Every function returns MF_OK, MF_NOT_FOUND, or a negative error
   code. The library never lets a panic cross into C. */
#ifndef MEMFORGE_H
#define MEMFORGE_H

#include <stddef.h>
#include <stdint.h>

/* An open database. C only ever holds a pointer to it: what is inside belongs to Rust. */
typedef struct MemforgeDb MemforgeDb;

enum {
    MF_OK = 0,
    MF_NOT_FOUND = 1,
    MF_NULL = -1,
    MF_BAD_LENGTH = -2,
    MF_TOO_LARGE = -3,
    MF_IO = -4,
    MF_CORRUPT = -5,
    MF_BUFFER_TOO_SMALL = -6,
    MF_PANIC = -7,
};

/* Opens the database in the directory dir; on success, *out is its handle. */
int mf_open(const char *dir, MemforgeDb **out);

/* Stores value under key. */
int mf_put(MemforgeDb *db, const uint8_t *key, size_t key_len, const uint8_t *value,
           size_t value_len);

/* Copies key's value into buf, and its length into *value_len, even if buf is too small. */
int mf_get(const MemforgeDb *db, const uint8_t *key, size_t key_len, uint8_t *buf,
           size_t buf_len, size_t *value_len);

/* Flushes and closes the database, and frees the handle: never use db again. */
int mf_close(MemforgeDb *db);

/* Panics inside MEMFORGE: guarded (non-zero) returns MF_PANIC; unguarded aborts. */
int mf_demo_panic(int guarded);

#endif
