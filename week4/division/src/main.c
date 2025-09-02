#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>

#define MAXSIZE 1024

typedef struct {
    int digits[MAXSIZE];
    int len;
    bool is_negative;
} BigInt;


void bi_format(const BigInt* bi) {
        if (bi->is_negative) {
                printf("-");
        }

        int i;
        for (i = bi->len - 1; i >= 0; i--) {
                printf("%u", bi->digits[i]);
        }
}

void bi_print(const BigInt* bi) {
		bi_format(bi);
        printf("\n");
}

void bi_zero_justify(BigInt* bi) {
        while (bi->len > 1 && bi->digits[bi->len - 1] == 0) {
                bi->len--;
        }

        if (bi->len == 1 && bi->digits[0] == 0) {
                bi->is_negative = false;
        }
}

void bi_mul_10(BigInt* bi) {
        int i;
        for (i = bi->len; i >= 1; i--) {
                bi->digits[i] = bi->digits[i - 1];
        }
        bi->digits[0] = 0;

        bi->len++;
}

void bi_shift_left(BigInt* bi, int count) {
    int i;
    for (i = 0; i < count; i++) {
        bi_mul_10(bi);
    }
}

void bi_push_digit_front(BigInt* bi, int digit) {
        if (bi->len == MAXSIZE) {
                fprintf(stderr, "exceeded max length");
                exit(1);
        }

        int cur;

        bi_mul_10(bi);

        bi->digits[0] = digit;
}

void bi_push_digit_back(BigInt* bi, int digit) {
        if (bi->len == MAXSIZE) {
                fprintf(stderr, "exceeded max length");
                exit(1);
        }

        bi->digits[bi->len] = digit;
        bi->len++;
}

bool bi_is_zero(BigInt* bi) {
    return bi->len == 1 && bi->digits[0] == 0;
}

bool bi_is_one(BigInt* bi) {
    return bi->len == 1 && bi->digits[0] == 1;
}

BigInt* bi_init_empty(void) {
        BigInt* bi = calloc(1, sizeof(*bi));
        return bi;
}

BigInt* bi_init_zero(void) {
        BigInt *bi = bi_init_empty();
        bi_push_digit_front(bi, 0);
        return bi;
}

BigInt* bi_copy(const BigInt* src) {
    BigInt* dest = bi_init_empty();
    dest->is_negative = src->is_negative;
    int i;
    for (i = 0; i < src->len; i++) {
        bi_push_digit_back(dest, src->digits[i]);
    }
    return dest;
}

typedef enum {
        GREATER,
        EQUAL,
        LESS
} Comparison;

Comparison bi_compare(BigInt* lhs, BigInt* rhs) {
        if (lhs->is_negative && !rhs->is_negative) {
                return LESS;
        } else if (!lhs->is_negative && rhs->is_negative) {
                return GREATER;
        }

        if (lhs->len > rhs->len) {
                return lhs->is_negative ? LESS : GREATER;
        } else if (lhs->len < rhs->len) {
                return lhs->is_negative ? GREATER : LESS;
        }

        int i;
        for (i = lhs->len - 1; i >= 0; i--) {
                if (lhs->digits[i] > rhs->digits[i]) {
                        return lhs->is_negative ? LESS : GREATER;
                }
                if (lhs->digits[i] < rhs->digits[i]) {
                        return lhs->is_negative ? GREATER : LESS;
                }
        }

        return EQUAL;
}


bool is_numeric(const char c) {
        return (c >= 48) && (c <= 57);
}

char* bi_parse_from_str(char* str, BigInt* bi) {
        int pos = 0;
        int digit;

        if (str[pos] == '+') {
                pos++;
        } else if (str[pos] == '-') {
                bi->is_negative = true;
                pos++;
        }

        while (is_numeric(str[pos])) {
                digit = str[pos] - 48;
                bi_push_digit_front(bi, digit);
                pos++;
        }

        return &str[pos + 1];
}


BigInt* bi_sub(BigInt* lhs, BigInt* rhs);

BigInt* bi_add(BigInt* lhs, BigInt* rhs) {
        BigInt* result = bi_init_empty();

        if (lhs->is_negative == rhs->is_negative) {
                result->is_negative = lhs->is_negative;
        } else if (lhs->is_negative) {
                lhs->is_negative = !lhs->is_negative;
                result = bi_sub(rhs, lhs);
                lhs->is_negative = !lhs->is_negative;
                return result;
        } else if (rhs->is_negative) {
                rhs->is_negative = !rhs->is_negative;
                result = bi_sub(lhs, rhs);
                rhs->is_negative = !rhs->is_negative;
                return result;
        }

        int carry = 0;
        int sum = 0;

        int i;
        for (i = 0; i <= lhs->len || i <= rhs->len; i++) {
                sum += carry;
                if (i < lhs->len) {
                        sum += lhs->digits[i];
                }
                if (i < rhs->len) {
                        sum += rhs->digits[i];
                }

                bi_push_digit_back(result, sum % 10);

                carry = sum / 10;
                sum = 0;
        }

        bi_zero_justify(result);

        return result;
}

