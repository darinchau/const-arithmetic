# Implementation psuedocode for division
def long_division(h, k):
    if k == 0:
        raise ValueError("Divisor cannot be zero.")

    dividend_str = f"{h:X}"

    quotient = 0
    remainder = 0
    result = ""
    idx = 0

    while idx < len(dividend_str):
        current_digit = int(dividend_str[idx], base = 16)
        remainder = (remainder * 16) + current_digit

        if remainder < k:
            result += "0"
        else:
            quotient = remainder // k
            result += f"{quotient:X}"
            remainder = remainder % k

        idx += 1

    return int(result, base=16), remainder