#ifndef MID_H
#define MID_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Get machine ID hash using the provided service name.
 * The returned string must be freed using mid_free_string.
 * Returns NULL on error.
 */
char* mid_get(const char* service_name);

/**
 * Free the string returned by mid_get.
 */
void mid_free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif /* MID_H */