BigInt* bi_sub(BigInt* lhs, BigInt* rhs) {
    BigInt* result;

    if (lhs->is_negative || rhs->is_negative) {
        rhs->is_negative = !rhs->is_negative;
        result = bi_add(lhs, rhs);
        rhs->is_negative = !rhs->is_negative;
        return result;
    }

    if (bi_compare(lhs, rhs) == LESS) {
        result = bi_sub(rhs, lhs);
        result->is_negative = true;
        return result;
    }

    result = bi_init_empty();

    int borrow = 0;
    int digit;

    int i;
    for (i = 0; i < lhs->len; i++) {
        digit = lhs->digits[i] - borrow - rhs->digits[i];
        if (lhs->digits[i] > 0) borrow = 0;
        if (digit < 0) {
            digit += 10;
            borrow = 1;
        }

        bi_push_digit_back(result, digit);
    }

    bi_zero_justify(result);

    return result;
}

bool bi_sign_mul(BigInt* lhs, BigInt* rhs) {
    return lhs->is_negative != rhs->is_negative;
}

BigInt* bi_mul(BigInt* lhs, BigInt* rhs) {
    if (bi_is_zero(lhs) || bi_is_zero(rhs)) {
        return bi_init_zero();
    }

    if (bi_is_one(lhs)) {
        return rhs;
    }

    if (bi_is_one(rhs)) {
        return lhs;
    }

    BigInt* result = bi_init_zero();
    BigInt* row;
    BigInt tmp;

    row = bi_copy(lhs);

    int i;
    for (i = 0; i <= rhs->len; i++) {
        int j;
        for (j = 0; j < rhs->digits[i]; j++) {
            result = bi_add(result, row);
        }

        bi_mul_10(row);
    }

    result->is_negative = bi_sign_mul(lhs, rhs);
    bi_zero_justify(result);

    return result;
}

BigInt* bi_div(BigInt* lhs, BigInt* rhs) {
        BigInt* result = bi_init_zero();
        BigInt* row = bi_init_zero();

        result->is_negative = bi_sign_mul(lhs, rhs);
        bool lhs_is_neg = lhs->is_negative;
        bool rhs_is_neg = rhs->is_negative;

        int i;
        for (i = lhs->len - 1; i >= 0; i--) {
                bi_mul_10(row);
                bi_zero_justify(row);
                row->digits[0] = lhs->digits[i];
                bi_push_digit_back(result, 0);
                while (bi_compare(row, rhs) != LESS) {
                        result->digits[i]++;
                        row = bi_sub(row, rhs);
                }
        }

        lhs->is_negative = lhs_is_neg;
        rhs->is_negative = rhs_is_neg;

        bi_zero_justify(result);

        return result;
}

BigInt* bi_max_i32() {
    char num[100] = "2147483647";
    BigInt* result = bi_init_empty();
    bi_parse_from_str(num, result);
    return result;
}

BigInt* bi_init_one() {
        BigInt* bi = bi_init_zero();
        bi->digits[0] = 1;
        return bi;
}

BigInt* bi_pow(BigInt* lhs, BigInt* rhs) {
        BigInt* result = bi_init_zero();
        result->digits[0] = 1;

        BigInt* i = bi_init_zero();
        BigInt* one = bi_init_one();
        for (; bi_compare(i, rhs) == GREATER ; i = bi_add(i, one)) {
                result = bi_mul(lhs, lhs);
        }

        return result;
}

int main(void) {
    char buffer[100000];
    char* input;
    BigInt* max = bi_max_i32();
    BigInt* one = bi_init_one();

    while(fgets(buffer, sizeof(buffer), stdin) != NULL)  {
        printf("%s", buffer);
        input = buffer;

        BigInt* t = bi_init_empty();
        BigInt* a = bi_init_empty();
        BigInt* b = bi_init_empty();
        input = bi_parse_from_str(input, t);
        input = bi_parse_from_str(input, a);
        input = bi_parse_from_str(input, b);

        printf("(");
        bi_format(t);
        printf("^");
        bi_format(a);
        printf("-1)/(");
        bi_format(t);
        printf("^");
        bi_format(b);
        printf("-1) ");

        BigInt* pow = bi_pow(t, one);
        bi_print(pow);

        BigInt* result = bi_div(bi_sub(bi_pow(t, a), one), bi_sub(bi_pow(t, b), one));
        bi_print(result);

    }
}

