#include "pow.h"
#include "mult.h"

uint32 poww(uint32 base, uint8 exponent)
{
    uint32 result = 1u;
    for(uint8 i = 0; i < exponent; ++i)
    {
        result = mult(result, base); // result *= base
    }

    return result;
}