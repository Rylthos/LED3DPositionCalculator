#ifndef UTIL_HELPER_HPP
#define UTIL_HELPER_HPP

inline float randomValue() { return (float)(rand() / (float)RAND_MAX); }

inline float mapValue(float initial, float iS, float iE, float oS, float oE)
{
    float slope = (oE - oS) / (iE - iS);
    return oS + slope * (initial - iS);
}

#endif
