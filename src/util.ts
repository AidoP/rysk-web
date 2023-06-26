export const to_fixed_hex = (value: number, len = 8): string => {
    return ('0'.repeat(len) + (value.toString(16))).slice(-len);
};