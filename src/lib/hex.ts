export const to_hex = (value: number, len?: number): string => {
    if (len !== undefined) {
        return value.toString(16).padStart(len, '0');
    } else {
        return value.toString(16);
    }
};