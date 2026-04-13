// =============================================================================
// mathlib.h — A small C library header for Rust FFI demonstration
// =============================================================================

#ifndef MATHLIB_H
#define MATHLIB_H

#include <stdint.h>  // fixed-width integers for ABI compatibility with Rust

// ---------------------------------------------------------------------------
// 1. Simple arithmetic (demonstrates basic scalar FFI)
// ---------------------------------------------------------------------------
int32_t add(int32_t a, int32_t b);
int32_t multiply(int32_t a, int32_t b);
double  divide_safe(double numerator, double denominator);

// ---------------------------------------------------------------------------
// 2. Working with strings (demonstrates C string <-> Rust &CStr / CString)
// ---------------------------------------------------------------------------
int32_t string_length(const char *s);
void    greet(const char *name, char *out_buf, int32_t buf_size);

// ---------------------------------------------------------------------------
// 3. Working with structs (demonstrates repr(C) struct interop)
// ---------------------------------------------------------------------------
typedef struct {
    double x;
    double y;
} Point2D;

double  point_distance(const Point2D *a, const Point2D *b);
Point2D point_midpoint(const Point2D *a, const Point2D *b);

// ---------------------------------------------------------------------------
// 4. Working with arrays (demonstrates pointer + length convention)
// ---------------------------------------------------------------------------
int32_t array_sum(const int32_t *arr, int32_t len);
void    array_double(int32_t *arr, int32_t len);  // mutates in place

// ---------------------------------------------------------------------------
// 5. Callbacks (demonstrates function-pointer FFI)
// ---------------------------------------------------------------------------
typedef int32_t (*TransformFn)(int32_t);
void    array_transform(int32_t *arr, int32_t len, TransformFn fn);

// ---------------------------------------------------------------------------
// 6. Global mutable state (demonstrates static-mut interop)
// ---------------------------------------------------------------------------
int32_t get_call_count(void);

#endif // MATHLIB_H
