// =============================================================================
// mathlib.c — Implementation of our small C library
// =============================================================================

#include "mathlib.h"
#include <math.h>
#include <stdio.h>
#include <string.h>

// A file-scoped counter to demonstrate global mutable state across FFI
static int32_t call_count = 0;

// ---------------------------------------------------------------------------
// 1. Simple arithmetic
// ---------------------------------------------------------------------------

int32_t add(int32_t a, int32_t b) {
    call_count++;
    return a + b;
}

int32_t multiply(int32_t a, int32_t b) {
    call_count++;
    return a * b;
}

double divide_safe(double numerator, double denominator) {
    call_count++;
    if (denominator == 0.0) {
        return 0.0;  // safe fallback — real code would signal an error
    }
    return numerator / denominator;
}

// ---------------------------------------------------------------------------
// 2. Working with strings
// ---------------------------------------------------------------------------

int32_t string_length(const char *s) {
    call_count++;
    if (!s) return 0;
    return (int32_t)strlen(s);
}

void greet(const char *name, char *out_buf, int32_t buf_size) {
    call_count++;
    if (!name || !out_buf || buf_size <= 0) return;
    snprintf(out_buf, (size_t)buf_size, "Hello, %s! Welcome to C-from-Rust.", name);
}

// ---------------------------------------------------------------------------
// 3. Working with structs
// ---------------------------------------------------------------------------

double point_distance(const Point2D *a, const Point2D *b) {
    call_count++;
    double dx = b->x - a->x;
    double dy = b->y - a->y;
    return sqrt(dx * dx + dy * dy);
}

Point2D point_midpoint(const Point2D *a, const Point2D *b) {
    call_count++;
    Point2D mid;
    mid.x = (a->x + b->x) / 2.0;
    mid.y = (a->y + b->y) / 2.0;
    return mid;
}

// ---------------------------------------------------------------------------
// 4. Working with arrays
// ---------------------------------------------------------------------------

int32_t array_sum(const int32_t *arr, int32_t len) {
    call_count++;
    int32_t total = 0;
    for (int32_t i = 0; i < len; i++) {
        total += arr[i];
    }
    return total;
}

void array_double(int32_t *arr, int32_t len) {
    call_count++;
    for (int32_t i = 0; i < len; i++) {
        arr[i] *= 2;
    }
}

// ---------------------------------------------------------------------------
// 5. Callbacks
// ---------------------------------------------------------------------------

void array_transform(int32_t *arr, int32_t len, TransformFn fn) {
    call_count++;
    for (int32_t i = 0; i < len; i++) {
        arr[i] = fn(arr[i]);
    }
}

// ---------------------------------------------------------------------------
// 6. Global state
// ---------------------------------------------------------------------------

int32_t get_call_count(void) {
    return call_count;
}
